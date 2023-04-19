use bevy_ui_borders::BorderColor;

use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::*,
};

use crate::utils::ReflectableUuid;

use super::{DeleteDoc, DocListItemButton, DocListItemContainer, DocListItemText, GenericButton};

pub fn add_list_item(
    commands: &mut Commands,
    font: Handle<Font>,
    id: ReflectableUuid,
    name: String,
) -> Entity {
    let root = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.)),
                    ..default()
                },
                ..default()
            },
            GenericButton,
            DocListItemContainer { id },
            BorderColor(Color::GRAY),
            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
        ))
        .id();
    let doc_button = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgba(0.8, 0.8, 0.8, 0.5).into(),
                style: Style {
                    size: Size::new(Val::Percent(90.), Val::Percent(100.)),
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                ..default()
            },
            DocListItemButton { id },
            GenericButton,
        ))
        .id();
    let doc_label = commands
        .spawn((
            TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: name,
                            style: TextStyle {
                                font: font.clone(),
                                font_size: 18.,
                                color: Color::BLACK,
                            },
                        },
                        TextSection {
                            value: " ".to_string(),
                            style: TextStyle {
                                font: font.clone(),
                                font_size: 18.,
                                color: Color::BLACK,
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
            DocListItemText { id },
            Label,
        ))
        .id();
    let del_button = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgba(0.8, 0.8, 0.8, 0.5).into(),
                style: Style {
                    size: Size::new(Val::Percent(10.), Val::Percent(100.)),
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                ..default()
            },
            DeleteDoc,
            GenericButton,
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
                            font,
                            font_size: 24.,
                            color: Color::BLACK,
                        },
                    }],
                    ..default()
                },
                ..default()
            },
            Label,
        ))
        .id();
    commands.entity(doc_button).add_child(doc_label);
    commands.entity(del_button).add_child(del_label);
    commands.entity(root).add_child(doc_button);
    commands.entity(root).add_child(del_button);
    root
}
