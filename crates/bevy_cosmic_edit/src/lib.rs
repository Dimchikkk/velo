use std::{collections::VecDeque, path::PathBuf, time::Duration};
#[path = "utils.rs"]
pub mod utils;
pub use utils::*;

use bevy::{
    asset::HandleId,
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::Extent3d,
    window::{PrimaryWindow, WindowScaleFactorChanged},
};
use cosmic_text::{
    Action, AttrsList, AttrsOwned, Buffer, BufferLine, Cursor, Edit, Editor, FontSystem, Metrics,
    Shaping, SwashCache,
};
use image::{imageops::FilterType, GenericImageView};

#[derive(Clone)]
pub struct CosmicEditUi;

#[derive(Clone)]
pub struct CosmicEditSprite {
    pub transform: Transform,
}

#[derive(Clone)]
pub enum CosmicNode {
    Ui,
    Sprite(CosmicEditSprite),
}

#[derive(Clone)]
pub enum CosmicText {
    OneStyle(String),
    MultiStyle(Vec<Vec<(String, cosmic_text::AttrsOwned)>>),
}

#[derive(Clone)]
pub struct CosmicMetrics {
    pub font_size: f32,
    pub line_height: f32,
    pub scale_factor: f32,
}

/// Contains metadata for spawning cosmic edit, including text content, position, size, and style.
#[derive(Clone)]
pub struct CosmicEditMeta {
    pub text: CosmicText,
    pub text_pos: CosmicTextPos,
    pub attrs: cosmic_text::AttrsOwned,
    pub metrics: CosmicMetrics,
    pub font_system_handle: Handle<CosmicFont>,
    pub size: Option<(f32, f32)>, // None used for bevy-ui nodes to use parent size
    pub node: CosmicNode,
    pub bg: bevy::prelude::Color,
    pub bg_image: Option<Handle<Image>>,
    pub readonly: bool,
}

/// Enum representing the position of the cosmic text.
#[derive(Clone)]
pub enum CosmicTextPos {
    Center,
    TopLeft,
}

#[derive(Clone)]
pub struct EditHistoryItem {
    pub cursor: Cursor,
    pub lines: Vec<Vec<(String, AttrsOwned)>>,
}

#[derive(Component)]
pub struct CosmicEditHistory {
    pub edits: VecDeque<EditHistoryItem>,
    pub current_edit: usize,
}

#[derive(Component)]
pub struct CosmicEdit {
    pub editor: Editor,
    pub font_system: Handle<CosmicFont>,
    pub attrs: cosmic_text::AttrsOwned,
    pub text_pos: CosmicTextPos,
    pub bg: bevy::prelude::Color,
    pub bg_image: Option<Handle<Image>>,
    pub width: f32,
    pub height: f32,
    pub font_size: f32,
    pub line_height: f32,
    pub readonly: bool,
    pub is_ui_node: bool,
}

#[derive(TypeUuid, TypePath)]
#[uuid = "DC6A0357-7941-4ADE-9332-24EA87E38961"]
pub struct CosmicFont(pub FontSystem);

/// Plugin struct that adds systems and initializes resources related to cosmic edit functionality.
pub struct CosmicEditPlugin;

impl Plugin for CosmicEditPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                cosmic_edit_bevy_events,
                cosmic_edit_set_redraw,
                on_scale_factor_change,
                cosmic_edit_redraw_buffer_ui
                    .before(cosmic_edit_set_redraw)
                    .before(on_scale_factor_change),
                cosmic_edit_redraw_buffer.before(on_scale_factor_change),
            ),
        )
        .init_resource::<ActiveEditor>()
        .add_asset::<CosmicFont>()
        .insert_resource(SwashCacheState {
            swash_cache: SwashCache::new(),
        });
    }
}

/// Resource struct that keeps track of the currently active editor entity.
#[derive(Resource, Default)]
pub struct ActiveEditor {
    pub entity: Option<Entity>,
}

/// Resource struct that holds configuration options for cosmic fonts.
#[derive(Resource, Default)]
pub struct CosmicFontConfig {
    pub fonts_dir_path: Option<PathBuf>,
    pub font_bytes: Option<Vec<&'static [u8]>>,
    pub load_system_fonts: bool, // caution: this can be relatively slow
}

#[derive(Resource)]
struct SwashCacheState {
    swash_cache: SwashCache,
}

