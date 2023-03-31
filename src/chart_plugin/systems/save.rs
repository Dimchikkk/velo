use base64::{engine::general_purpose, Engine};
use bevy::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use image::*;

use serde_json::json;
use std::io::Cursor;

use crate::{
    chart_plugin::ui_helpers::style_to_pos, AppState, JsonNode, JsonNodeText, SaveRequest,
};

use super::ui_helpers::{ArrowMeta, EditableText, Rectangle};

pub fn should_save(request: Option<Res<SaveRequest>>) -> bool {
    request.is_some()
}

pub fn remove_save_request(world: &mut World) {
    world.remove_resource::<SaveRequest>().unwrap();
}

const MAX_CHECKPOINTS: i32 = 10;

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
    let mut json = json!({
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
            text: JsonNodeText {
                text,
                pos: style_to_pos((style.justify_content, style.align_items)),
            },
            z_index,
            tags: vec![], // TODO
        }));
    }

    let json_arrows = json["arrows"].as_array_mut().unwrap();
    for arrow_meta in arrows.iter() {
        json_arrows.push(json!(arrow_meta));
    }

    for tab in &mut state.tabs {
        if request.tab_id.is_some() {
            if tab.id == request.tab_id.unwrap() {
                if (tab.checkpoints.len() as i32) > MAX_CHECKPOINTS {
                    tab.checkpoints.pop_front();
                }
                tab.checkpoints.push_back(json.to_string());
                break;
            }
        } else if tab.is_active {
            if (tab.checkpoints.len() as i32) > MAX_CHECKPOINTS {
                tab.checkpoints.pop_front();
            }
            tab.checkpoints.push_back(json.to_string());
            break;
        }
    }

    if let Some(path) = request.path.clone() {
        let json = json!({
            "version": "0.1.0",
            "tabs": json!(state.tabs),
        });
        std::fs::write(path, serde_json::to_string_pretty(&json).unwrap())
            .expect("Error saving state to file")
    }
}
