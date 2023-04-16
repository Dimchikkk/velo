use bevy::prelude::*;


use crate::chart_plugin::ui_helpers::{get_tooltip, ButtonAction, GenericButton, Tooltip};



pub fn add_new_delete_rec(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    font: Handle<Font>,
    create_component: ButtonAction,
    delete_component: ButtonAction,
) -> Entity {
    let node = commands
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(80.), Val::Percent(18.)),
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
    let new_rec = commands
        .spawn((
            ButtonBundle {
                background_color: Color::BLACK.into(),
                image: asset_server.load("rec-add.png").into(),
                style: Style {
                    size: Size::new(Val::Px(45.), Val::Px(45.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                ..default()
            },
            create_component,
            GenericButton,
        ))
        .with_children(|builder| {
            builder.spawn((
                get_tooltip(font.clone(), "New Rectangle".to_string(), 14.),
                Tooltip,
            ));
        })
        .id();

    let del_rec = commands
        .spawn((
            ButtonBundle {
                background_color: Color::BLACK.into(),
                image: asset_server.load("rec-del.png").into(),
                style: Style {
                    size: Size::new(Val::Px(45.), Val::Px(45.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                ..default()
            },
            delete_component,
            GenericButton,
        ))
        .with_children(|builder| {
            builder.spawn((
                get_tooltip(font.clone(), "Delete Rectangle".to_string(), 14.),
                Tooltip,
            ));
        })
        .id();
    commands.entity(node).add_child(del_rec);
    commands.entity(node).add_child(new_rec);
    node
}