pub fn create_cosmic_font_system(cosmic_font_config: CosmicFontConfig) -> FontSystem {
    let locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en-US"));
    let mut db = cosmic_text::fontdb::Database::new();
    if let Some(dir_path) = cosmic_font_config.fonts_dir_path.clone() {
        db.load_fonts_dir(dir_path);
    }
    if let Some(custom_font_data) = &cosmic_font_config.font_bytes {
        for elem in custom_font_data {
            db.load_font_data(elem.to_vec());
        }
    }
    if cosmic_font_config.load_system_fonts {
        db.load_system_fonts();
    }
    cosmic_text::FontSystem::new_with_locale_and_db(locale, db)
}

fn on_scale_factor_change(
    mut scale_factor_changed: EventReader<WindowScaleFactorChanged>,
    mut cosmic_edit_query: Query<&mut CosmicEdit, With<CosmicEdit>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
) {
    if !scale_factor_changed.is_empty() {
        let new_scale_factor = scale_factor_changed.iter().last().unwrap().scale_factor as f32;
        for mut cosmic_edit in &mut cosmic_edit_query.iter_mut() {
            if let Some(font_system) = cosmic_fonts.get_mut(&cosmic_edit.font_system) {
                let font_system = &mut font_system.0;
                let metrics = Metrics::new(cosmic_edit.font_size, cosmic_edit.line_height)
                    .scale(new_scale_factor);
                cosmic_edit
                    .editor
                    .buffer_mut()
                    .set_metrics(font_system, metrics);
                cosmic_edit.editor.buffer_mut().set_redraw(true);
            }
        }
    }
}

pub fn get_node_cursor_pos(
    window: &Window,
    node_transform: &GlobalTransform,
    size: (f32, f32),
    is_ui_node: bool,
) -> Option<(f32, f32)> {
    let (x_min, y_min, x_max, y_max) = match is_ui_node {
        true => (
            node_transform.affine().translation.x - size.0 / 2.,
            node_transform.affine().translation.y - size.1 / 2.,
            node_transform.affine().translation.x + size.0 / 2.,
            node_transform.affine().translation.y + size.1 / 2.,
        ),
        false => (
            node_transform.affine().translation.x - size.0 / 2.,
            -node_transform.affine().translation.y - size.1 / 2.,
            node_transform.affine().translation.x + size.0 / 2.,
            -node_transform.affine().translation.y + size.1 / 2.,
        ),
    };

    window.cursor_position().and_then(|mut pos| {
        if !is_ui_node {
            pos = Vec2::new(pos.x - window.width() / 2., pos.y - window.height() / 2.);
        }
        if x_min < pos.x && pos.x < x_max && y_min < pos.y && pos.y < y_max {
            Some((pos.x - x_min, pos.y - y_min))
        } else {
            None
        }
    })
}

/// Retrieves the cosmic text content from an editor.
///
/// # Arguments
///
/// * `editor` - A reference to the `Editor` instance containing the text content.
///
/// # Returns
///
/// A `String` containing the cosmic text content.
///
/// # Examples
///
/// ```
/// let editor = Editor::new();
/// let cosmic_text = get_cosmic_text(&editor);
/// println!("Cosmic text: {}", cosmic_text);
/// ```
pub fn get_cosmic_text(buffer: &Buffer) -> String {
    let mut text = String::new();
    let line_count = buffer.lines.len();

    for (i, line) in buffer.lines.iter().enumerate() {
        text.push_str(line.text());

        if i < line_count - 1 {
            text.push('\n');
        }
    }

    text
}

pub fn get_text_spans(
    buffer: &Buffer,
    default_attrs: AttrsOwned,
) -> Vec<Vec<(String, AttrsOwned)>> {
    let mut spans = Vec::new();
    for line in buffer.lines.iter() {
        let mut line_spans = Vec::new();
        let line_text = line.text();
        let line_attrs = line.attrs_list();
        if line_attrs.spans().is_empty() {
            line_spans.push((line_text.to_string(), default_attrs.clone()));
        } else {
            let mut current_pos = 0;
            for span in line_attrs.spans() {
                let span_range = span.0;
                let span_attrs = span.1.clone();
                let start_index = span_range.start;
                let end_index = span_range.end;
                if start_index > current_pos {
                    // Add the text between the current position and the start of the span
                    let non_span_text = line_text[current_pos..start_index].to_string();
                    line_spans.push((non_span_text, default_attrs.clone()));
                }
                let span_text = line_text[start_index..end_index].to_string();
                line_spans.push((span_text.clone(), span_attrs));
                current_pos = end_index;
            }
            if current_pos < line_text.len() {
                // Add the remaining text after the last span
                let remaining_text = line_text[current_pos..].to_string();
                line_spans.push((remaining_text, default_attrs.clone()));
            }
        }
        spans.push(line_spans);
    }
    spans
}

