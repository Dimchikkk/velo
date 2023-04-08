use bevy::prelude::*;
use bevy_ui_borders::BorderColor;

use super::ui_helpers::add_rectangle_txt;

pub fn add_new_delete_rec(
    commands: &mut Commands,
    font: Handle<Font>,
    label_do: String,
    label_undo: String,
    component_do: impl Component,
    component_undo: impl Component,
) -> Entity {
    let node = commands
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(85.), Val::Percent(15.)),
                margin: UiRect {
                    left: Val::Px(5.),
                    right: Val::Px(5.),
                    top: Val::Px(5.),
                    bottom: Val::Px(5.),
                },
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .id();
    let do_button = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                style: Style {
                    size: Size::new(Val::Percent(40.), Val::Percent(100.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.)),
                    ..default()
                },
                ..default()
            },
            component_do,
            BorderColor(Color::BLACK),
        ))
        .with_children(|builder| {
            builder.spawn(add_rectangle_txt(font.clone(), label_do));
        })
        .id();

    let undo_button = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                style: Style {
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(40.), Val::Percent(100.)),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.)),
                    ..default()
                },
                ..default()
            },
            component_undo,
            BorderColor(Color::BLACK),
        ))
        .with_children(|builder| {
            builder.spawn(add_rectangle_txt(font.clone(), label_undo));
        })
        .id();
    commands.entity(node).add_child(do_button);
    commands.entity(node).add_child(undo_button);
    node
}
