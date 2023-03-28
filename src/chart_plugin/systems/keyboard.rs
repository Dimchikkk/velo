use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

#[cfg(not(target_arch = "wasm32"))]
use image::*;

pub use ron::de::SpannedError as ParseError;
pub use ron::Error as DeserializeError;
use std::convert::TryInto;
use uuid::Uuid;

use crate::{AppState, LoadRequest, SaveRequest};

use super::ui_helpers::{spawn_path_modal, EditableText, ReflectableUuid};

pub fn keyboard_input_system(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut state: ResMut<AppState>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Text, &EditableText), With<EditableText>>,
    mut char_evr: EventReader<ReceivedCharacter>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/iosevka-regular.ttf");
    let command = input.any_pressed([KeyCode::RWin, KeyCode::LWin]);
    let shift = input.any_pressed([KeyCode::RShift, KeyCode::LShift]);

    if command && input.just_pressed(KeyCode::V) {
        #[cfg(not(target_arch = "wasm32"))]
        insert_from_clipboard(&mut commands, &mut images, &mut state, &mut query);
    } else if command && shift && input.just_pressed(KeyCode::S) {
        let id = ReflectableUuid(Uuid::new_v4());
        state.path_modal_id = Some(id);
        state.entity_to_edit = None;
        spawn_path_modal(&mut commands, font, id, true);
    } else if command && shift && input.just_pressed(KeyCode::L) {
        let id = ReflectableUuid(Uuid::new_v4());
        state.path_modal_id = Some(id);
        state.entity_to_edit = None;
        spawn_path_modal(&mut commands, font, id, false);
    } else if command && input.just_pressed(KeyCode::S) {
        commands.insert_resource(SaveRequest { path: None });
    } else if command && input.just_pressed(KeyCode::L) {
        commands.insert_resource(LoadRequest { path: None });
    } else {
        for (mut text, editable_text) in &mut query.iter_mut() {
            if Some(editable_text.id) == state.entity_to_edit {
                if input.just_pressed(KeyCode::Back) {
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
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn insert_from_clipboard(
    commands: &mut Commands,
    images: &mut ResMut<Assets<Image>>,
    state: &mut ResMut<AppState>,
    query: &mut Query<(&mut Text, &EditableText), With<EditableText>>,
) {
    use super::ui_helpers::spawn_node;
    use crate::chart_plugin::ui_helpers::NodeMeta;

    let mut clipboard = arboard::Clipboard::new().unwrap();
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
        let entity = spawn_node(
            commands,
            NodeMeta {
                font: Handle::default(),
                size: Vec2::new(size.width as f32, size.height as f32),
                id: ReflectableUuid(Uuid::new_v4()),
                image: Some(image.into()),
            },
        );
        commands.entity(state.main_panel.unwrap()).add_child(entity);
    }

    if let Ok(clipboard_text) = clipboard.get_text() {
        for (mut text, editable_text) in &mut query.iter_mut() {
            if Some(editable_text.id) == state.entity_to_edit {
                text.sections[0].value =
                    format!("{}{}", text.sections[0].value, clipboard_text.clone());
            }
        }
    }
}
