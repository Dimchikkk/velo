use bevy::prelude::*;
use bevy_ui_borders::*;

fn spawn_example(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                flex_basis: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                size: Size::new(Val::Px(100.), Val::Px(100.)),
                                border: UiRect::all(Val::Px(10.)),
                                ..Default::default()
                            },
                            background_color: Color::NAVY.into(),
                            ..Default::default()
                        },
                        BorderColor(Color::RED),
                        Outline::all(Color::WHITE, Val::Px(10.)),
                    ));
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                size: Size::new(Val::Px(100.), Val::Px(100.)),
                                border: UiRect::all(Val::Px(10.)),
                                ..Default::default()
                            },
                            background_color: Color::GREEN.into(),
                            ..Default::default()
                        },
                        BorderColor(Color::DARK_GREEN),
                    ));
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                size: Size::new(Val::Px(100.), Val::Px(100.)),
                                border: UiRect::all(Val::Px(10.)),
                                ..Default::default()
                            },
                            background_color: Color::NAVY.into(),
                            ..Default::default()
                        },
                        BorderColor(Color::RED),
                    ));
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                size: Size::new(Val::Px(100.), Val::Px(100.)),
                                border: UiRect::all(Val::Px(10.)),
                                ..Default::default()
                            },
                            background_color: Color::GREEN.into(),
                            ..Default::default()
                        },
                        BorderColor(Color::DARK_GREEN),
                        Outline::all(Color::WHITE, Val::Px(10.)),
                    ));
                });
        });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BordersPlugin)
        .add_startup_system(spawn_example)
        .run();
}
