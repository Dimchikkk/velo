use bevy_ui_borders::BorderColor;

use bevy::prelude::*;

use super::{
    add_rectangle_txt, create_rectangle_txt, EditableText, ModalAction, ModalCancel, ModalConfirm,
    ModalTop,
};
use crate::utils::ReflectableUuid;
pub fn spawn_modal(
    commands: &mut Commands,
    window: &Window,
    id: ReflectableUuid,
    modal_action: ModalAction,
) -> Entity {
    let width = 350.;
    let height = 250.;
    let default_value = match modal_action {
        ModalAction::SaveToFile => "./velo.json".to_string(),
        ModalAction::LoadFromFile => "./velo.json".to_string(),
        ModalAction::LoadFromUrl => "https://<path-to-velo.json>".to_string(),
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
                            padding: UiRect::all(Val::Px(5.)),
                            // overflow: Overflow::Hidden,
                            ..default()
                        },
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    ModalConfirm {
                        id,
                        action: modal_action.clone(),
                    },
                ))
                .with_children(|builder| {
                    builder.spawn(add_rectangle_txt(" Ok ".to_string()));
                });
            builder
                .spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(1.)),
                            padding: UiRect::all(Val::Px(5.)),
                            ..default()
                        },
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    ModalCancel { id },
                ))
                .with_children(|builder| {
                    builder.spawn(add_rectangle_txt("Cancel".to_string()));
                });
        })
        .id();

    let modal_dynamic = match modal_action {
        ModalAction::SaveToFile | ModalAction::LoadFromFile | ModalAction::LoadFromUrl => {
            commands
                .spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceAround,
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
                                margin: UiRect {
                                    left: Val::Px(20.),
                                    ..default()
                                },
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
                        });
                    builder
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    justify_content: JustifyContent::Start,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(1.)),
                                    size: Size {
                                        width: Val::Px(220.),
                                        height: Val::Percent(30.),
                                    },
                                    padding: UiRect::all(Val::Px(5.)),
                                    // overflow: Overflow::Hidden,
                                    ..default()
                                },
                                ..default()
                            },
                            BorderColor(Color::BLACK),
                        ))
                        .with_children(|builder| {
                            builder.spawn((
                                create_rectangle_txt(default_value, None),
                                EditableText { id },
                            ));
                        });
                })
                .id()
        }
        ModalAction::DeleteDocument | ModalAction::DeleteTab => {
            commands
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
                            builder.spawn(add_rectangle_txt(format!(
                                "Are you sure you want to {}?",
                                modal_action
                            )));
                        });
                })
                .id()
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
