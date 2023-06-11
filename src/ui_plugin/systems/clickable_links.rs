use bevy::{prelude::*, window::PrimaryWindow};
use bevy_cosmic_edit::{get_node_cursor_pos, get_x_offset, get_y_offset, CosmicEdit};
use cosmic_text::Edit;

use super::{ui_helpers::BevyMarkdownView, UiState, VeloNode};

pub fn clickable_links(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut markdown_text_query: Query<
        (&Node, &GlobalTransform, &mut CosmicEdit, &BevyMarkdownView),
        With<BevyMarkdownView>,
    >,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<VeloNode>)>,
    ui_state: Res<UiState>,
) {
    if ui_state.hold_entity.is_some() {
        return;
    }
    let primary_window = windows.iter_mut().next().unwrap();
    let scale_factor = primary_window.scale_factor() as f32;
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Clicked {
            for (node, transform, cosmic_edit, bevy_markdown_view) in markdown_text_query.iter_mut()
            {
                let size = (node.size().x, node.size().y);
                if let Some(pos) =
                    get_node_cursor_pos(&primary_window, transform, size, cosmic_edit.is_ui_node)
                {
                    let font_size = cosmic_edit.editor.buffer().metrics().font_size;
                    let line_height = cosmic_edit.editor.buffer().metrics().line_height;
                    let y_start = get_y_offset(&cosmic_edit.editor) as f32;
                    let x_start = get_x_offset(&cosmic_edit.editor) as f32;
                    for layout_runs in cosmic_edit.editor.buffer().layout_runs() {
                        let line_offset =
                            (y_start + (layout_runs.line_y - font_size)) / scale_factor;
                        if pos.1 < (line_offset + line_height / scale_factor) && pos.1 > line_offset
                        {
                            for glyph in layout_runs.glyphs {
                                let start = (x_start + glyph.x) / scale_factor;
                                let end = (x_start + glyph.x + glyph.w) / scale_factor;
                                if pos.0 > start && pos.0 < end {
                                    let idx = glyph.metadata;
                                    let text_span =
                                        bevy_markdown_view.span_metadata.get(idx).unwrap();
                                    if let Some(link) = text_span.link.clone() {
                                        #[cfg(not(target_arch = "wasm32"))]
                                        open::that(link.clone()).unwrap();
                                        #[cfg(target_arch = "wasm32")]
                                        open_url_in_new_tab(link.clone().as_str()).unwrap();
                                    }
                                }
                            }
                        }
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
