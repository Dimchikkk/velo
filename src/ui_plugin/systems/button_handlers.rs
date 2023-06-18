use std::collections::HashMap;
use std::{collections::VecDeque, time::Duration};

use bevy::render::view::RenderLayers;
use bevy::sprite::collide_aabb::collide;
use bevy::{prelude::*, window::PrimaryWindow};

use bevy_cosmic_edit::{CosmicEdit, CosmicFont};
use bevy_pkv::PkvStore;
use bevy_prototype_lyon::prelude::Fill;
use cosmic_text::{Cursor, Edit};
use serde::Serialize;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::themes::Theme;
use crate::{AddRect, JsonNode, JsonNodeText, NodeType, UiState};

use super::ui_helpers::{
    spawn_modal, ButtonAction, ChangeColor, DeleteDoc, DocListItemButton, GenericButton, NewDoc,
    ParticlesEffect, RawText, SaveDoc, TextPosMode, Tooltip, VeloBorder, VeloNode, VeloShadow,
};
use super::{ExportToFile, ImportFromFile, ImportFromUrl, MainPanel, ShareDoc};
use crate::canvas::arrow::components::{ArrowMeta, ArrowMode};
use crate::components::{Doc, EffectsCamera, Tab};
use crate::resources::{AppState, FontSystemState, LoadDocRequest, SaveDocRequest};
use crate::utils::{bevy_color_to_cosmic, get_timestamp, load_doc_to_memory, ReflectableUuid};

