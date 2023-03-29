use bevy::prelude::*;

use uuid::Uuid;

use crate::{AddRect, AppState, JsonNode, NodeType};

use super::ui_helpers::{ArrowMeta, ButtonAction, Rectangle};

pub fn button_handler(
    mut commands: Commands,
    mut events: EventWriter<AddRect>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonAction),
        (Changed<Interaction>, With<ButtonAction>),
    >,
    nodes: Query<(Entity, &Rectangle), With<Rectangle>>,
    arrows: Query<(Entity, &ArrowMeta), With<ArrowMeta>>,
    mut state: ResMut<AppState>,
) {
    for (interaction, mut color, button_action) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => match button_action.button_type {
                super::ui_helpers::ButtonTypes::ADD => {
                    events.send(AddRect {
                        node: JsonNode {
                            id: Uuid::new_v4(),
                            node_type: NodeType::RECT,
                            left: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                            width: Val::Px(100.0),
                            height: Val::Px(100.0),
                            text: "".to_string(),
                            bg_color: Color::WHITE,
                        },
                        image: None,
                    });
                }
                super::ui_helpers::ButtonTypes::DEL => {
                    if let Some(id) = state.entity_to_edit {
                        state.entity_to_edit = None;
                        state.entity_to_resize = None;
                        state.hold_entity = None;
                        state.arrow_to_draw_start = None;
                        for (entity, node) in nodes.iter() {
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
                super::ui_helpers::ButtonTypes::FRONT => {},
                super::ui_helpers::ButtonTypes::BACK => {},
                super::ui_helpers::ButtonTypes::TAG => {},
                super::ui_helpers::ButtonTypes::UNTAG => {},
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
