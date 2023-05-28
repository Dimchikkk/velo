use bevy_cosmic_edit::{spawn_cosmic_edit, ActiveEditor, CosmicEditMeta, FontSystemState};
use bevy_ui_borders::BorderColor;

use bevy::prelude::*;

use super::{
    add_rectangle_txt, EditableText, GenericButton, ModalAction, ModalCancel, ModalConfirm,
    ModalTop,
};
use crate::{
    ui_plugin::TextPos,
    utils::{to_cosmic_text_pos, ReflectableUuid},
};

pub fn spawn_modal(
    commands: &mut Commands,
    font_system: &mut ResMut<FontSystemState>,
    window: &Window,
    id: ReflectableUuid,
    modal_action: ModalAction,
) -> Entity {
    let width = 350.;
    let height = 250.;
    let default_value = match modal_action {
        ModalAction::SaveToFile => "./velo.json".to_string(),
        ModalAction::LoadFromFile => "./velo.json".to_string(),
        ModalAction::LoadFromUrl => "https://gist..".to_string(),
        _ => "".to_string(),
    };
    let top = commands
        .spawn((
            NodeBundle {
                z_index: ZIndex::Global(1),
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Stretch,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(window.width() / 2. - 250.),
                        bottom: Val::Px(window.height() / 2. - 50.),
                        ..default()
                    },
                    size: Size::new(Val::Px(width), Val::Px(height)),
                    ..default()
                },
                background_color: Color::BLACK.with_a(0.5).into(),
                ..default()
            },
            ModalTop {
                id,
                action: modal_action.clone(),
            },
        ))
        .id();
    let modal_static = commands
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                size: Size {
                    width: Val::Percent(100.),
                    height: Val::Percent(30.),
                },
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            ..default()
        })
        .id();

    let ok_button = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgb(63.0 / 255.0, 81.0 / 255.0, 181.0 / 255.0).into(),
                style: Style {
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.)),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(5.)),
                    // overflow: Overflow::Hidden,
                    ..default()
                },
                ..default()
            },
            BorderColor(Color::BLACK),
            GenericButton,
            ModalConfirm {
                id,
                action: modal_action.clone(),
            },
        ))
        .with_children(|builder| {
            let text_style = TextStyle {
                font_size: 18.0,
                color: Color::rgb(1., 1., 1.),
                ..default()
            };

            builder.spawn(
                TextBundle::from_section(" Ok ", text_style).with_style(Style {
                    position_type: PositionType::Relative,
                    ..default()
                }),
            );
        })
        .id();
    let cancel_button = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgb(63.0 / 255.0, 81.0 / 255.0, 181.0 / 255.0).into(),
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.)),
                    padding: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                ..default()
            },
            GenericButton,
            BorderColor(Color::BLACK),
            ModalCancel { id },
        ))
        .with_children(|builder| {
            let text_style = TextStyle {
                font_size: 18.0,
                color: Color::rgb(1., 1., 1.),
                ..default()
            };

            builder.spawn(
                TextBundle::from_section("Cancel", text_style).with_style(Style {
                    position_type: PositionType::Relative,
                    ..default()
                }),
            );
        })
        .id();
    commands.entity(modal_static).add_child(ok_button);
    commands.entity(modal_static).add_child(cancel_button);

    let modal_dynamic = match modal_action {
        ModalAction::SaveToFile | ModalAction::LoadFromFile | ModalAction::LoadFromUrl => {
            let top = commands
                .spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceAround,
                        padding: UiRect::all(Val::Px(20.)),
                        size: Size {
                            width: Val::Percent(100.),
                            height: Val::Percent(70.),
                        },
                        ..default()
                    },
                    ..default()
                })
                .id();
            let label = commands
                .spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size {
                            width: Val::Px(50.),
                            height: Val::Percent(30.),
                        },
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn(add_rectangle_txt(modal_action.to_string()));
                })
                .id();
            let width = 180.;
            let height = 35.;
            let button = commands
                .spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Start,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(1.)),
                            size: Size {
                                width: Val::Px(width),
                                height: Val::Px(height),
                            },
                            padding: UiRect::all(Val::Px(5.)),
                            ..default()
                        },
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                ))
                .id();
            let cosmic_edit_meta = CosmicEditMeta {
                text: default_value,
                text_pos: to_cosmic_text_pos(TextPos::Center),
                initial_background: None,
                initial_size: Some((width, height)),
                font_size: 14.,
                line_height: 18.,
                scale_factor: window.scale_factor() as f32,
                font_system: font_system.font_system.as_mut().unwrap(),
                is_visible: true,
            };
            let cosmic_edit = spawn_cosmic_edit(commands, cosmic_edit_meta);
            commands.entity(cosmic_edit).insert(EditableText { id });
            commands.insert_resource(ActiveEditor {
                entity: Some(cosmic_edit),
            });
            commands.entity(top).add_child(label);
            commands.entity(button).add_child(cosmic_edit);
            commands.entity(top).add_child(button);
            top
        }
        ModalAction::DeleteDocument | ModalAction::DeleteTab => {
            let top = commands
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
                .id();
            let node = commands
                .spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .id();
            let node_label = commands
                .spawn(add_rectangle_txt(format!(
                    "Are you sure you want to {}?",
                    modal_action
                )))
                .id();
            commands.entity(node).add_child(node_label);
            commands.entity(top).add_child(node);
            top
        }
    };
    let modal = commands
        .spawn((
            NodeBundle {
                background_color: Color::WHITE.into(),
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.)),
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(-3.),
                        right: Val::Px(0.),
                        top: Val::Px(-3.),
                        bottom: Val::Px(0.),
                    },
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            BorderColor(Color::BLACK),
        ))
        .id();
    commands.entity(modal).add_child(modal_dynamic);
    commands.entity(modal).add_child(modal_static);
    commands.entity(top).add_child(modal);
    top
}
