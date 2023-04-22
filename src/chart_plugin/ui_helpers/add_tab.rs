use bevy::prelude::*;

use crate::utils::ReflectableUuid;

use super::{DeleteTab, EditableText, GenericButton, TabButton, TabContainer};

pub fn add_tab(commands: &mut Commands, name: String, id: ReflectableUuid) -> Entity {
    let root = commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(80.), Val::Px(30.)),
                    justify_content: JustifyContent::Center,
                    margin: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                ..default()
            },
            TabContainer { id },
        ))
        .id();
    let tab_button = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgba(0.8, 0.8, 0.8, 0.5).into(),
                style: Style {
                    size: Size::new(Val::Percent(90.), Val::Percent(100.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    overflow: Overflow::Hidden,
                    ..default()
                },
                ..default()
            },
            GenericButton,
            TabButton { id },
        ))
        .id();
    let tab_label = commands
        .spawn((
            TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: name,
                            style: TextStyle {
                                font_size: 18.,
                                color: Color::BLACK,
                                ..default()
                            },
                        },
                        TextSection {
                            value: " ".to_string(),
                            style: TextStyle {
                                font_size: 18.,
                                color: Color::BLACK,
                                ..default()
                            },
                        },
                    ],
                    ..default()
                },
                style: Style {
                    margin: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                ..default()
            },
            EditableText { id },
        ))
        .id();
    let del_button = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgba(0.8, 0.8, 0.8, 0.5).into(),
                visibility: Visibility::Hidden,
                style: Style {
                    size: Size::new(Val::Percent(10.), Val::Percent(100.)),
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                ..default()
            },
            GenericButton,
            DeleteTab,
        ))
        .id();
    let del_label = commands
        .spawn((
            TextBundle {
                style: Style {
                    margin: UiRect {
                        left: Val::Px(5.),
                        right: Val::Px(5.),
                        top: Val::Px(0.),
                        bottom: Val::Px(0.),
                    },
                    ..default()
                },
                text: Text {
                    sections: vec![TextSection {
                        value: "x".to_string(),
                        style: TextStyle {
                            font_size: 18.,
                            color: Color::BLACK,
                            ..default()
                        },
                    }],
                    ..default()
                },
                ..default()
            },
            Label,
        ))
        .id();
    commands.entity(tab_button).add_child(tab_label);
    commands.entity(del_button).add_child(del_label);
    commands.entity(root).add_child(tab_button);
    commands.entity(root).add_child(del_button);
    root
}
