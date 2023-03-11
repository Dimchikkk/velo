use bevy::{prelude::*, ui::RelativeCursorPosition};
#[cfg(not(target_arch = "wasm32"))]
mod image_from_clipboard;
#[cfg(not(target_arch = "wasm32"))]
pub use image_from_clipboard::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct IRectangle;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((update_pos, update_text_on_typing, create_new_rectangle));

        #[cfg(not(target_arch = "wasm32"))]
        app.add_system(insert_image_from_clipboard);
    }
}

fn update_pos(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut sprite_position: Query<(&mut Transform, &RelativeCursorPosition)>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_q.single();
    for event in cursor_moved_events.iter() {
        let last_sprite = sprite_position.iter_mut().last();
        if let Some((mut transform, _)) = last_sprite {
            if let Some(world_position) =
                camera.viewport_to_world_2d(camera_transform, event.position)
            {
                transform.translation.x = world_position.x;
                transform.translation.y = world_position.y;
            }
        }
    }
}

#[derive(Component)]
struct EditableText;

fn update_text_on_typing(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Text, With<EditableText>>,
) {
    let last_sprite = query.iter_mut().last();
    if let Some(mut text) = last_sprite {
        if keys.just_pressed(KeyCode::Back) {
            let mut str = text.sections[0].value.clone();
            str.pop();
            text.sections[0].value = str;
        } else {
            for ev in char_evr.iter() {
                text.sections[0].value = format!("{}{}", text.sections[0].value, ev.char);
            }
        }
    }
}

fn create_new_rectangle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_q.single();
    let mut x = 0.0;
    let mut y = 0.0;
    for event in cursor_moved_events.iter() {
        if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, event.position)
        {
            x = world_position.x;
            y = world_position.y;
        }
    }
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let font = asset_server.load("fonts/SourceCodePro-Regular.ttf");
        let text_style = TextStyle {
            font,
            font_size: 18.0,
            color: Color::BLACK,
        };
        let box_size = Vec2::new(200.0, 200.0);
        // Rectangle
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        position: UiRect {
                            left: Val::Px(x),
                            bottom: Val::Px(y),
                            ..Default::default()
                        },
                        position_type: PositionType::Absolute,
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                IRectangle,
            ))
            .with_children(|builder| {
                builder.spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(box_size.x), Val::Px(box_size.y)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                }).with_children(|builder| {
                    builder.spawn((TextBundle::from_section(
                        "",
                        text_style.clone(),
                    )
                    .with_style(Style {
                        position_type: PositionType::Relative,
                        ..default()
                    }), EditableText));
                });
            });
    }
}
