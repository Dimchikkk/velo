use base64::{engine::general_purpose, Engine};
use bevy::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use image::*;

use serde_json::json;
use std::io::Cursor;

use crate::{AppState, JsonNode, SaveRequest};

use super::ui_helpers::{ArrowMeta, EditableText, Rectangle};

pub fn should_save(request: Option<Res<SaveRequest>>) -> bool {
    request.is_some()
}

pub fn remove_save_request(world: &mut World) {
    world.remove_resource::<SaveRequest>().unwrap();
}

pub fn save_json(
    images: Res<Assets<Image>>,
    rec_query: Query<
        (
            &Rectangle,
            &UiImage,
            &BackgroundColor,
            &Style,
            &Children,
            &ZIndex,
        ),
        With<Rectangle>,
    >,
    arrows: Query<&ArrowMeta, With<ArrowMeta>>,
    request: Res<SaveRequest>,
    mut state: ResMut<AppState>,
    text_query: Query<&mut Text, With<EditableText>>,
) {
    eprintln!("save json: {:?}", request);
    let mut json = json!({
        "bevy_version": "0.10",
        "images": {},
        "nodes": [],
        "arrows": [],
    });
    let json_images = json["images"].as_object_mut().unwrap();
    for (rect, image, _, _, _, _) in rec_query.iter() {
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

    let json_nodes = json["nodes"].as_array_mut().unwrap();
    for (rect, _, bg_color, style, children, z_index) in rec_query.iter() {
        let text = text_query.get(children[children.len() - 1]).unwrap();
        let text = text.sections[0].value.clone();
        let left = style.position.left;
        let bottom = style.position.bottom;
        let size = style.size;
        let bg_color = bg_color.0;
        let z_index = match *z_index {
            ZIndex::Local(v) => v,
            _ => -1,
        };
        json_nodes.push(json!(JsonNode {
            node_type: crate::NodeType::Rect,
            id: rect.id.0,
            left,
            bottom,
            width: size.width,
            height: size.height,
            bg_color,
            text,
            z_index,
            tags: vec![],                     // TODO
            text_pos: crate::TextPos::Center  // TODO
        }));
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
