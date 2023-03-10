#[cfg(not(target_arch = "wasm32"))]
use arboard::*;
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    text::Text2dBounds,
    ui::RelativeCursorPosition,
};
#[cfg(not(target_arch = "wasm32"))]
use image::*;
#[cfg(not(target_arch = "wasm32"))]
use std::convert::TryInto;

#[derive(Component)]
pub struct MainCamera;

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
struct InputText;

fn update_text_on_typing(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Text, With<InputText>>,
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
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(box_size),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(x, y, 0.1)),
                    ..default()
                },
                RelativeCursorPosition::default(),
            ))
            .with_children(|builder| {
                builder.spawn((
                    Text2dBundle {
                        text: Text {
                            sections: vec![TextSection::new("", text_style.clone())],
                            alignment: TextAlignment::Center,
                            linebreak_behaviour: bevy::text::BreakLineOn::WordBoundary,
                        },
                        text_2d_bounds: Text2dBounds {
                            // Wrap text in the rectangle
                            size: box_size,
                        },
                        // ensure the text is drawn on top of the box
                        transform: Transform::from_translation(Vec3::Z),
                        ..default()
                    },
                    InputText,
                ));
            });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn insert_image_from_clipboard(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut clipboard = Clipboard::new().unwrap();
    if let Ok(image) = clipboard.get_image() {
        let image: RgbaImage = ImageBuffer::from_raw(
            image.width.try_into().unwrap(),
            image.height.try_into().unwrap(),
            image.bytes.into_owned(),
        )
        .unwrap();
        let size: Extent3d = Extent3d {
            width: image.width(),
            height: image.height(),
            ..Default::default()
        };
        let image = Image::new(
            size,
            TextureDimension::D2,
            image.to_vec(),
            TextureFormat::Rgba8UnormSrgb,
        );
        let image = images.add(image);
        commands.spawn(SpriteBundle {
            texture: image,
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        });
    }
}
