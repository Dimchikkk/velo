use bevy::{prelude::*, window::PrimaryWindow};

use uuid::Uuid;

use crate::{AddRect, AppState, JsonNode, NodeType};

use super::ui_helpers::{ArrowMeta, ArrowMode, ButtonAction, ChangeColor, Rectangle};

pub fn button_handler(
    mut commands: Commands,
    mut events: EventWriter<AddRect>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonAction),
        (Changed<Interaction>, With<ButtonAction>),
    >,
    mut nodes: Query<(Entity, &Rectangle, &mut ZIndex), With<Rectangle>>,
    arrows: Query<(Entity, &ArrowMeta), With<ArrowMeta>>,
    mut state: ResMut<AppState>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    for (interaction, mut color, button_action) in &mut interaction_query {
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
                            text: "".to_string(),
                            bg_color: Color::WHITE,
                            tags: vec![],
                            text_pos: crate::TextPos::Center,
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
                super::ui_helpers::ButtonTypes::Tag => {
                    eprintln!("Tagging is not implemented yet");
                }
                super::ui_helpers::ButtonTypes::Untag => {
                    eprintln!("Untagging is not implemented yet");
                }
            },
            Interaction::Hovered => {
                color.0 = Color::GRAY;
            }
            Interaction::None => {
                color.0 = Color::rgb(0.8, 0.8, 0.8);
            }
        }
    }
}

pub fn change_color_pallete(
    interaction_query: Query<
        (&Interaction, &ChangeColor),
        (Changed<Interaction>, With<ChangeColor>),
    >,
    mut nodes: Query<(&mut BackgroundColor, &Rectangle), With<Rectangle>>,
    state: Res<AppState>,
) {
    for (interaction, change_color) in &interaction_query {
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

pub fn change_arrow_type(
    mut interaction_query: Query<
        (&Interaction, &ArrowMode, &mut BackgroundColor),
        (Changed<Interaction>, With<ArrowMode>),
    >,
    mut state: ResMut<AppState>,
) {
    for (interaction, arrow_mode, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state.arrow_type = arrow_mode.arrow_type;
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
