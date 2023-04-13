use std::collections::VecDeque;

use bevy::{prelude::*, window::PrimaryWindow};

use serde_json::json;
use uuid::Uuid;

use crate::{
    AddRect, AppState, Doc, JsonNode, JsonNodeText, LoadRequest, NodeType, SaveRequest, Tab,
    UpdateListHighlight,
};

use super::ui_helpers::{
    add_list_item, get_sections, pos_to_style, spawn_modal, ArrowMeta, ArrowMode, ButtonAction,
    ChangeColor, DeleteDoc, DocList, DocListItemText, EditableText, ModalEntity, NewDoc, Rectangle,
    ReflectableUuid, RenameDoc, SaveDoc, TextManipulation, TextManipulationAction, TextPosMode,
    Tooltip,
};

pub fn rec_button_handlers(
    mut commands: Commands,
    mut events: EventWriter<AddRect>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonAction, &Children),
        (Changed<Interaction>, With<ButtonAction>),
    >,
    mut tooltips_query: Query<&mut Visibility, With<Tooltip>>,
    mut nodes: Query<(Entity, &Rectangle, &mut ZIndex), With<Rectangle>>,
    arrows: Query<(Entity, &ArrowMeta), With<ArrowMeta>>,
    mut state: ResMut<AppState>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    for (interaction, mut color, button_action, children) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => match button_action.button_type {
                super::ui_helpers::ButtonTypes::Add => {
                    events.send(AddRect {
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
                            bg_color: Color::WHITE,
                            tags: vec![],
                            z_index: 0,
                        },
                        image: None,
                    });
                }
                super::ui_helpers::ButtonTypes::Del => {
                    if let Some(id) = state.entity_to_edit {
                        state.entity_to_edit = None;
                        state.entity_to_resize = None;
                        state.hold_entity = None;
                        state.arrow_to_draw_start = None;
                        for (entity, node, _) in nodes.iter() {
                            if node.id == id {
                                commands.entity(entity).despawn_recursive();
                            }
                        }
                        for (entity, arrow) in arrows.iter() {
                            if arrow.start.id == id || arrow.end.id == id {
                                commands.entity(entity).despawn_recursive();
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
            Interaction::Hovered => {
                color.0 = Color::GRAY;
                match button_action.button_type {
                    super::ui_helpers::ButtonTypes::Add => {}
                    super::ui_helpers::ButtonTypes::Del => {}
                    super::ui_helpers::ButtonTypes::Front => {
                        let child = children.iter().next().unwrap();
                        let mut visibility = tooltips_query.get_mut(*child).unwrap();
                        *visibility = Visibility::Visible;
                    }
                    super::ui_helpers::ButtonTypes::Back => {
                        let child = children.iter().next().unwrap();
                        let mut visibility = tooltips_query.get_mut(*child).unwrap();
                        *visibility = Visibility::Visible;
                    }
                }
            }
            Interaction::None => {
                color.0 = Color::rgb(0.8, 0.8, 0.8);
                match button_action.button_type {
                    super::ui_helpers::ButtonTypes::Add => {}
                    super::ui_helpers::ButtonTypes::Del => {}
                    super::ui_helpers::ButtonTypes::Front => {
                        let child = children.iter().next().unwrap();
                        let mut visibility = tooltips_query.get_mut(*child).unwrap();
                        *visibility = Visibility::Hidden;
                    }
                    super::ui_helpers::ButtonTypes::Back => {
                        let child = children.iter().next().unwrap();
                        let mut visibility = tooltips_query.get_mut(*child).unwrap();
                        *visibility = Visibility::Hidden;
                    }
                }
            }
        }
    }
}

pub fn change_color_pallete(
    mut interaction_query: Query<
        (&Interaction, &ChangeColor, &mut BackgroundColor),
        (Changed<Interaction>, With<ChangeColor>, Without<Rectangle>),
    >,
    mut nodes: Query<(&mut BackgroundColor, &Rectangle), With<Rectangle>>,
    state: Res<AppState>,
) {
    for (interaction, change_color, mut bg_color) in &mut interaction_query {
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
            Interaction::Hovered => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.5);
            }
            Interaction::None => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 1.);
            }
        }
    }
}

