use base64::{engine::general_purpose, Engine};
use bevy::{
    ecs::schedule::SystemConfig,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy::{
    reflect::erased_serde::__private::serde::de::DeserializeSeed, scene::serde::SceneDeserializer,
};
#[cfg(not(target_arch = "wasm32"))]
use image::*;
use moonshine_save::{
    load::{self, load, unload},
    prelude::{LoadSet, Unload},
    save::{Save, Saved},
};

pub use ron::de::SpannedError as ParseError;
use ron::Deserializer;
pub use ron::Error as DeserializeError;
use serde_json::Value;
use std::collections::VecDeque;

use crate::{AppState, LoadRequest};

use super::ui_helpers::{ArrowMeta, CreateArrow, Rectangle};

pub fn should_load(request: Option<Res<LoadRequest>>) -> bool {
    request.is_some()
}

pub fn remove_load_request(world: &mut World) {
    world.remove_resource::<LoadRequest>().unwrap();
}

pub fn load_ron() -> SystemConfig {
    from_file_or_memory
        .pipe(unload::<Or<(With<Save>, With<Unload>)>>)
        .pipe(load)
        .pipe(load::finish)
        .in_set(LoadSet::Load)
}

pub fn post_load(
    mut rec: Query<(&Rectangle, &mut UiImage), With<Rectangle>>,
    request: Res<LoadRequest>,
    mut state: ResMut<AppState>,
    mut commands: Commands,
    mut res_images: ResMut<Assets<Image>>,
    old_arrows: Query<Entity, With<ArrowMeta>>,
    mut create_arrow: EventWriter<CreateArrow>,
) {
    let mut json: Value = match &request.path {
        Some(path) => {
            let json = std::fs::read_to_string(path).expect("Error reading state from file");
            serde_json::from_str(&json).unwrap()
        }
        None => {
            let json = if state.checkpoints.len() == 1 {
                state.checkpoints.back().unwrap().clone()
            } else {
                state.checkpoints.pop_back().unwrap()
            };
            serde_json::from_str(&json).unwrap()
        }
    };
    let images = json["images"].as_object_mut().unwrap();
    for (rect, mut ui_image) in rec.iter_mut() {
        if images.contains_key(&rect.id.0.to_string()) {
            let image = images.get(&rect.id.0.to_string()).unwrap();
            let image_bytes = general_purpose::STANDARD
                .decode(image.as_str().unwrap().as_bytes())
                .unwrap();
            #[cfg(not(target_arch = "wasm32"))]
            {
                let img = load_from_memory_with_format(&image_bytes, ImageFormat::Png).unwrap();
                let size: Extent3d = Extent3d {
                    width: img.width(),
                    height: img.height(),
                    ..Default::default()
                };
                let image = Image::new(
                    size,
                    TextureDimension::D2,
                    img.into_bytes(),
                    TextureFormat::Rgba8UnormSrgb,
                );
                let image_handle = res_images.add(image);
                ui_image.texture = image_handle;
            }
        }
    }

    for entity in old_arrows.iter() {
        commands.entity(entity).despawn_recursive();
    }
    let arrows = json["arrows"].as_array_mut().unwrap();
    for arrow in arrows.iter() {
        let arrow_meta: ArrowMeta = serde_json::from_value(arrow.clone()).unwrap();
        create_arrow.send(CreateArrow {
            start: arrow_meta.start,
            end: arrow_meta.end,
        });
    }
}

pub fn from_file_or_memory(
    type_registry: Res<AppTypeRegistry>,
    request: Res<LoadRequest>,
    mut state: ResMut<AppState>,
) -> Result<Saved, load::Error> {
    eprintln!("load: {:?}", request);

    state.entity_to_edit = None;
    state.hold_entity = None;
    state.entity_to_resize = None;
    state.arrow_to_draw_start = None;
    state.path_modal_id = None;

    let ron;
    match &request.path {
        Some(path) => {
            let json_bytes = std::fs::read(path).unwrap();
            let json: Value = serde_json::from_slice(&json_bytes).unwrap();
            ron = json["ron"].as_str().unwrap().to_string();
            state.checkpoints = VecDeque::new();
        }
        None => {
            let json: Value = serde_json::from_str(state.checkpoints.back().unwrap()).unwrap();
            ron = json["ron"].as_str().unwrap().to_string();
        }
    }
    let ron = general_purpose::STANDARD.decode(ron.as_bytes()).unwrap();
    let mut deserializer = Deserializer::from_bytes(&ron)?;
    let scene = {
        let type_registry = &type_registry.read();
        let scene_deserializer = SceneDeserializer { type_registry };
        scene_deserializer.deserialize(&mut deserializer)
    }?;
    Ok(Saved { scene })
}