pub fn rec_button_handlers(
    mut commands: Commands,
    mut events: EventWriter<AddRect>,
    mut interaction_query: Query<
        (&Interaction, &ButtonAction),
        (Changed<Interaction>, With<ButtonAction>),
    >,
    mut raw_text_query: Query<(&mut CosmicEdit, &RawText, &Parent), With<RawText>>,
    border_query: Query<&Parent, With<VeloBorder>>,
    mut velo_node_query: Query<(Entity, &VeloNode, &mut Transform), With<VeloNode>>,
    mut arrows: Query<(Entity, &ArrowMeta, &mut Visibility), (With<ArrowMeta>, Without<Tooltip>)>,
    mut state: ResMut<UiState>,
    mut app_state: ResMut<AppState>,
    theme: Res<Theme>,
) {
    for (interaction, button_action) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => match button_action.button_type {
                super::ui_helpers::ButtonTypes::AddRec => {
                    events.send(AddRect {
                        node: JsonNode {
                            id: Uuid::new_v4(),
                            node_type: NodeType::Rect,
                            x: 0.,
                            y: 0.,
                            width: theme.node_width,
                            height: theme.node_height,
                            text: JsonNodeText {
                                text: "".to_string(),
                                pos: crate::TextPos::Center,
                            },
                            bg_color: theme.node_bg,
                            ..default()
                        },
                        image: None,
                    });
                }
                super::ui_helpers::ButtonTypes::AddCircle => {
                    events.send(AddRect {
                        node: JsonNode {
                            id: Uuid::new_v4(),
                            node_type: NodeType::Circle,
                            x: 0.,
                            y: 0.,
                            width: theme.node_width,
                            height: theme.node_height,
                            text: JsonNodeText {
                                text: "".to_string(),
                                pos: crate::TextPos::Center,
                            },
                            bg_color: theme.node_bg,
                            ..default()
                        },
                        image: None,
                    });
                }
                super::ui_helpers::ButtonTypes::AddPaper => {
                    events.send(AddRect {
                        node: JsonNode {
                            id: Uuid::new_v4(),
                            node_type: NodeType::Paper,
                            x: 0.,
                            y: 0.,
                            width: theme.node_width,
                            height: theme.node_height,
                            text: JsonNodeText {
                                text: "".to_string(),
                                pos: crate::TextPos::Center,
                            },
                            bg_color: theme.paper_node_bg,
                            ..default()
                        },
                        image: None,
                    });
                }
                super::ui_helpers::ButtonTypes::Del => {
                    if let Some(id) = state.entity_to_edit {
                        commands.insert_resource(bevy_cosmic_edit::ActiveEditor { entity: None });
                        *state = UiState::default();
                        for (entity, node, _) in velo_node_query.iter() {
                            if node.id == id {
                                commands.entity(entity).despawn_recursive();
                            }
                        }
                        #[allow(unused)]
                        for (entity, arrow, mut visibility) in &mut arrows.iter_mut() {
                            if arrow.start.id == id || arrow.end.id == id {
                                commands.entity(entity).despawn_recursive();
                            }
                        }
                    }
                }
                super::ui_helpers::ButtonTypes::Front => {
                    let current_document = app_state.current_document.unwrap();
                    let mut tab = app_state
                        .docs
                        .get_mut(&current_document)
                        .unwrap()
                        .tabs
                        .iter_mut()
                        .find(|x| x.is_active)
                        .unwrap();
                    if let Some(id) = state.entity_to_edit {
                        let mut data = None;
                        // fint current z_index
                        for (cosmic_edit, raw_text, parent) in &mut raw_text_query.iter_mut() {
                            if raw_text.id == id {
                                let border = border_query.get(parent.get()).unwrap();
                                let top = velo_node_query.get_mut(border.get()).unwrap();
                                let size = Vec2::new(cosmic_edit.width, cosmic_edit.height);
                                let translation = top.2.translation;
                                data = Some((size, translation));
                                break;
                            }
                        }
                        // find higher z_index if collide
                        for (cosmic_edit, raw_text, parent) in &mut raw_text_query.iter_mut() {
                            if raw_text.id != id {
                                let border = border_query.get(parent.get()).unwrap();
                                let top = velo_node_query.get_mut(border.get()).unwrap();
                                let size = Vec2::new(cosmic_edit.width, cosmic_edit.height);
                                let translation = top.2.translation;
                                if let Some((active_size, active_translation)) = data {
                                    if collide(translation, size, active_translation, active_size)
                                        .is_some()
                                        && translation.z > active_translation.z
                                    {
                                        data = Some((size, translation));
                                        break;
                                    }
                                }
                            }
                        }
                        // update z_index
                        for (_, node, mut transform) in velo_node_query.iter_mut() {
                            if node.id == id {
                                if let Some((_, translation)) = data {
                                    transform.translation.z = (translation.z + 3.) % f32::MAX;
                                } else {
                                    transform.translation.z =
                                        (transform.translation.z + 3.) % f32::MAX;
                                }
                                if tab.z_index < transform.translation.z {
                                    tab.z_index = transform.translation.z;
                                }
                                break;
                            }
                        }
                    }
                }
                super::ui_helpers::ButtonTypes::Back => {
                    if let Some(id) = state.entity_to_edit {
                        let mut data = None;
                        // fint current z_index
                        for (cosmic_edit, raw_text, parent) in &mut raw_text_query.iter_mut() {
                            if raw_text.id == id {
                                let border = border_query.get(parent.get()).unwrap();
                                let top = velo_node_query.get_mut(border.get()).unwrap();
                                let size = Vec2::new(cosmic_edit.width, cosmic_edit.height);
                                let translation = top.2.translation;
                                data = Some((size, translation));
                                break;
                            }
                        }
                        // find lower z_index if collide
                        for (cosmic_edit, raw_text, parent) in &mut raw_text_query.iter_mut() {
                            if raw_text.id != id {
                                let border = border_query.get(parent.get()).unwrap();
                                let top = velo_node_query.get_mut(border.get()).unwrap();
                                let size = Vec2::new(cosmic_edit.width, cosmic_edit.height);
                                let translation = top.2.translation;
                                if let Some((active_size, active_translation)) = data {
                                    if collide(translation, size, active_translation, active_size)
                                        .is_some()
                                        && translation.z < active_translation.z
                                    {
                                        data = Some((size, translation));
                                        break;
                                    }
                                }
                            }
                        }
                        // update z_index
                        for (_, node, mut transform) in velo_node_query.iter_mut() {
                            if node.id == id {
                                if let Some((_, translation)) = data {
                                    transform.translation.z = f32::max(translation.z - 3., 1.);
                                } else {
                                    transform.translation.z =
                                        f32::max(transform.translation.z - 3., 1.);
                                }
                                break;
                            }
                        }
                    }
                }
            },
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn change_color_pallete(
    mut interaction_query: Query<
        (&Interaction, &ChangeColor),
        (Changed<Interaction>, With<ChangeColor>),
    >,
    mut velo_border: Query<(&mut Fill, &VeloBorder), With<VeloBorder>>,
    ui_state: Res<UiState>,
) {
    for (interaction, change_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                let color = change_color.color;
                for (mut stroke, velo_border) in velo_border.iter_mut() {
                    if Some(velo_border.id) == ui_state.entity_to_edit {
                        stroke.color = color;
                        break;
                    }
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn change_text_pos(
    mut interaction_query: Query<
        (&Interaction, &TextPosMode),
        (Changed<Interaction>, With<TextPosMode>),
    >,
    state: Res<UiState>,
    mut raw_text_node_query: Query<(&RawText, &mut CosmicEdit), With<RawText>>,
) {
    for (interaction, text_pos_mode) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if let Some(entity_to_edit) = state.entity_to_edit {
                    for (raw_text, mut cosmit_edit) in raw_text_node_query.iter_mut() {
                        if raw_text.id == entity_to_edit {
                            cosmit_edit.text_pos = text_pos_mode.text_pos.clone().into();
                            cosmit_edit.editor.buffer_mut().set_redraw(true);
                        }
                    }
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn change_arrow_type(
    mut interaction_query: Query<
        (&Interaction, &ArrowMode),
        (Changed<Interaction>, With<ArrowMode>),
    >,
    mut state: ResMut<UiState>,
) {
    for (interaction, arrow_mode) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state.arrow_type = arrow_mode.arrow_type;
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn new_doc_handler(
    mut commands: Commands,
    mut new_doc_query: Query<&Interaction, (Changed<Interaction>, With<NewDoc>)>,
    mut app_state: ResMut<AppState>,
) {
    for interaction in &mut new_doc_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let doc_id = ReflectableUuid::generate();
                let name = "Untitled".to_string();
                let tab_id = ReflectableUuid::generate();
                let mut checkpoints = VecDeque::new();
                checkpoints.push_back(
                    json!({
                        "nodes": [],
                        "arrows": [],
                        "images": {},
                    })
                    .to_string(),
                );
                let tabs = vec![Tab {
                    id: tab_id,
                    name: "Tab 1".to_string(),
                    checkpoints,
                    is_active: true,
                    z_index: 10.,
                }];
                app_state.docs.insert(
                    doc_id,
                    Doc {
                        id: doc_id,
                        name: name.clone(),
                        tabs,
                        tags: vec![],
                    },
                );
                commands.insert_resource(SaveDocRequest {
                    doc_id: app_state.current_document.unwrap(),
                    path: None,
                });
                app_state.current_document = Some(doc_id);
                commands.insert_resource(LoadDocRequest { doc_id });
                app_state.doc_list_ui.insert(doc_id);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn rename_doc_handler(
    mut commands: Commands,
    mut rename_doc_query: Query<
        (&Interaction, &DocListItemButton, Entity, &mut CosmicEdit),
        (Changed<Interaction>, With<DocListItemButton>),
    >,
    mut ui_state: ResMut<UiState>,
    mut double_click: Local<(Duration, Option<ReflectableUuid>)>,
    theme: Res<Theme>,
) {
    for (interaction, item, entity, mut cosmic_edit) in &mut rename_doc_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let now_ms = get_timestamp();
                if double_click.1 == Some(item.id)
                    && Duration::from_millis(now_ms as u64) - double_click.0
                        < Duration::from_millis(500)
                {
                    *ui_state = UiState::default();
                    commands.insert_resource(bevy_cosmic_edit::ActiveEditor {
                        entity: Some(entity),
                    });
                    cosmic_edit.readonly = false;
                    cosmic_edit.editor.set_cursor(Cursor::new_with_color(
                        0,
                        0,
                        bevy_color_to_cosmic(theme.font),
                    ));
                    ui_state.doc_to_edit = Some(item.id);
                    *double_click = (Duration::from_secs(0), None);
                } else {
                    *double_click = (Duration::from_millis(now_ms as u64), Some(item.id));
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn delete_doc_handler(
    mut commands: Commands,
    mut delete_doc_query: Query<&Interaction, (Changed<Interaction>, With<DeleteDoc>)>,
    mut ui_state: ResMut<UiState>,
    mut app_state: ResMut<AppState>,
    main_panel_query: Query<Entity, With<MainPanel>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    pkv: Res<PkvStore>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    font_system_state: ResMut<FontSystemState>,
    theme: Res<Theme>,
) {
    let window = windows.single();
    for interaction in &mut delete_doc_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if app_state.docs.len() == 1 {
                    if let Ok(docs) = pkv.get::<HashMap<ReflectableUuid, Doc>>("docs") {
                        if docs.len() > 1 {
                            for (id, doc) in docs.iter() {
                                if app_state.docs.len() != 1 {
                                    break;
                                }
                                app_state.docs.insert(*id, doc.clone());
                            }
                        } else {
                            // do not allow deletion if there is less than two docs
                            return;
                        }
                    } else {
                        // do not allow deletion if there is less than two docs
                        return;
                    }
                }
                let id = ReflectableUuid::generate();
                *ui_state = UiState::default();
                commands.insert_resource(bevy_cosmic_edit::ActiveEditor { entity: None });
                ui_state.modal_id = Some(id);
                let entity = spawn_modal(
                    &mut commands,
                    &theme,
                    &mut cosmic_fonts,
                    font_system_state.0.clone().unwrap(),
                    window,
                    id,
                    super::ModalAction::DeleteDocument,
                );
                commands.entity(main_panel_query.single()).add_child(entity);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn save_doc_handler(
    mut commands: Commands,
    mut save_doc_query: Query<&Interaction, (Changed<Interaction>, With<SaveDoc>)>,
    state: Res<AppState>,
) {
    for interaction in &mut save_doc_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                commands.insert_resource(SaveDocRequest {
                    doc_id: state.current_document.unwrap(),
                    path: None,
                });
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn export_to_file(
    mut commands: Commands,
    mut query: Query<&Interaction, (Changed<Interaction>, With<ExportToFile>)>,
    mut ui_state: ResMut<UiState>,
    main_panel_query: Query<Entity, With<MainPanel>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    font_system_state: ResMut<FontSystemState>,
    theme: Res<Theme>,
) {
    let window = windows.single();
    for interaction in &mut query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let id = ReflectableUuid::generate();
                *ui_state = UiState::default();
                commands.insert_resource(bevy_cosmic_edit::ActiveEditor { entity: None });
                ui_state.modal_id = Some(id);
                let entity = spawn_modal(
                    &mut commands,
                    &theme,
                    &mut cosmic_fonts,
                    font_system_state.0.clone().unwrap(),
                    window,
                    id,
                    super::ModalAction::SaveToFile,
                );
                commands.entity(main_panel_query.single()).add_child(entity);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn set_window_property(mut app_state: ResMut<AppState>, mut pkv: ResMut<PkvStore>) {
    if let Some(doc_id) = app_state.current_document {
        load_doc_to_memory(doc_id, &mut app_state, &mut pkv);
        let current_doc = app_state.docs.get(&doc_id).unwrap().clone();
        let value = serde_json::to_string_pretty(&current_doc).unwrap();
        let window = wasm_bindgen::JsValue::from(web_sys::window().unwrap());
        let velo_var = wasm_bindgen::JsValue::from("velo");
        let state = wasm_bindgen::JsValue::from(value);
        js_sys::Reflect::set(&window, &velo_var, &state).unwrap();
    }
}

#[derive(Serialize)]
struct GistFile {
    content: String,
}

#[derive(Serialize)]
struct GistCreateRequest {
    description: String,
    public: bool,
    files: std::collections::HashMap<String, GistFile>,
}

pub fn shared_doc_handler(
    mut app_state: ResMut<AppState>,
    mut query: Query<&Interaction, (Changed<Interaction>, With<ShareDoc>)>,
    mut pkv: ResMut<PkvStore>,
) {
    for interaction in &mut query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if let Some(doc_id) = app_state.current_document {
                    load_doc_to_memory(doc_id, &mut app_state, &mut pkv);
                    let current_doc = app_state.docs.get(&doc_id).unwrap().clone();
                    let contents = serde_json::to_string_pretty(&current_doc).unwrap();
                    let mut files = std::collections::HashMap::new();
                    let filename = "velo.json";
                    let file = GistFile {
                        content: contents.to_string(),
                    };
                    files.insert(filename.to_string(), file);

                    let request = GistCreateRequest {
                        description: "Velo Document".to_string(),
                        public: true,
                        files,
                    };

                    let mut request = ehttp::Request::post(
                        "https://api.github.com/gists",
                        serde_json::to_string_pretty(&request).unwrap(),
                    );
                    request.headers.insert(
                        "Accept".to_string(),
                        "application/vnd.github.v3+json".to_string(),
                    );
                    request.headers.insert(
                        "Authorization".to_string(),
                        format!("token {}", app_state.github_token.as_ref().unwrap()),
                    );
                    #[cfg(not(target_arch = "wasm32"))]
                    let mut clipboard = arboard::Clipboard::new().unwrap();
                    ehttp::fetch(request, move |result| {
                        let response = result.unwrap();
                        if response.ok {
                            let res_json: Value =
                                serde_json::from_str(response.text().unwrap().as_str()).unwrap();
                            let files: Value = res_json["files"].clone();
                            let velo = files["velo.json"].clone();
                            #[cfg(not(target_arch = "wasm32"))]
                            clipboard
                                .set_text(format!(
                                    "https://staffengineer.github.io/velo?document={}",
                                    velo["raw_url"].to_string().replace('\"', "")
                                ))
                                .unwrap();
                        } else {
                            error!("Error sharing document: {}", response.status_text);
                        }
                    });
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn import_from_file(
    mut commands: Commands,
    mut query: Query<&Interaction, (Changed<Interaction>, With<ImportFromFile>)>,
    mut ui_state: ResMut<UiState>,
    main_panel_query: Query<Entity, With<MainPanel>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    font_system_state: ResMut<FontSystemState>,
    theme: Res<Theme>,
) {
    let window = windows.single();
    for interaction in &mut query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let id = ReflectableUuid::generate();
                *ui_state = UiState::default();
                commands.insert_resource(bevy_cosmic_edit::ActiveEditor { entity: None });
                ui_state.modal_id = Some(id);
                let entity = spawn_modal(
                    &mut commands,
                    &theme,
                    &mut cosmic_fonts,
                    font_system_state.0.clone().unwrap(),
                    window,
                    id,
                    super::ModalAction::LoadFromFile,
                );
                commands.entity(main_panel_query.single()).add_child(entity);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn import_from_url(
    mut commands: Commands,
    mut query: Query<&Interaction, (Changed<Interaction>, With<ImportFromUrl>)>,
    mut ui_state: ResMut<UiState>,
    main_panel_query: Query<Entity, With<MainPanel>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    font_system_state: ResMut<FontSystemState>,
    theme: Res<Theme>,
) {
    let window = windows.single();
    for interaction in &mut query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let id = ReflectableUuid::generate();
                *ui_state = UiState::default();
                commands.insert_resource(bevy_cosmic_edit::ActiveEditor { entity: None });
                ui_state.modal_id = Some(id);
                let entity = spawn_modal(
                    &mut commands,
                    &theme,
                    &mut cosmic_fonts,
                    font_system_state.0.clone().unwrap(),
                    window,
                    id,
                    super::ModalAction::LoadFromUrl,
                );
                commands.entity(main_panel_query.single()).add_child(entity);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn button_generic_handler(
    _commands: Commands,
    mut generic_button_query: Query<
        (&Interaction, &mut BackgroundColor, Entity),
        (Changed<Interaction>, With<GenericButton>),
    >,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut tooltips_query: Query<(&mut Style, &Parent), With<Tooltip>>,
) {
    let mut primary_window = windows.single_mut();
    for (interaction, mut bg_color, entity) in &mut generic_button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {}
            Interaction::Hovered => {
                primary_window.cursor.icon = CursorIcon::Hand;
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.8);
                for (mut style, parent) in tooltips_query.iter_mut() {
                    if parent.get() == entity {
                        style.display = Display::Flex;
                    }
                }
            }
            Interaction::None => {
                primary_window.cursor.icon = CursorIcon::Default;
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 1.);
                for (mut style, parent) in tooltips_query.iter_mut() {
                    if parent.get() == entity {
                        style.display = Display::None;
                    }
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn particles_effect(
    mut query: Query<&Interaction, (Changed<Interaction>, With<ParticlesEffect>)>,
    mut commands: Commands,
    mut effects: ResMut<Assets<bevy_hanabi::EffectAsset>>,
    mut effects_camera: Query<&mut Camera, With<EffectsCamera>>,
    mut effects_query: Query<(&Name, Entity)>,
    mut shadow_query: Query<&mut Transform, With<VeloShadow>>,
) {
    use bevy_hanabi::prelude::*;
    use rand::Rng;

    for interaction in &mut query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if effects_camera.single_mut().is_active {
                    effects_camera.single_mut().is_active = false;
                    for mut transform in shadow_query.iter_mut() {
                        // this is a hack to make bevy_hanabi work
                        transform.translation.z = 0.09;
                    }
                    for (name, entity) in effects_query.iter_mut() {
                        if name.as_str() == "effect:2d" {
                            commands.entity(entity).despawn_recursive();
                        }
                    }
                } else {
                    effects_camera.single_mut().is_active = true;
                    for mut transform in shadow_query.iter_mut() {
                        // this is a hack to make bevy_hanabi work
                        transform.translation.z = -1000.;
                    }
                    let mut gradient = Gradient::new();
                    let mut rng = rand::thread_rng();
                    gradient.add_key(
                        0.0,
                        Vec4::new(
                            rng.gen_range(0.0..1.0),
                            rng.gen_range(0.0..1.0),
                            rng.gen_range(0.0..1.0),
                            1.0,
                        ),
                    );
                    gradient.add_key(
                        1.0,
                        Vec4::new(
                            rng.gen_range(0.0..1.0),
                            rng.gen_range(0.0..1.0),
                            rng.gen_range(0.0..1.0),
                            0.0,
                        ),
                    );

                    let mut size_gradient1 = Gradient::new();
                    size_gradient1.add_key(0.0, Vec2::splat(0.008));
                    size_gradient1.add_key(0.3, Vec2::splat(0.012));
                    size_gradient1.add_key(1.0, Vec2::splat(0.0));

                    let spawner = Spawner::rate(rng.gen_range(10.0..300.0).into());
                    let effect = effects.add(
                        EffectAsset {
                            name: "Effect".into(),
                            capacity: 32768,
                            spawner,
                            ..Default::default()
                        }
                        .init(InitPositionCircleModifier {
                            center: Vec3::ZERO,
                            axis: Vec3::Z,
                            radius: 0.0001,
                            dimension: ShapeDimension::Surface,
                        })
                        .init(InitVelocityCircleModifier {
                            center: Vec3::ZERO,
                            axis: Vec3::Z,
                            speed: Value::Uniform((0.05, 0.1)),
                        })
                        .init(InitLifetimeModifier {
                            lifetime: Value::Uniform((5., 10.)),
                        })
                        .render(SizeOverLifetimeModifier {
                            gradient: size_gradient1,
                        })
                        .render(ColorOverLifetimeModifier { gradient }),
                    );

                    commands
                        .spawn(ParticleEffectBundle {
                            effect: ParticleEffect::new(effect),
                            ..default()
                        })
                        .insert(Name::new("effect:2d"))
                        .insert(RenderLayers::layer(2));
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
