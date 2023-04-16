use bevy_ui_borders::BorderColor;

use bevy::prelude::*;

use super::{add_rectangle_txt, ModalCancel, ModalConfirm, ModalEntity, ModalTop};
use crate::utils::ReflectableUuid;
pub fn spawn_modal(
    commands: &mut Commands,
    font: Handle<Font>,
    id: ReflectableUuid,
    modal_entity: ModalEntity,
) -> Entity {
    let width = 300.;
    let height = 200.;
    commands
        .spawn((
            NodeBundle {
                z_index: ZIndex::Global(1),
                background_color: Color::WHITE.into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Px(width), Val::Px(height)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            BorderColor(Color::BLACK),
            ModalTop {
                id,
                delete: modal_entity.clone(),
            },
        ))
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        size: Size {
                            width: Val::Percent(100.),
                            height: Val::Percent(50.),
                        },
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder: &mut ChildBuilder| {
                    builder
                        .spawn(ButtonBundle {
                            style: Style {
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                // overflow: Overflow::Hidden,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|builder| {
                            builder.spawn(add_rectangle_txt(
                                font.clone(),
                                format!("Are you sure you want to delete {}?", modal_entity),
                            ));
                        });
                });

            builder
                .spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        size: Size {
                            width: Val::Percent(100.),
                            height: Val::Percent(50.),
                        },
                        justify_content: JustifyContent::SpaceAround,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    builder
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    border: UiRect::all(Val::Px(1.)),
                                    align_items: AlignItems::Center,
                                    padding: UiRect {
                                        left: Val::Px(5.),
                                        right: Val::Px(5.),
                                        top: Val::Px(5.),
                                        bottom: Val::Px(5.),
                                    },
                                    // overflow: Overflow::Hidden,
                                    ..default()
                                },
                                ..default()
                            },
                            BorderColor(Color::BLACK),
                            ModalConfirm {
                                id,
                                delete: modal_entity.clone(),
                            },
                        ))
                        .with_children(|builder| {
                            builder.spawn(add_rectangle_txt(font.clone(), "Yes".to_string()));
                        });
                    builder
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(1.)),
                                    padding: UiRect {
                                        left: Val::Px(5.),
                                        right: Val::Px(5.),
                                        top: Val::Px(5.),
                                        bottom: Val::Px(5.),
                                    },
                                    ..default()
                                },
                                ..default()
                            },
                            BorderColor(Color::BLACK),
                            ModalCancel { id },
                        ))
                        .with_children(|builder| {
                            builder.spawn(add_rectangle_txt(font.clone(), "Cancel".to_string()));
                        });
                });
        })
        .id()
}