fn save_edit_history(cosmic_edit: &mut CosmicEdit, edit_history: &mut CosmicEditHistory) {
    let edits = &edit_history.edits;
    let current_lines = get_text_spans(cosmic_edit.editor.buffer(), cosmic_edit.attrs.clone());
    let current_edit = edit_history.current_edit;
    let mut new_edits = VecDeque::new();
    new_edits.extend(edits.iter().take(current_edit + 1).cloned());
    // remove old edits
    if new_edits.len() > 1000 {
        new_edits.drain(0..100);
    }
    new_edits.push_back(EditHistoryItem {
        cursor: cosmic_edit.editor.cursor(),
        lines: current_lines,
    });
    let len = new_edits.len();
    *edit_history = CosmicEditHistory {
        edits: new_edits,
        current_edit: len - 1,
    };
}

fn bevy_color_to_cosmic(color: bevy::prelude::Color) -> cosmic_text::Color {
    cosmic_text::Color::rgba(
        (color.r() * 255.) as u8,
        (color.g() * 255.) as u8,
        (color.b() * 255.) as u8,
        (color.a() * 255.) as u8,
    )
}

fn get_text_size(buffer: &Buffer) -> (f32, f32) {
    let width = buffer.layout_runs().map(|run| run.line_w).reduce(f32::max);
    let height = buffer.layout_runs().count() as f32 * buffer.metrics().line_height;
    if width.is_none() || height == 0. {
        return (1., 1.);
    }
    (width.unwrap(), height)
}

pub fn get_y_offset(buffer: &Buffer) -> i32 {
    let (_, text_height) = get_text_size(buffer);
    ((buffer.size().1 - text_height) / 2.0) as i32
}

pub fn get_x_offset(buffer: &Buffer) -> i32 {
    let (text_width, _) = get_text_size(buffer);
    ((buffer.size().0 - text_width) / 2.0) as i32
}

