use std::cmp;

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    window::PrimaryWindow,
};
use cosmic_text::{
    Action, Affinity, Attrs, Buffer, Cursor, Edit, Editor, FontSystem, Metrics, SwashCache,
};
use image::{ImageBuffer, RgbaImage};

pub struct CosmicEditMeta<'a> {
    pub text: String,
    pub width: f32,
    pub height: f32,
    pub font_size: f32,
    pub line_height: f32,
    pub scale_factor: f32,
    pub font_system: &'a mut FontSystem,
    pub is_visible: bool,
    pub initial_background: Option<UiImage>,
}

#[derive(Component)]
pub struct CosmicEditImage {
    pub editor: Editor,
}
pub struct CosmicEditPlugin;

impl Plugin for CosmicEditPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init)
            .add_systems((cosmic_edit_bevy_events, cosmic_edit_redraw_buffer))
            .init_resource::<FontSystemState>()
            .init_resource::<SwashCacheState>()
            .init_resource::<ActiveEditor>();
    }
}

#[derive(Resource, Default)]
pub struct ActiveEditor {
    pub entity: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct FontSystemState {
    pub font_system: Option<FontSystem>,
}

#[derive(Resource, Default)]
struct SwashCacheState {
    swash_cache: Option<SwashCache>,
}

fn init(
    mut font_system_state: ResMut<FontSystemState>,
    mut swash_cache_state: ResMut<SwashCacheState>,
) {
    let locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en-US"));
    let mut db = cosmic_text::fontdb::Database::new();
    db.load_system_fonts();
    // TODO: figure out whether 'Source Code Pro' is valid value
    db.set_monospace_family("Source Code Pro");
    db.set_sans_serif_family("Source Code Pro");
    db.set_serif_family("Source Code Pro");
    let font_system = cosmic_text::FontSystem::new_with_locale_and_db(locale, db);
    font_system_state.font_system = Some(font_system);
    swash_cache_state.swash_cache = Some(SwashCache::new());
}

fn get_node_cursor_pos(
    window: &Window,
    node_transform: &GlobalTransform,
    node: &Node,
) -> Option<(f32, f32)> {
    let x_min = node_transform.affine().translation.x - node.size().x / 2.;
    let y_min = window.height() - node_transform.affine().translation.y - node.size().y / 2.;
    let x_max = x_min + node.size().x;
    let y_max = y_min + node.size().y;
    let node_cursor_pos = match window.cursor_position() {
        Some(pos) => {
            if x_min < pos.x && pos.x < x_max && y_min < pos.y && pos.y < y_max {
                Some((pos.x - x_min, y_max - pos.y))
            } else {
                None
            }
        }
        None => None,
    };
    node_cursor_pos
}

pub fn get_cosmic_text(editor: &Editor) -> String {
    let mut text = String::new();
    for line in editor.buffer().lines.iter() {
        text.push_str(line.text());
        text.push('\n');
    }
    text
}

fn get_y_offset(editor: &Editor) -> i32 {
    let text_height = editor.buffer().metrics().line_height
        * cmp::min(
            editor.buffer().visible_lines(),
            editor.buffer().lines.len() as i32,
        ) as f32;
    ((editor.buffer().size().1 - text_height) / 2.0) as i32
}

fn get_x_offset(editor: &Editor) -> i32 {
    let mut max_line_width = 0.;
    for line in editor.buffer().lines.iter() {
        if let Some(layout_opt) = line.layout_opt().as_ref() {
            for layout_line in layout_opt {
                if layout_line.w > max_line_width {
                    max_line_width = layout_line.w;
                }
            }
        }
    }
    ((editor.buffer().size().0
        - cmp::min(max_line_width as i32, editor.buffer().size().0 as i32) as f32)
        / 2.0) as i32
}

fn cosmic_edit_bevy_events(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut font_system_state: ResMut<FontSystemState>,
    active_editor: Res<ActiveEditor>,
    keys: Res<Input<KeyCode>>,
    mut char_evr: EventReader<ReceivedCharacter>,
    buttons: Res<Input<MouseButton>>,
    mut cosmic_edit_query: Query<
        (&mut CosmicEditImage, &GlobalTransform, &Node, Entity),
        With<CosmicEditImage>,
    >,
    mut is_deleting: Local<bool>,
) {
    let window = windows.single();
    let font_system = font_system_state.font_system.as_mut().unwrap();
    for (mut cosmic_edit, node_transform, node, entity) in &mut cosmic_edit_query.iter_mut() {
        if active_editor.entity == Some(entity) {
            let command = keys.any_pressed([KeyCode::RWin, KeyCode::LWin]);
            let option = keys.any_pressed([KeyCode::LAlt, KeyCode::RAlt]);
            if keys.just_pressed(KeyCode::Left) {
                cosmic_edit.editor.action(font_system, Action::Left);
            }
            if keys.just_pressed(KeyCode::Right) {
                cosmic_edit.editor.action(font_system, Action::Right);
            }
            if keys.just_pressed(KeyCode::Up) {
                cosmic_edit.editor.action(font_system, Action::Up);
            }
            if keys.just_pressed(KeyCode::Down) {
                cosmic_edit.editor.action(font_system, Action::Down);
            }
            if keys.just_pressed(KeyCode::Back) {
                *is_deleting = true;
            }
            if keys.just_released(KeyCode::Back) {
                *is_deleting = false;
            }
            if keys.just_pressed(KeyCode::Delete) {
                cosmic_edit.editor.action(font_system, Action::Delete);
            }
            if keys.just_pressed(KeyCode::Return) {
                cosmic_edit.editor.action(font_system, Action::Enter);
            }
            if keys.just_pressed(KeyCode::Escape) {
                cosmic_edit.editor.action(font_system, Action::Escape);
            }
            if command && keys.just_pressed(KeyCode::A) {
                cosmic_edit.editor.action(font_system, Action::BufferEnd);
                cosmic_edit.editor.set_select_opt(Some(Cursor {
                    line: 0,
                    index: 0,
                    affinity: Affinity::Before,
                }));
                // RETURN
                return;
            }
            if command && option && keys.just_pressed(KeyCode::Left) {
                cosmic_edit.editor.action(font_system, Action::PreviousWord);
                // RETURN
                return;
            }
            if command && option && keys.just_pressed(KeyCode::Right) {
                cosmic_edit.editor.action(font_system, Action::NextWord);
                // RETURN
                return;
            }
            let offset_y = get_y_offset(&cosmic_edit.editor);
            let offset_x = get_x_offset(&cosmic_edit.editor);
            if buttons.just_pressed(MouseButton::Left) {
                if let Some(node_cursor_pos) = get_node_cursor_pos(&window, node_transform, node) {
                    cosmic_edit.editor.action(
                        font_system,
                        Action::Click {
                            x: (node_cursor_pos.0 * window.scale_factor() as f32) as i32 - offset_x,
                            y: (node_cursor_pos.1 * window.scale_factor() as f32) as i32 - offset_y,
                        },
                    );
                }
                // RETURN
                return;
            }
            if buttons.pressed(MouseButton::Left) {
                if let Some(node_cursor_pos) = get_node_cursor_pos(&window, node_transform, node) {
                    cosmic_edit.editor.action(
                        font_system,
                        Action::Drag {
                            x: (node_cursor_pos.0 * window.scale_factor() as f32) as i32 - offset_x,
                            y: (node_cursor_pos.1 * window.scale_factor() as f32) as i32 - offset_y,
                        },
                    );
                }
                // RETURN
                return;
            }
            for char_ev in char_evr.iter() {
                if *is_deleting {
                    cosmic_edit.editor.action(font_system, Action::Backspace);
                } else {
                    cosmic_edit
                        .editor
                        .action(font_system, Action::Insert(char_ev.char));
                }
            }
        }
    }
}

fn cosmic_edit_redraw_buffer(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut images: ResMut<Assets<Image>>,
    mut font_system_state: ResMut<FontSystemState>,
    mut swash_cache_state: ResMut<SwashCacheState>,
    mut cosmic_edit_query: Query<
        (&mut CosmicEditImage, &mut UiImage, &Node),
        With<CosmicEditImage>,
    >,
) {
    let window = windows.single();
    let mut font_system = font_system_state.font_system.as_mut().unwrap();
    let mut swash_cache = swash_cache_state.swash_cache.as_mut().unwrap();
    for (mut cosmic_edit, mut img, node) in &mut cosmic_edit_query.iter_mut() {
        cosmic_edit.editor.shape_as_needed(&mut font_system);
        if cosmic_edit.editor.buffer().redraw() {
            let width = node.size().x * window.scale_factor() as f32;
            let height = node.size().y * window.scale_factor() as f32;
            let font_color = cosmic_text::Color::rgb(0, 0, 0);
            let mut pixels = vec![0; width as usize * height as usize * 4];
            let offset_y = get_y_offset(&cosmic_edit.editor);
            let offset_x = get_x_offset(&cosmic_edit.editor);
            cosmic_edit.editor.draw(
                &mut font_system,
                &mut swash_cache,
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
            let image: RgbaImage =
                ImageBuffer::from_vec(width as u32, height as u32, pixels).unwrap();
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
            *img = UiImage {
                texture: image.clone(),
                ..default()
            };
        }
    }
}

pub fn spawn_cosmic_edit(commands: &mut Commands, cosmic_edit_meta: CosmicEditMeta) -> Entity {
    let mut font_system = cosmic_edit_meta.font_system;
    let metrics = Metrics::new(cosmic_edit_meta.font_size, cosmic_edit_meta.line_height)
        .scale(cosmic_edit_meta.scale_factor);
    let mut buffer = Buffer::new(&mut font_system, metrics);
    buffer.set_size(
        &mut font_system,
        cosmic_edit_meta.width * cosmic_edit_meta.scale_factor as f32,
        cosmic_edit_meta.height * cosmic_edit_meta.scale_factor as f32,
    );
    let mut editor = Editor::new(buffer);
    editor.buffer_mut().lines.clear();
    let attrs = Attrs::new();
    editor
        .buffer_mut()
        .set_text(&mut font_system, cosmic_edit_meta.text.as_str(), attrs);
    let mut style = Style {
        size: Size {
            width: Val::Px(cosmic_edit_meta.width),
            height: Val::Px(cosmic_edit_meta.height),
        },
        ..default()
    };
    if !cosmic_edit_meta.is_visible {
        style.display = Display::None;
        editor.buffer_mut().set_redraw(false);
    }
    let mut image_bundle = ImageBundle {
        background_color: bevy::prelude::Color::WHITE.into(),
        style,
        ..default()
    };
    if let Some(initial_background) = cosmic_edit_meta.initial_background {
        image_bundle.image = initial_background.clone();
    }
    let cosmic_edit = commands
        .spawn((image_bundle, CosmicEditImage { editor }))
        .id();
    cosmic_edit
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
        | (buffer[offset + 0] as u32) << 16
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
    buffer[offset + 0] = (current >> 16) as u8;
    buffer[offset + 3] = (current >> 24) as u8;
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::*;

    fn test_spawn_cosmic_edit_system(mut commands: Commands) {
        let cosmic_edit_meta = CosmicEditMeta {
            text: "Blah".to_string(),
            width: 50.,
            height: 50.,
            font_size: 18.,
            line_height: 20.,
            scale_factor: 1.,
            font_system: &mut FontSystem::new(),
            is_visible: true,
            initial_background: None,
        };
        spawn_cosmic_edit(&mut commands, cosmic_edit_meta);
    }

    #[test]
    fn test_spawn_cosmic_edit() {
        let mut app = App::new();
        app.add_plugin(TaskPoolPlugin::default());
        app.add_plugin(AssetPlugin::default());
        app.add_plugin(WindowPlugin::default());
        app.add_plugin(CosmicEditPlugin);
        app.add_system(test_spawn_cosmic_edit_system);

        let input = Input::<KeyCode>::default();
        app.insert_resource(input);
        let mouse_input: Input<MouseButton> = Input::<MouseButton>::default();
        app.insert_resource(mouse_input);
        app.add_asset::<Image>();

        app.add_event::<ReceivedCharacter>();

        app.update();

        let mut text_nodes_query = app.world.query::<&CosmicEditImage>();
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
