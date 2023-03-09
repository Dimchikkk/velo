use bevy::{prelude::*, render::camera::RenderTarget, window::ReceivedCharacter};

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MousePos { x: i32, y: i32 }
}

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::MousePos { x: 0, y: 0 })
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .add_system(my_cursor_system)
        .add_system(print_char_event_system)
        .run();
}

fn setup(
    mut commands: Commands
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    // Rectangle
    commands.spawn((SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
            ..default()
        },
        Direction::Up
));
}

fn sprite_movement(app_state: Res<State<AppState>>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match app_state.current() {
            AppState::MousePos{ x , y} => {
                transform.translation.x = *x as f32;
                transform.translation.y = *y as f32;
            }
        }
    }
}

fn my_cursor_system(
    mut app_state: ResMut<State<AppState>>,
    // need to get window dimensions
    windows: Res<Windows>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    // get the window that the camera is displaying to (or the primary window)
    let window = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    { 
        // TODO: properly handle cast from f32 to i32
        app_state.set(AppState::MousePos { x: world_position.x.round() as i32, y: world_position.y.round() as i32 });
    }
}

/// This system prints out all char events as they come in
fn print_char_event_system(mut char_input_events: EventReader<ReceivedCharacter>) {
    for event in char_input_events.iter() {
        info!("{:?}: '{}'", event, event.char);
    }
}