pub fn cosmic_edit_bevy_events(
    windows: Query<&Window, With<PrimaryWindow>>,
    active_editor: Res<ActiveEditor>,
    keys: Res<Input<KeyCode>>,
    mut char_evr: EventReader<ReceivedCharacter>,
    buttons: Res<Input<MouseButton>>,
    mut cosmic_edit_query: Query<
        (
            &mut CosmicEdit,
            &mut CosmicEditHistory,
            &GlobalTransform,
            Entity,
        ),
        With<CosmicEdit>,
    >,
    mut is_deleting: Local<bool>,
    mut font_system_assets: ResMut<Assets<CosmicFont>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut edits_duration: Local<Option<Duration>>,
    mut undoredo_duration: Local<Option<Duration>>,
) {
    let primary_window = windows.single();
    let scale_factor = primary_window.scale_factor() as f32;
    for (mut cosmic_edit, mut edit_history, node_transform, entity) in
        &mut cosmic_edit_query.iter_mut()
    {
        if active_editor.entity == Some(entity) {
            if let Some(font_system) = font_system_assets.get_mut(&cosmic_edit.font_system) {
                let now_ms = get_timestamp();
                let command = keys.any_pressed([KeyCode::SuperLeft, KeyCode::SuperRight]);
                let shift = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
                let option = keys.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]);
                if keys.just_pressed(KeyCode::Left) {
                    cosmic_edit.editor.action(&mut font_system.0, Action::Left);
                }
                if keys.just_pressed(KeyCode::Right) {
                    cosmic_edit.editor.action(&mut font_system.0, Action::Right);
                }
                if keys.just_pressed(KeyCode::Up) {
                    cosmic_edit.editor.action(&mut font_system.0, Action::Up);
                }
                if keys.just_pressed(KeyCode::Down) {
                    cosmic_edit.editor.action(&mut font_system.0, Action::Down);
                }
                if !cosmic_edit.readonly && keys.just_pressed(KeyCode::Back) {
                    #[cfg(target_arch = "wasm32")]
                    cosmic_edit
                        .editor
                        .action(&mut font_system.0, Action::Backspace);
                    *is_deleting = true;
                }
                if !cosmic_edit.readonly && keys.just_released(KeyCode::Back) {
                    *is_deleting = false;
                }
                if !cosmic_edit.readonly && keys.just_pressed(KeyCode::Delete) {
                    cosmic_edit
                        .editor
                        .action(&mut font_system.0, Action::Delete);
                }
                if keys.just_pressed(KeyCode::Escape) {
                    cosmic_edit
                        .editor
                        .action(&mut font_system.0, Action::Escape);
                }
                if command && keys.just_pressed(KeyCode::A) {
                    cosmic_edit
                        .editor
                        .action(&mut font_system.0, Action::BufferEnd);
                    let current_cursor = cosmic_edit.editor.cursor();
                    cosmic_edit.editor.set_select_opt(Some(Cursor {
                        line: 0,
                        index: 0,
                        affinity: current_cursor.affinity,
                        color: current_cursor.color,
                    }));
                    // RETURN
                    return;
                }
                if command && option && keys.just_pressed(KeyCode::Left) {
                    cosmic_edit
                        .editor
                        .action(&mut font_system.0, Action::PreviousWord);
                    // RETURN
                    return;
                }
                if command && option && keys.just_pressed(KeyCode::Right) {
                    cosmic_edit
                        .editor
                        .action(&mut font_system.0, Action::NextWord);
                    // RETURN
                    return;
                }

                // redo
                if !cosmic_edit.readonly && command && shift && keys.just_pressed(KeyCode::Z) {
                    let edits = &edit_history.edits;
                    if edits.is_empty() {
                        // RETURN
                        return;
                    }
                    if edit_history.current_edit == edits.len() - 1 {
                        // RETURN
                        return;
                    }
                    let idx = edit_history.current_edit + 1;
                    if let Some(current_edit) = edits.get(idx) {
                        cosmic_edit.editor.buffer_mut().lines.clear();
                        for line in current_edit.lines.iter() {
                            let mut line_text = String::new();
                            let mut attrs_list = AttrsList::new(cosmic_edit.attrs.as_attrs());
                            for (text, attrs) in line.iter() {
                                let start = line_text.len();
                                line_text.push_str(text);
                                let end = line_text.len();
                                attrs_list.add_span(start..end, attrs.as_attrs());
                            }
                            cosmic_edit.editor.buffer_mut().lines.push(BufferLine::new(
                                line_text,
                                attrs_list,
                                Shaping::Advanced,
                            ));
                        }
                        cosmic_edit.editor.set_cursor(current_edit.cursor);
                        cosmic_edit.editor.buffer_mut().set_redraw(true);
                        edit_history.current_edit += 1;
                    }
                    *undoredo_duration = Some(Duration::from_millis(now_ms as u64));
                    // RETURN
                    return;
                }
                // undo
                if !cosmic_edit.readonly && command && keys.just_pressed(KeyCode::Z) {
                    let edits = &edit_history.edits;
                    if edits.is_empty() {
                        // RETURN
                        return;
                    }
                    if edit_history.current_edit <= 1 {
                        // RETURN
                        return;
                    }
                    let idx = edit_history.current_edit - 1;
                    if let Some(current_edit) = edits.get(idx) {
                        cosmic_edit.editor.buffer_mut().lines.clear();
                        for line in current_edit.lines.iter() {
                            let mut line_text = String::new();
                            let mut attrs_list = AttrsList::new(cosmic_edit.attrs.as_attrs());
                            for (text, attrs) in line.iter() {
                                let start = line_text.len();
                                line_text.push_str(text);
                                let end = line_text.len();
                                attrs_list.add_span(start..end, attrs.as_attrs());
                            }
                            cosmic_edit.editor.buffer_mut().lines.push(BufferLine::new(
                                line_text,
                                attrs_list,
                                Shaping::Advanced,
                            ));
                        }
                        cosmic_edit.editor.set_cursor(current_edit.cursor);
                        cosmic_edit.editor.buffer_mut().set_redraw(true);
                        edit_history.current_edit -= 1;
                    }
                    *undoredo_duration = Some(Duration::from_millis(now_ms as u64));
                    // RETURN
                    return;
                }

                let mut is_clipboard = false;
                #[cfg(not(target_arch = "wasm32"))]
                {
                    if let Ok(mut clipboard) = arboard::Clipboard::new() {
                        if command && keys.just_pressed(KeyCode::C) {
                            if let Some(text) = cosmic_edit.editor.copy_selection() {
                                clipboard.set_text(text).unwrap();
                                // RETURN
                                return;
                            }
                        }
                        if !cosmic_edit.readonly && command && keys.just_pressed(KeyCode::X) {
                            if let Some(text) = cosmic_edit.editor.copy_selection() {
                                clipboard.set_text(text).unwrap();
                                cosmic_edit.editor.delete_selection();
                            }
                            is_clipboard = true;
                        }
                        if !cosmic_edit.readonly && command && keys.just_pressed(KeyCode::V) {
                            if let Ok(text) = clipboard.get_text() {
                                for c in text.chars() {
                                    cosmic_edit
                                        .editor
                                        .action(&mut font_system.0, Action::Insert(c));
                                }
                            }
                            is_clipboard = true;
                        }
                    }
                }
                let (offset_x, offset_y) = match cosmic_edit.text_pos {
                    CosmicTextPos::Center => (
                        get_x_offset(cosmic_edit.editor.buffer()),
                        get_y_offset(cosmic_edit.editor.buffer()),
                    ),
                    CosmicTextPos::TopLeft => (0, 0),
                };
                if buttons.just_pressed(MouseButton::Left) {
                    if let Some(node_cursor_pos) = get_node_cursor_pos(
                        primary_window,
                        node_transform,
                        (cosmic_edit.width, cosmic_edit.height),
                        cosmic_edit.is_ui_node,
                    ) {
                        cosmic_edit.editor.action(
                            &mut font_system.0,
                            Action::Click {
                                x: (node_cursor_pos.0 * scale_factor) as i32 - offset_x,
                                y: (node_cursor_pos.1 * scale_factor) as i32 - offset_y,
                            },
                        );
                    }
                    // RETURN
                    return;
                }
                if buttons.pressed(MouseButton::Left) {
                    if let Some(node_cursor_pos) = get_node_cursor_pos(
                        primary_window,
                        node_transform,
                        (cosmic_edit.width, cosmic_edit.height),
                        cosmic_edit.is_ui_node,
                    ) {
                        cosmic_edit.editor.action(
                            &mut font_system.0,
                            Action::Drag {
                                x: (node_cursor_pos.0 * scale_factor) as i32 - offset_x,
                                y: (node_cursor_pos.1 * scale_factor) as i32 - offset_y,
                            },
                        );
                    }
                    // RETURN
                    return;
                }
                for ev in scroll_evr.iter() {
                    match ev.unit {
                        MouseScrollUnit::Line => {
                            cosmic_edit.editor.action(
                                &mut font_system.0,
                                Action::Scroll {
                                    lines: -ev.y as i32,
                                },
                            );
                        }
                        MouseScrollUnit::Pixel => {
                            let line_height = cosmic_edit.editor.buffer().metrics().line_height;
                            cosmic_edit.editor.action(
                                &mut font_system.0,
                                Action::Scroll {
                                    lines: -(ev.y / line_height) as i32,
                                },
                            );
                        }
                    }
                }

                if cosmic_edit.readonly {
                    // RETURN
                    return;
                }

                let mut is_edit = is_clipboard;
                let mut is_return = false;
                if keys.just_pressed(KeyCode::Return) {
                    is_return = true;
                    is_edit = true;
                    // to have new line on wasm rather than E
                    cosmic_edit
                        .editor
                        .action(&mut font_system.0, Action::Insert('\n'));
                }

                if !(is_clipboard || is_return) {
                    for char_ev in char_evr.iter() {
                        is_edit = true;
                        if *is_deleting {
                            cosmic_edit
                                .editor
                                .action(&mut font_system.0, Action::Backspace);
                        } else {
                            cosmic_edit
                                .editor
                                .action(&mut font_system.0, Action::Insert(char_ev.char));
                        }
                    }
                }

                if !is_edit {
                    // RETURN
                    return;
                }

                if let Some(last_edit_duration) = *edits_duration {
                    if Duration::from_millis(now_ms as u64) - last_edit_duration
                        > Duration::from_millis(150)
                    {
                        save_edit_history(&mut cosmic_edit, &mut edit_history);
                        *edits_duration = Some(Duration::from_millis(now_ms as u64));
                    }
                } else {
                    save_edit_history(&mut cosmic_edit, &mut edit_history);
                    *edits_duration = Some(Duration::from_millis(now_ms as u64));
                }
            }
        }
    }
}

