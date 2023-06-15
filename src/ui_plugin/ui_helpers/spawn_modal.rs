use bevy_cosmic_edit::{
    spawn_cosmic_edit, ActiveEditor, CosmicEditMeta, CosmicFont, CosmicMetrics, CosmicNode,
    CosmicText,
};
use bevy_ui_borders::BorderColor;

use bevy::prelude::*;
use cosmic_text::AttrsOwned;

use super::{
    add_rectangle_txt, EditableText, GenericButton, ModalAction, ModalCancel, ModalConfirm,
    ModalTop,
};
use crate::{
    themes::Theme,
    ui_plugin::TextPos,
    utils::{bevy_color_to_cosmic, ReflectableUuid},
};

pub fn spawn_modal(
    commands: &mut Commands,
    theme: &Res<Theme>,
    cosmic_fonts: &mut ResMut<Assets<CosmicFont>>,
    cosmic_font_handle: Handle<CosmicFont>,
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
                background_color: theme.shadow.into(),
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
                background_color: theme.ok_cancel_bg.into(),
                style: Style {
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.)),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                ..default()
            },
            BorderColor(theme.btn_border),
            GenericButton,
            ModalConfirm {
                id,
                action: modal_action.clone(),
            },
        ))
        .with_children(|builder| {
            let text_style = TextStyle {
                font_size: 18.0,
                color: theme.font,
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
                background_color: theme.ok_cancel_bg.into(),
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
            BorderColor(theme.btn_border),
            ModalCancel { id },
        ))
        .with_children(|builder| {
            let text_style = TextStyle {
                font_size: 18.0,
                color: theme.font,
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
                    builder.spawn(add_rectangle_txt(theme, modal_action.to_string()));
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
                            padding: UiRect {
                                top: Val::Px(7.),
                                left: Val::Px(7.),
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    },
                    BorderColor(theme.btn_border),
                ))
                .id();
            let mut attrs = cosmic_text::Attrs::new();
            attrs = attrs.family(cosmic_text::Family::Name(theme.font_name.as_str()));
            attrs = attrs.color(bevy_color_to_cosmic(theme.font));
            let cosmic_edit_meta = CosmicEditMeta {
                text: CosmicText::OneStyle(default_value),
                attrs: AttrsOwned::new(attrs),
                font_system_handle: cosmic_font_handle,
                text_pos: TextPos::TopLeft.into(),
                size: Some((width, height)),
                metrics: CosmicMetrics {
                    font_size: theme.font_size,
                    line_height: theme.line_height,
                    scale_factor: window.scale_factor() as f32,
                },
                bg: theme.modal_text_input_bg,
                node: CosmicNode::Ui,
                readonly: false,
                bg_image: None,
            };
            let cosmic_edit = spawn_cosmic_edit(commands, cosmic_fonts, cosmic_edit_meta);
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
                .spawn(add_rectangle_txt(
                    theme,
                    format!("Are you sure you want to {}?", modal_action),
                ))
                .id();
            commands.entity(node).add_child(node_label);
            commands.entity(top).add_child(node);
            top
        }
    };
    let modal = commands
        .spawn((
            NodeBundle {
                background_color: theme.modal_bg.into(),
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
            BorderColor(theme.btn_border),
        ))
        .id();
    commands.entity(modal).add_child(modal_dynamic);
    commands.entity(modal).add_child(modal_static);
    commands.entity(top).add_child(modal);
    top
}
