use bevy::prelude::*;
use bevy_ui_borders::*;

fn spawn_example(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(BorderedNodeBundle {
        style: Style {
            size: Size::new(Val::Px(100.), Val::Px(100.)),
            margin: UiRect::all(Val::Px(100.)),
            border: UiRect::all(Val::Px(10.)),
            ..Default::default()
        },
        background_color: Color::WHITE.into(),
        border_color: Color::RED.into(),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BordersPlugin)
        .add_startup_system(spawn_example)
        .run();
}