fn cosmic_edit_set_redraw(mut cosmic_edit_query: Query<&mut CosmicEdit, Added<CosmicEdit>>) {
    for mut cosmic_edit in cosmic_edit_query.iter_mut() {
        cosmic_edit.editor.buffer_mut().set_redraw(true);
    }
}

fn redraw_buffer_common(
    images: &mut ResMut<Assets<Image>>,
    swash_cache_state: &mut ResMut<SwashCacheState>,
    cosmic_edit: &mut CosmicEdit,
    img_handle: &mut Handle<Image>,
    font_system_assets: &mut ResMut<Assets<CosmicFont>>,
    scale_factor: f32,
    original_width: f32,
    original_height: f32,
) {
    let width = original_width * scale_factor;
    let height = original_height * scale_factor;
    let swash_cache = &mut swash_cache_state.swash_cache;
    if let Some(font_system) = font_system_assets.get_mut(&cosmic_edit.font_system) {
        cosmic_edit.editor.shape_as_needed(&mut font_system.0);
        if cosmic_edit.editor.buffer().redraw() {
            cosmic_edit
                .editor
                .buffer_mut()
                .set_size(&mut font_system.0, width, height);
            let font_color = cosmic_text::Color::rgb(0, 0, 0);

            let mut pixels = vec![0; width as usize * height as usize * 4];
            if let Some(bg_image) = cosmic_edit.bg_image.clone() {
                let image = images.get(&bg_image).unwrap();

                let mut dynamic_image = image.clone().try_into_dynamic().unwrap();
                if image.size().x != width || image.size().y != height {
                    dynamic_image = dynamic_image.resize_to_fill(
                        width as u32,
                        height as u32,
                        FilterType::Triangle,
                    );
                }
                for (i, (_, _, rgba)) in dynamic_image.pixels().enumerate() {
                    if let Some(p) = pixels.get_mut(i * 4..(i + 1) * 4) {
                        p[0] = rgba[0];
                        p[1] = rgba[1];
                        p[2] = rgba[2];
                        p[3] = rgba[3];
                    }
                }
            } else {
                let bg = cosmic_edit.bg;
                for pixel in pixels.chunks_exact_mut(4) {
                    pixel[0] = (bg.r() * 255.) as u8; // Red component
                    pixel[1] = (bg.g() * 255.) as u8; // Green component
                    pixel[2] = (bg.b() * 255.) as u8; // Blue component
                    pixel[3] = (bg.a() * 255.) as u8; // Alpha component
                }
            }

            let (offset_y, offset_x) = match cosmic_edit.text_pos {
                CosmicTextPos::Center => (
                    get_y_offset(cosmic_edit.editor.buffer()),
                    get_x_offset(cosmic_edit.editor.buffer()),
                ),
                CosmicTextPos::TopLeft => (0, 0),
            };
            cosmic_edit.editor.draw(
                &mut font_system.0,
                swash_cache,
                font_color,
                |x, y, w, h, color| {
                    for row in 0..h as i32 {
                        for col in 0..w as i32 {
                            draw_pixel(
                                &mut pixels,
                                width as i32,
                                height as i32,
                                x + col + offset_x,
                                y + row + offset_y,
                                color,
                            );
                        }
                    }
                },
            );
            cosmic_edit.editor.buffer_mut().set_redraw(false);

            if let Some(prev_image) = images.get_mut(img_handle) {
                if *img_handle == bevy::render::texture::DEFAULT_IMAGE_HANDLE.typed() {
                    let mut prev_image = prev_image.clone();
                    prev_image.data.clear();
                    prev_image.data.extend_from_slice(pixels.as_slice());
                    prev_image.resize(Extent3d {
                        width: width as u32,
                        height: height as u32,
                        depth_or_array_layers: 1,
                    });
                    let handle_id: HandleId = HandleId::random::<Image>();
                    let new_handle: Handle<Image> = Handle::weak(handle_id);
                    let new_handle = images.set(new_handle, prev_image);
                    *img_handle = new_handle;
                } else {
                    prev_image.data.clear();
                    prev_image.data.extend_from_slice(pixels.as_slice());
                    prev_image.resize(Extent3d {
                        width: width as u32,
                        height: height as u32,
                        depth_or_array_layers: 1,
                    });
                }
            }
        }
    }
}