pub fn change_text_pos(
    mut interaction_query: Query<
        (&Interaction, &TextPosMode, &mut BackgroundColor),
        (Changed<Interaction>, With<TextPosMode>),
    >,
    mut nodes: Query<(&mut Style, &Rectangle), With<Rectangle>>,
    state: Res<AppState>,
) {
    for (interaction, text_pos_mode, mut bg_color) in &mut interaction_query {
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
            Interaction::Hovered => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.8);
            }
            Interaction::None => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.5);
            }
        }
    }
}

pub fn change_arrow_type(
    mut interaction_query: Query<
        (&Interaction, &ArrowMode, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<ArrowMode>),
    >,
    mut state: ResMut<AppState>,
    mut tooltips_query: Query<&mut Visibility, With<Tooltip>>,
) {
    for (interaction, arrow_mode, mut bg_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state.arrow_type = arrow_mode.arrow_type;
            }
            Interaction::Hovered => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.8);
                let child = children.iter().next().unwrap();
                let mut visibility = tooltips_query.get_mut(*child).unwrap();
                *visibility = Visibility::Visible;
            }
            Interaction::None => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.5);
                let child = children.iter().next().unwrap();
                let mut visibility = tooltips_query.get_mut(*child).unwrap();
                *visibility = Visibility::Hidden;
            }
        }
    }
}

pub fn text_manipulation(
    mut interaction_query: Query<
        (
            &Interaction,
            &TextManipulationAction,
            &mut BackgroundColor,
            &Children,
        ),
        (Changed<Interaction>, With<TextManipulationAction>),
    >,
    mut editable_text: Query<(&mut Text, &EditableText), With<EditableText>>,
    mut tooltips_query: Query<&mut Visibility, With<Tooltip>>,
    state: Res<AppState>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/iosevka-regular.ttf");
    for (interaction, text_manipulation, mut bg_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                #[cfg(not(target_arch = "wasm32"))]
                let mut clipboard = arboard::Clipboard::new().unwrap();

                match text_manipulation.action_type {
                    TextManipulation::Cut => {
                        if let Some(id) = state.entity_to_edit {
                            for (mut text, node) in editable_text.iter_mut() {
                                if node.id == id {
                                    let mut str = "".to_string();
                                    for section in text.sections.iter_mut() {
                                        str = format!("{}{}", str, section.value.clone());
                                    }
                                    text.sections = vec![TextSection {
                                        value: "".to_string(),
                                        style: TextStyle {
                                            font: font.clone(),
                                            font_size: 20.0,
                                            color: Color::BLACK,
                                        },
                                    }];
                                    #[cfg(not(target_arch = "wasm32"))]
                                    clipboard.set_text(str).unwrap()
                                }
                            }
                        }
                    }
                    TextManipulation::Paste =>
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        if let Ok(clipboard_text) = clipboard.get_text() {
                            for (mut text, editable_text) in editable_text.iter_mut() {
                                if Some(editable_text.id) == state.entity_to_edit {
                                    let mut str = "".to_string();
                                    for section in text.sections.iter_mut() {
                                        str = format!("{}{}", str, section.value.clone());
                                    }
                                    str = format!("{}{}", str, clipboard_text);
                                    text.sections = get_sections(str, font.clone()).0;
                                }
                            }
                        }
                    }
                    TextManipulation::Copy => {
                        if let Some(id) = state.entity_to_edit {
                            for (mut text, node) in editable_text.iter_mut() {
                                if node.id == id {
                                    let mut str = "".to_string();
                                    for section in text.sections.iter_mut() {
                                        str = format!("{}{}", str, section.value.clone());
                                    }
                                    #[cfg(not(target_arch = "wasm32"))]
                                    clipboard.set_text(str).unwrap()
                                }
                            }
                        }
                    }
                    TextManipulation::OpenAllLinks => {
                        if let Some(id) = state.entity_to_edit {
                            for (mut text, node) in editable_text.iter_mut() {
                                if node.id == id {
                                    let mut str = "".to_string();
                                    for section in text.sections.iter_mut() {
                                        str = format!("{}{}", str, section.value.clone());
                                    }
                                    let (sections, is_link) = get_sections(str, font.clone());
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
            Interaction::Hovered => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.8);
                let child = children.iter().next().unwrap();
                let mut visibility = tooltips_query.get_mut(*child).unwrap();
                *visibility = Visibility::Visible;
            }
            Interaction::None => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.5);
                let child = children.iter().next().unwrap();
                let mut visibility = tooltips_query.get_mut(*child).unwrap();
                *visibility = Visibility::Hidden;
            }
        }
    }
}

