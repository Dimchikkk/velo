use bevy::{
    prelude::*,
    text::{PositionedGlyph, TextLayoutInfo},
    window::PrimaryWindow,
};
use bevy_markdown::BevyMarkdownNode;

use super::{get_sections, EditableText, UiState, VeloNode};

pub fn clickable_links(
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut editable_text_query: Query<
        (&Node, &GlobalTransform, &mut Text, &TextLayoutInfo),
        (With<EditableText>, Without<BevyMarkdownNode>),
    >,
    mut markdown_text_query: Query<
        (
            &Node,
            &GlobalTransform,
            &mut Text,
            &TextLayoutInfo,
            &BevyMarkdownNode,
        ),
        (With<BevyMarkdownNode>, Without<EditableText>),
    >,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<VeloNode>)>,
    mut ui_state: ResMut<UiState>,
) {
    if ui_state.hold_entity.is_some() {
        return;
    }
    let mut primary_window = primary_window.iter_mut().next().unwrap();
    let scale_factor = primary_window.scale_factor() as f32;

    if let Some(cursor_position) = primary_window.cursor_position() {
        let window_height = primary_window.height();
        let pos = Vec2::new(cursor_position.x, window_height - cursor_position.y);
        for (node, transform, text, text_layout_info) in editable_text_query.iter_mut() {
            let mut str = "".to_string();
            let mut text_copy = text.clone();
            for section in text_copy.sections.iter_mut() {
                str = format!("{}{}", str, section.value.clone());
            }
            let sections = get_sections(str);

            let offset = transform.translation().truncate() - 0.5 * node.size();
            for PositionedGlyph {
                position,
                section_index,
                size,
                ..
            } in &text_layout_info.glyphs
            {
                let rect = bevy::math::Rect::from_center_size(
                    offset + *position / scale_factor,
                    *size / scale_factor,
                );
                if rect.contains(pos) {
                    if sections.1[*section_index] {
                        primary_window.cursor.icon = CursorIcon::Hand;
                        for interaction in &mut interaction_query {
                            if *interaction == Interaction::Clicked {
                                #[cfg(not(target_arch = "wasm32"))]
                                open::that(text.sections[*section_index].value.clone()).unwrap();
                                #[cfg(target_arch = "wasm32")]
                                open_url_in_new_tab(&text.sections[*section_index].value).unwrap();
                            }
                        }
                    } else {
                        primary_window.cursor.icon = CursorIcon::Default;
                    }
                }
            }
        }
        for (node, transform, text, text_layout_info, markdown_text) in
            markdown_text_query.iter_mut()
        {
            let mut str = "".to_string();
            let mut text_copy = text.clone();
            for section in text_copy.sections.iter_mut() {
                str = format!("{}{}", str, section.value.clone());
            }
            let link_sections = markdown_text.link_sections.clone();

            let offset = transform.translation().truncate() - 0.5 * node.size();
            for PositionedGlyph {
                position,
                section_index,
                size,
                ..
            } in &text_layout_info.glyphs
            {
                let rect = bevy::math::Rect::from_center_size(
                    offset + *position / scale_factor,
                    *size / scale_factor,
                );
                if rect.contains(pos) {
                    if let Some(link) = link_sections[*section_index].clone() {
                        primary_window.cursor.icon = CursorIcon::Hand;
                        for interaction in &mut interaction_query {
                            if *interaction == Interaction::Clicked {
                                #[cfg(not(target_arch = "wasm32"))]
                                open::that(link.clone()).unwrap();
                                #[cfg(target_arch = "wasm32")]
                                open_url_in_new_tab(link.clone().as_str()).unwrap();
                                ui_state.entity_to_edit = None;
                            }
                        }
                    } else {
                        primary_window.cursor.icon = CursorIcon::Default;
                    }
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn open_url_in_new_tab(url: &str) -> Result<(), wasm_bindgen::prelude::JsValue> {
    use wasm_bindgen::prelude::*;
    use web_sys::window;

    let window = window().ok_or_else(|| JsValue::from_str("Failed to get window object"))?;
    let new_window: Option<web_sys::Window> = window.open_with_url_and_target(url, "_blank")?;
    new_window.unwrap().focus()?;
    Ok(())
}