fn cosmic_edit_redraw_buffer_ui(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut images: ResMut<Assets<Image>>,
    mut swash_cache_state: ResMut<SwashCacheState>,
    mut cosmic_edit_query: Query<
        (&mut CosmicEdit, &mut UiImage, &Node, &mut Visibility),
        With<CosmicEdit>,
    >,
    mut font_system_assets: ResMut<Assets<CosmicFont>>,
) {
    let primary_window = windows.single();
    for (mut cosmic_edit, mut img, node, mut visibility) in &mut cosmic_edit_query.iter_mut() {
        if node.size().x != 0. && node.size().y != 0. {
            cosmic_edit.width = node.size().x;
            cosmic_edit.height = node.size().y;
        }
        let width = cosmic_edit.width;
        let height = cosmic_edit.height;

        redraw_buffer_common(
            &mut images,
            &mut swash_cache_state,
            &mut cosmic_edit,
            &mut img.texture,
            &mut font_system_assets,
            primary_window.scale_factor() as f32,
            width,
            height,
        );

        if *visibility == Visibility::Hidden
            && img.texture.clone() != bevy::render::texture::DEFAULT_IMAGE_HANDLE.typed()
        {
            *visibility = Visibility::Visible;
        }
    }
}

fn cosmic_edit_redraw_buffer(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut images: ResMut<Assets<Image>>,
    mut swash_cache_state: ResMut<SwashCacheState>,
    mut cosmic_edit_query: Query<
        (&mut CosmicEdit, &mut Handle<Image>, &mut Visibility),
        With<CosmicEdit>,
    >,
    mut font_system_assets: ResMut<Assets<CosmicFont>>,
) {
    let primary_window = windows.single();
    for (mut cosmic_edit, mut handle, mut visibility) in &mut cosmic_edit_query.iter_mut() {
        let width = cosmic_edit.width;
        let height = cosmic_edit.height;
        redraw_buffer_common(
            &mut images,
            &mut swash_cache_state,
            &mut cosmic_edit,
            &mut handle,
            &mut font_system_assets,
            primary_window.scale_factor() as f32,
            width,
            height,
        );

        if *visibility == Visibility::Hidden
            && handle.clone() != bevy::render::texture::DEFAULT_IMAGE_HANDLE.typed()
        {
            *visibility = Visibility::Visible;
        }
    }
}

