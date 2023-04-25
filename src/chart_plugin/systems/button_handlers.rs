use std::{collections::VecDeque, time::Duration};

use bevy::{prelude::*, window::PrimaryWindow};

use serde_json::json;
use uuid::Uuid;

use crate::{get_timestamp, AddRectEvent, JsonNode, JsonNodeText, NodeType, UiState};

use super::ui_helpers::{
    add_list_item, get_sections, pos_to_style, spawn_modal, ButtonAction, ChangeColor, DeleteDoc,
    DocList, DocListItemButton, EditableText, GenericButton, NewDoc, SaveDoc, TextManipulation,
    TextManipulationAction, TextPosMode, Tooltip, VeloNode,
};
use super::{ExportToFile, ImportFromFile, ImportFromUrl, MainPanel, VeloNodeContainer};
use crate::canvas::arrow::components::{ArrowMeta, ArrowMode};
use crate::components::{Doc, Tab};
use crate::resources::{AppState, LoadRequest, SaveRequest};
use crate::utils::ReflectableUuid;

pub fn rec_button_handlers(
    mut commands: Commands,
    mut events: EventWriter<AddRectEvent>,
    mut interaction_query: Query<
        (&Interaction, &ButtonAction),
        (Changed<Interaction>, With<ButtonAction>),
    >,
    mut nodes: Query<(Entity, &VeloNodeContainer, &mut ZIndex), With<VeloNodeContainer>>,
    mut arrows: Query<(Entity, &ArrowMeta, &mut Visibility), (With<ArrowMeta>, Without<Tooltip>)>,
    mut state: ResMut<UiState>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    for (interaction, button_action) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => match button_action.button_type {
                super::ui_helpers::ButtonTypes::Add => {
                    events.send(AddRectEvent {
                        node: JsonNode {
                            id: Uuid::new_v4(),
                            node_type: NodeType::Rect,
                            left: Val::Px(window.width() / 2. - 200.),
                            bottom: Val::Px(window.height() / 2.),
                            width: Val::Px(100.0),
                            height: Val::Px(100.0),
                            text: JsonNodeText {
                                text: "".to_string(),
                                pos: crate::TextPos::Center,
                            },
                            bg_color: Color::rgb(1.0, 1.0, 1.0),
                            tags: vec![],
                            z_index: 0,
                        },
                        image: None,
                    });
                }
                super::ui_helpers::ButtonTypes::Del => {
                    if let Some(id) = state.entity_to_edit {
                        *state = UiState::default();
                        for (entity, node, _) in nodes.iter() {
                            if node.id == id {
                                commands.entity(entity).despawn_recursive();
                            }
                        }
                        #[allow(unused)]
                        for (entity, arrow, mut visibility) in &mut arrows.iter_mut() {
                            if arrow.start.id == id || arrow.end.id == id {
                                #[cfg(not(target_arch = "wasm32"))]
                                {
                                    commands.entity(entity).despawn_recursive();
                                }
                                #[cfg(target_arch = "wasm32")]
                                {
                                    *visibility = Visibility::Hidden;
                                }
                            }
                        }
                    }
                }
                super::ui_helpers::ButtonTypes::Front => {
                    if let Some(id) = state.entity_to_edit {
                        for (_, node, mut z_index) in nodes.iter_mut() {
                            if node.id == id {
                                if let ZIndex::Local(i) = *z_index {
                                    *z_index = ZIndex::Local(i + 1);
                                }
                            }
                        }
                    }
                }
                super::ui_helpers::ButtonTypes::Back => {
                    if let Some(id) = state.entity_to_edit {
                        for (_, node, mut z_index) in nodes.iter_mut() {
                            if node.id == id {
                                if let ZIndex::Local(i) = *z_index {
                                    *z_index = ZIndex::Local(i - 1);
                                }
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
        (Changed<Interaction>, With<ChangeColor>, Without<VeloNode>),
    >,
    mut nodes: Query<(&mut BackgroundColor, &VeloNode), With<VeloNode>>,
    state: Res<UiState>,
) {
    for (interaction, change_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                let color = change_color.color;
                if state.entity_to_edit.is_some() {
                    for (mut bg_color, node) in nodes.iter_mut() {
                        if node.id == state.entity_to_edit.unwrap() {
                            bg_color.0 = color;
                        }
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
    mut nodes: Query<(&mut Style, &VeloNode), With<VeloNode>>,
    state: Res<UiState>,
) {
    for (interaction, text_pos_mode) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if state.entity_to_edit.is_some() {
                    for (mut style, node) in nodes.iter_mut() {
                        if node.id == state.entity_to_edit.unwrap() {
                            let (justify_content, align_items) =
                                pos_to_style(text_pos_mode.text_pos.clone());
                            style.justify_content = justify_content;
                            style.align_items = align_items;
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

pub fn text_manipulation(
    mut interaction_query: Query<
        (&Interaction, &TextManipulationAction),
        (Changed<Interaction>, With<TextManipulationAction>),
    >,
    mut editable_text: Query<(&mut Text, &EditableText), With<EditableText>>,
    ui_state: Res<UiState>,
) {
    for (interaction, text_manipulation) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                #[cfg(not(target_arch = "wasm32"))]
                let mut clipboard = arboard::Clipboard::new().unwrap();

                match text_manipulation.action_type {
                    TextManipulation::Cut => {
                        if let Some(id) = vec![
                            ui_state.entity_to_edit,
                            ui_state.tab_to_edit,
                            ui_state.doc_to_edit,
                            ui_state.modal_id,
                        ]
                        .into_iter()
                        .find_map(|x| x)
                        {
                            for (mut text, node) in editable_text.iter_mut() {
                                if node.id == id {
                                    let mut str = "".to_string();
                                    let mut text_sections = text.sections.clone();
                                    text_sections.pop();
                                    for section in text_sections.iter() {
                                        str = format!("{}{}", str, section.value.clone());
                                    }
                                    text.sections = vec![
                                        TextSection {
                                            value: "".to_string(),
                                            style: TextStyle {
                                                font_size: 20.0,
                                                color: Color::BLACK,
                                                ..default()
                                            },
                                        },
                                        TextSection {
                                            value: " ".to_string(),
                                            style: TextStyle {
                                                font_size: 20.0,
                                                color: Color::BLACK,
                                                ..default()
                                            },
                                        },
                                    ];
                                }
                            }
                        }
                    }
                    TextManipulation::Paste =>
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        if let Ok(clipboard_text) = clipboard.get_text() {
                            for (mut text, editable_text) in editable_text.iter_mut() {
                                if vec![
                                    ui_state.entity_to_edit,
                                    ui_state.tab_to_edit,
                                    ui_state.doc_to_edit,
                                    ui_state.modal_id,
                                ]
                                .contains(&Some(editable_text.id))
                                {
                                    let mut str = "".to_string();
                                    let mut text_sections = text.sections.clone();
                                    text_sections.pop();
                                    for section in text_sections.iter() {
                                        str = format!("{}{}", str, section.value.clone());
                                    }
                                    str = format!("{}{}", str, clipboard_text);
                                    text.sections = get_sections(str).0;
                                }
                            }
                        }
                    }
                    TextManipulation::Copy => {
                        if let Some(id) = vec![
                            ui_state.entity_to_edit,
                            ui_state.tab_to_edit,
                            ui_state.doc_to_edit,
                            ui_state.modal_id,
                        ]
                        .into_iter()
                        .find_map(|x| x)
                        {
                            for (text, node) in editable_text.iter_mut() {
                                if node.id == id {
                                    let mut str = "".to_string();
                                    let mut text_sections = text.sections.clone();
                                    text_sections.pop();
                                    for section in text_sections.iter() {
                                        str = format!("{}{}", str, section.value.clone());
                                    }
                                    #[cfg(not(target_arch = "wasm32"))]
                                    clipboard.set_text(str).unwrap()
                                }
                            }
                        }
                    }
                    TextManipulation::OpenAllLinks => {
                        if let Some(id) = vec![
                            ui_state.entity_to_edit,
                            ui_state.tab_to_edit,
                            ui_state.doc_to_edit,
                            ui_state.modal_id,
                        ]
                        .into_iter()
                        .find_map(|x| x)
                        {
                            for (text, node) in editable_text.iter_mut() {
                                if node.id == id {
                                    let mut str = "".to_string();
                                    let mut text_sections = text.sections.clone();
                                    text_sections.pop();
                                    for section in text_sections.iter() {
                                        str = format!("{}{}", str, section.value.clone());
                                    }
                                    let (sections, is_link) = get_sections(str);
                                    for (i, section) in sections.iter().enumerate() {
                                        if is_link[i] {
                                            #[cfg(not(target_arch = "wasm32"))]
                                            open::that(section.value.clone()).unwrap();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn new_doc_handler(
    mut commands: Commands,
    mut new_doc_query: Query<&Interaction, (Changed<Interaction>, With<NewDoc>)>,
    mut doc_list_query: Query<Entity, With<DocList>>,
    mut app_state: ResMut<AppState>,
    asset_server: Res<AssetServer>,
) {
    for interaction in &mut new_doc_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let doc_id = ReflectableUuid(Uuid::new_v4());
                let name = "Untitled".to_string();
                let tab_id = ReflectableUuid(Uuid::new_v4());
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
                commands.insert_resource(SaveRequest {
                    doc_id: Some(app_state.current_document.unwrap()),
                    tab_id: None,
                    path: None,
                });
                app_state.current_document = Some(doc_id);
                commands.insert_resource(LoadRequest {
                    doc_id: Some(doc_id),
                    drop_last_checkpoint: false,
                });
                let button = add_list_item(&mut commands, &asset_server, doc_id, name, true);
                let doc_list = doc_list_query.single_mut();
                commands.entity(doc_list).add_child(button);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn rename_doc_handler(
    mut rename_doc_query: Query<
        (&Interaction, &DocListItemButton),
        (Changed<Interaction>, With<DocListItemButton>),
    >,
    mut ui_state: ResMut<UiState>,
    mut double_click: Local<(Duration, Option<ReflectableUuid>)>,
) {
    for (interaction, item) in &mut rename_doc_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let now_ms = get_timestamp();
                if double_click.1 == Some(item.id)
                    && Duration::from_millis(now_ms as u64) - double_click.0
                        < Duration::from_millis(500)
                {
                    *ui_state = UiState::default();
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
    app_state: Res<AppState>,
    main_panel_query: Query<Entity, With<MainPanel>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    for interaction in &mut delete_doc_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if app_state.docs.len() < 2 {
                    return;
                }
                let id = ReflectableUuid(Uuid::new_v4());
                *ui_state = UiState::default();
                ui_state.modal_id = Some(id);
                let entity = spawn_modal(
                    &mut commands,
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
                commands.insert_resource(SaveRequest {
                    doc_id: Some(state.current_document.unwrap()),
                    tab_id: None,
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
) {
    let window = windows.single();
    for interaction in &mut query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let id = ReflectableUuid(Uuid::new_v4());
                *ui_state = UiState::default();
                ui_state.modal_id = Some(id);
                let entity = spawn_modal(&mut commands, window, id, super::ModalAction::SaveToFile);
                commands.entity(main_panel_query.single()).add_child(entity);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn set_window_property(mut app_state: ResMut<AppState>) {
    if let Some(doc_id) = app_state.current_document {
        let current_doc = app_state.docs.get(&doc_id).unwrap().clone();
        let value = serde_json::to_string_pretty(&current_doc).unwrap();
        let window = wasm_bindgen::JsValue::from(web_sys::window().unwrap());
        let velo_var = wasm_bindgen::JsValue::from("velo");
        let state = wasm_bindgen::JsValue::from(value);
        js_sys::Reflect::set(&window, &velo_var, &state).unwrap();
    }
}

pub fn import_from_file(
    mut commands: Commands,
    mut query: Query<&Interaction, (Changed<Interaction>, With<ImportFromFile>)>,
    mut ui_state: ResMut<UiState>,
    main_panel_query: Query<Entity, With<MainPanel>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    for interaction in &mut query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let id = ReflectableUuid(Uuid::new_v4());
                *ui_state = UiState::default();
                ui_state.modal_id = Some(id);
                let entity =
                    spawn_modal(&mut commands, window, id, super::ModalAction::LoadFromFile);
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
) {
    let window = windows.single();
    for interaction in &mut query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let id = ReflectableUuid(Uuid::new_v4());
                *ui_state = UiState::default();
                ui_state.modal_id = Some(id);
                let entity =
                    spawn_modal(&mut commands, window, id, super::ModalAction::LoadFromUrl);
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
    mut tooltips_query: Query<(&mut Visibility, &Parent), With<Tooltip>>,
) {
    let mut primary_window = windows.single_mut();
    for (interaction, mut bg_color, entity) in &mut generic_button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {}
            Interaction::Hovered => {
                primary_window.cursor.icon = CursorIcon::Hand;
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.8);
                for (mut visibility, parent) in tooltips_query.iter_mut() {
                    if parent.get() == entity {
                        *visibility = Visibility::Visible;
                    }
                }
            }
            Interaction::None => {
                primary_window.cursor.icon = CursorIcon::Default;
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 1.);
                for (mut visibility, parent) in tooltips_query.iter_mut() {
                    if parent.get() == entity {
                        *visibility = Visibility::Hidden;
                    }
                }
            }
        }
    }
}