pub fn new_doc_handler(
    mut commands: Commands,
    mut new_doc_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<NewDoc>),
    >,
    mut doc_list_query: Query<Entity, With<DocList>>,
    mut state: ResMut<AppState>,
    mut events: EventWriter<UpdateListHighlight>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut bg_color) in &mut new_doc_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let font = asset_server.load("fonts/iosevka-regular.ttf");
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
                state.docs.insert(
                    doc_id,
                    Doc {
                        id: doc_id,
                        name: name.clone(),
                        tabs,
                        tags: vec![],
                    },
                );
                commands.insert_resource(SaveRequest {
                    doc_id: Some(state.current_document.unwrap()),
                    tab_id: None,
                });
                state.current_document = Some(doc_id);
                commands.insert_resource(LoadRequest {
                    doc_id: None,
                    drop_last_checkpoint: false,
                });
                let button = add_list_item(&mut commands, font.clone(), doc_id, name);
                let doc_list = doc_list_query.single_mut();
                commands.entity(doc_list).add_child(button);
                events.send(UpdateListHighlight);
            }
            Interaction::Hovered => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.8);
            }
            Interaction::None => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.5);
            }
        }
    }
}

pub fn rename_doc_handler(
    mut rename_doc_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<RenameDoc>),
    >,
    mut state: ResMut<AppState>,
) {
    for (interaction, mut bg_color) in &mut rename_doc_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                state.entity_to_edit = None;
                state.tab_to_edit = None;
                let current_document = state.current_document.unwrap();
                state.doc_to_edit = Some(current_document);
            }
            Interaction::Hovered => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.8);
            }
            Interaction::None => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.5);
            }
        }
    }
}

pub fn doc_keyboard_input_system(
    mut query: Query<(&mut Text, &DocListItemText), With<DocListItemText>>,
    mut state: ResMut<AppState>,
    input: Res<Input<KeyCode>>,
    mut char_evr: EventReader<ReceivedCharacter>,
    mut deleting: Local<bool>,
) {
    for (mut text, doc_list_item) in &mut query.iter_mut() {
        if Some(doc_list_item.id) == state.doc_to_edit {
            if text.sections[0].value == *"Untitled" {
                text.sections[0].value = "".to_string();
            }
            if input.just_pressed(KeyCode::Return) {
                state.doc_to_edit = None;
                continue;
            }
            let mut str = text.sections[0].value.clone();
            if input.just_pressed(KeyCode::Back) {
                *deleting = true;
                str.pop();
            } else if input.just_released(KeyCode::Back) {
                *deleting = false;
            } else {
                for ev in char_evr.iter() {
                    if *deleting {
                        str.pop();
                    } else {
                        str = format!("{}{}", text.sections[0].value, ev.char);
                    }
                }
            }
            text.sections[0].value = str;
            let doc = state.docs.get_mut(&doc_list_item.id).unwrap();
            doc.name = text.sections[0].value.clone();
        }
    }
}

pub fn delete_doc_handler(
    mut commands: Commands,
    mut delete_doc_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<DeleteDoc>),
    >,
    mut state: ResMut<AppState>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut bg_color) in &mut delete_doc_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let font = asset_server.load("fonts/iosevka-regular.ttf");
                let id = ReflectableUuid(Uuid::new_v4());
                state.entity_to_edit = None;
                state.tab_to_edit = None;
                state.doc_to_edit = None;
                state.path_modal_id = Some(id);
                let entity = spawn_modal(&mut commands, font.clone(), id, ModalEntity::Document);
                commands.entity(state.main_panel.unwrap()).add_child(entity);
            }
            Interaction::Hovered => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.8);
            }
            Interaction::None => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.5);
            }
        }
    }
}

pub fn save_doc_handler(
    mut commands: Commands,
    mut save_doc_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SaveDoc>),
    >,
    state: Res<AppState>,
) {
    for (interaction, mut bg_color) in &mut save_doc_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                commands.insert_resource(SaveRequest {
                    doc_id: Some(state.current_document.unwrap()),
                    tab_id: None,
                });
            }
            Interaction::Hovered => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.8);
            }
            Interaction::None => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.5);
            }
        }
    }
}