pub fn cosmic_edit_set_text(
    text: CosmicText,
    attrs: AttrsOwned,
    editor: &mut Editor,
    font_system: &mut FontSystem,
) {
    editor.buffer_mut().lines.clear();
    match text {
        CosmicText::OneStyle(text) => {
            editor.buffer_mut().set_text(
                font_system,
                text.as_str(),
                attrs.as_attrs(),
                Shaping::Advanced,
            );
        }
        CosmicText::MultiStyle(lines) => {
            for line in lines {
                let mut line_text = String::new();
                let mut attrs_list = AttrsList::new(attrs.as_attrs());
                for (text, attrs) in line.iter() {
                    let start = line_text.len();
                    line_text.push_str(text);
                    let end = line_text.len();
                    attrs_list.add_span(start..end, attrs.as_attrs());
                }
                editor.buffer_mut().lines.push(BufferLine::new(
                    line_text,
                    attrs_list,
                    Shaping::Advanced,
                ));
            }
        }
    }
}

/// Spawns a cosmic edit entity with the provided configuration.
///
/// # Returns
///
/// The `Entity` identifier of the spawned cosmic edit entity.
pub fn spawn_cosmic_edit(
    commands: &mut Commands,
    cosmic_fonts: &mut ResMut<Assets<CosmicFont>>,
    cosmic_edit_meta: CosmicEditMeta,
) -> Entity {
    let font_system = cosmic_fonts
        .get_mut(&cosmic_edit_meta.font_system_handle)
        .unwrap();
    let scale_factor = cosmic_edit_meta.metrics.scale_factor;
    let font_size = cosmic_edit_meta.metrics.font_size;
    let line_height = cosmic_edit_meta.metrics.line_height;
    let metrics = Metrics::new(font_size, line_height).scale(scale_factor);
    let buffer = Buffer::new(&mut font_system.0, metrics);
    let mut editor = Editor::new(buffer);
    cosmic_edit_set_text(
        cosmic_edit_meta.text,
        cosmic_edit_meta.attrs.clone(),
        &mut editor,
        &mut font_system.0,
    );
    if let Some((width, height)) = cosmic_edit_meta.size {
        editor.buffer_mut().set_size(
            &mut font_system.0,
            width * scale_factor,
            height * scale_factor,
        );
    }
    let mut cursor = editor.cursor();
    if cosmic_edit_meta.readonly {
        cursor.color = Some(bevy_color_to_cosmic(cosmic_edit_meta.bg));
    }
    editor.set_cursor(cursor);

    let mut edits = VecDeque::new();
    if !cosmic_edit_meta.readonly {
        edits.push_back(EditHistoryItem {
            cursor,
            lines: get_text_spans(editor.buffer(), cosmic_edit_meta.attrs.clone()),
        });
    }
    let edit_history = CosmicEditHistory {
        edits,
        current_edit: 0,
    };
    let mut cosmic_edit_component = CosmicEdit {
        editor,
        font_system: cosmic_edit_meta.font_system_handle,
        text_pos: cosmic_edit_meta.text_pos,
        bg: cosmic_edit_meta.bg,
        is_ui_node: false,
        font_size,
        line_height,
        width: cosmic_edit_meta.size.unwrap_or((1., 1.)).0,
        height: cosmic_edit_meta.size.unwrap_or((1., 1.)).1,
        readonly: cosmic_edit_meta.readonly,
        attrs: cosmic_edit_meta.attrs.clone(),
        bg_image: cosmic_edit_meta.bg_image,
    };
    match cosmic_edit_meta.node {
        CosmicNode::Ui => {
            cosmic_edit_component.is_ui_node = true;
            let style = Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            };
            let button_bundle = ButtonBundle {
                visibility: Visibility::Hidden,
                focus_policy: bevy::ui::FocusPolicy::Pass,
                style,
                ..default()
            };
            commands
                .spawn((button_bundle, cosmic_edit_component, edit_history))
                .id()
        }
        CosmicNode::Sprite(sprite_node) => {
            let sprite = SpriteBundle {
                visibility: Visibility::Hidden,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(
                        cosmic_edit_component.width,
                        cosmic_edit_component.height,
                    )),
                    ..default()
                },
                transform: sprite_node.transform,
                ..default()
            };
            commands
                .spawn((sprite, cosmic_edit_component, edit_history))
                .id()
        }
    }
}

