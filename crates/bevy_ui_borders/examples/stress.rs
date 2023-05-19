use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_ui_borders::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::Immediate,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(BordersPlugin)
        .add_startup_system(spawn_example)
        .run();
}

fn spawn_example(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let container_id = commands
        .spawn(BorderedNodeBundle {
            style: Style {
                size: Size::new(Val::Px(500.), Val::Px(500.)),
                flex_wrap: FlexWrap::Wrap,
                align_content: AlignContent::Start,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                ..Default::default()
            },
            background_color: Color::BLACK.into(),
            border_color: Color::RED.into(),
            ..Default::default()
        })
        .id();

    for _ in 0..10_000 {
        let child_id = commands
            .spawn(BorderedNodeBundle {
                style: Style {
                    size: Size::new(Val::Px(5.), Val::Px(5.)),
                    border: UiRect::all(Val::Px(1.)),
                    ..Default::default()
                },
                background_color: Color::NAVY.into(),
                border_color: Color::YELLOW.into(),
                ..Default::default()
            })
            .id();
        commands.entity(container_id).add_child(child_id);
    }
}
