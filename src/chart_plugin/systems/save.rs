use base64::{engine::general_purpose, Engine};
use bevy::{ecs::schedule::SystemConfig, prelude::*};

#[cfg(not(target_arch = "wasm32"))]
use image::*;
use moonshine_save::{
    prelude::SaveSet,
    save::{self, save, Save, Saved},
};
use regex::Regex;

pub use ron::de::SpannedError as ParseError;
pub use ron::Error as DeserializeError;
use serde_json::json;
use std::io::Cursor;

use crate::{AppState, SaveRequest};

use super::ui_helpers::{ArrowMeta, Rectangle};

const MAX_AMOUNT_OF_CHECKPOINTS: usize = 30;

pub fn should_save(request: Option<Res<SaveRequest>>) -> bool {
    request.is_some()
}

pub fn remove_save_request(world: &mut World) {
    world.remove_resource::<SaveRequest>().unwrap();
}

pub fn save_ron() -> SystemConfig {
    save::<With<Save>>
        .pipe(save_ron_as_checkpoint)
        .pipe(save::finish)
        .in_set(SaveSet::Save)
}

pub fn save_ron_as_checkpoint(
    In(saved): In<Saved>,
    type_registry: Res<AppTypeRegistry>,
    mut state: ResMut<AppState>,
) -> Result<Saved, save::Error> {
    let input = saved.scene.serialize_ron(&type_registry)?;
    let re = Regex::new(r"generation: (\d+)").unwrap();
    // lol
    let input = re.replace_all(&input, "generation: 0").to_string();
    let ron = general_purpose::STANDARD.encode(input);
    if state.checkpoints.len() > MAX_AMOUNT_OF_CHECKPOINTS {
        state.checkpoints.pop_front();
    }
    state.checkpoints.push_back(ron);
    Ok(saved)
}

pub fn post_save(
    images: Res<Assets<Image>>,
    rec: Query<(&Rectangle, &UiImage), With<Rectangle>>,
    arrows: Query<&ArrowMeta, With<ArrowMeta>>,
    request: Res<SaveRequest>,
    mut state: ResMut<AppState>,
) {
    eprintln!("post save: {:?}", request);
    let ron = state.checkpoints.pop_back().unwrap();
    let mut json = json!({
        "bevy_version": "0.10",
        "images": {},
        "arrows": [],
        "ron": ron,
    });
    let json_images = json["images"].as_object_mut().unwrap();
    for (rect, image) in rec.iter() {
        if let Some(image) = images.get(&image.texture) {
            if let Ok(img) = image.clone().try_into_dynamic() {
                let mut image_data: Vec<u8> = Vec::new();
                #[cfg(not(target_arch = "wasm32"))]
                img.write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
                    .unwrap();
                let res_base64 = general_purpose::STANDARD.encode(image_data);
                json_images.insert(rect.id.0.to_string(), json!(res_base64));
            }
        }
    }

    let json_arrows = json["arrows"].as_array_mut().unwrap();
    for arrow_meta in arrows.iter() {
        json_arrows.push(json!(arrow_meta));
    }

    if let Some(path) = request.path.clone() {
        std::fs::write(path, json.to_string()).expect("Error saving state to file")
    } else {
        state.checkpoints.push_back(json.to_string());
    }
}