fn draw_pixel(
    buffer: &mut [u8],
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    color: cosmic_text::Color,
) {
    let alpha = (color.0 >> 24) & 0xFF;
    if alpha == 0 {
        // Do not draw if alpha is zero
        return;
    }

    if y < 0 || y >= height {
        // Skip if y out of bounds
        return;
    }

    if x < 0 || x >= width {
        // Skip if x out of bounds
        return;
    }

    let offset = (y as usize * width as usize + x as usize) * 4;

    let mut current = buffer[offset + 2] as u32
        | (buffer[offset + 1] as u32) << 8
        | (buffer[offset] as u32) << 16
        | (buffer[offset + 3] as u32) << 24;

    if alpha >= 255 || current == 0 {
        // Alpha is 100% or current is null, replace with no blending
        current = color.0;
    } else {
        // Alpha blend with current value
        let n_alpha = 255 - alpha;
        let rb = ((n_alpha * (current & 0x00FF00FF)) + (alpha * (color.0 & 0x00FF00FF))) >> 8;
        let ag = (n_alpha * ((current & 0xFF00FF00) >> 8))
            + (alpha * (0x01000000 | ((color.0 & 0x0000FF00) >> 8)));
        current = (rb & 0x00FF00FF) | (ag & 0xFF00FF00);
    }

    buffer[offset + 2] = current as u8;
    buffer[offset + 1] = (current >> 8) as u8;
    buffer[offset] = (current >> 16) as u8;
    buffer[offset + 3] = (current >> 24) as u8;
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use cosmic_text::{Attrs, AttrsOwned};

    use crate::*;

    fn test_spawn_cosmic_edit_system(
        mut commands: Commands,
        mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    ) {
        let cosmic_font_config = CosmicFontConfig {
            fonts_dir_path: None,
            font_bytes: None,
            load_system_fonts: true,
        };
        let font_system = create_cosmic_font_system(cosmic_font_config);
        let font_system_handle = cosmic_fonts.add(CosmicFont(font_system));
        let cosmic_edit_meta = CosmicEditMeta {
            text: CosmicText::OneStyle("Blah".to_string()),
            attrs: AttrsOwned::new(Attrs::new()),
            metrics: CosmicMetrics {
                font_size: 14.,
                line_height: 18.,
                scale_factor: 1.,
            },
            text_pos: CosmicTextPos::Center,
            font_system_handle,
            node: CosmicNode::Ui,
            size: None,
            bg: bevy::prelude::Color::NONE,
            readonly: false,
            bg_image: None,
        };
        spawn_cosmic_edit(&mut commands, &mut cosmic_fonts, cosmic_edit_meta);
    }

    #[test]
    fn test_spawn_cosmic_edit() {
        let mut app = App::new();
        app.add_plugins(TaskPoolPlugin::default());
        app.add_plugins(AssetPlugin::default());
        app.add_systems(Update, test_spawn_cosmic_edit_system);

        let input = Input::<KeyCode>::default();
        app.insert_resource(input);
        let mouse_input: Input<MouseButton> = Input::<MouseButton>::default();
        app.insert_resource(mouse_input);
        app.add_asset::<Image>();
        app.add_asset::<CosmicFont>();

        app.add_event::<ReceivedCharacter>();

        app.update();

        let mut text_nodes_query = app.world.query::<&CosmicEdit>();
        for node in text_nodes_query.iter(&app.world) {
            insta::assert_debug_snapshot!(node
                .editor
                .buffer()
                .lines
                .iter()
                .map(|line| line.text())
                .collect::<Vec<_>>());
        }
    }
}
