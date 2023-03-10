use bevy::{prelude::*, ui::RelativeCursorPosition};

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update_pos)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let background_image = asset_server.load("bg.png");
    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands.spawn(SpriteBundle {
        texture: background_image,
        ..Default::default()
    });
    // Rectangle
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.1)),
        ..default()
    }, RelativeCursorPosition::default()));
}

fn update_pos(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut sprite_position: Query<(&mut Transform, &RelativeCursorPosition)>, 
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_q.single();
    for event in cursor_moved_events.iter() {
        for (mut transform, _) in sprite_position.iter_mut() {
            if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, event.position) {
                transform.translation.x = world_position.x;
                transform.translation.y = world_position.y;
            }
        }
    }
}