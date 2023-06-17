use std::path::Path;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, window::PrimaryWindow};
use bevy_cosmic_edit::{
    create_cosmic_font_system, spawn_cosmic_edit, ActiveEditor, CosmicEditMeta, CosmicEditPlugin,
    CosmicEditSprite, CosmicFont, CosmicFontConfig, CosmicMetrics, CosmicNode, CosmicText,
    CosmicTextPos,
};
use cosmic_text::AttrsOwned;

fn setup(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
) {
    let primary_window = windows.single();
    let camera_bundle = Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::WHITE),
        },
        ..default()
    };
    commands.spawn(camera_bundle);
    let cosmic_font_config = CosmicFontConfig {
        fonts_dir_path: Some(Path::new("assets/fonts").into()),
        font_bytes: None,
        load_system_fonts: true,
    };
    let font_system = create_cosmic_font_system(cosmic_font_config);
    let font_system_handle: Handle<CosmicFont> = cosmic_fonts.add(CosmicFont(font_system));
    let mut attrs = cosmic_text::Attrs::new();
    attrs = attrs.family(cosmic_text::Family::Name("Victor Mono"));
    attrs = attrs.color(cosmic_text::Color::rgb(0x94, 0x00, 0xD3));
    let scale_factor = primary_window.scale_factor() as f32;
    let cosmic_edit_meta = CosmicEditMeta {
        text: CosmicText::OneStyle("😀😀😀 x => y".to_string()),
        attrs: AttrsOwned::new(attrs),
        text_pos: CosmicTextPos::Center,
        bg: Color::GRAY.with_a(0.5),
        metrics: CosmicMetrics {
            font_size: 14.,
            line_height: 18.,
            scale_factor,
        },
        font_system_handle,
        node: CosmicNode::Sprite(CosmicEditSprite {
            transform: Transform {
                translation: Vec3::new(0., 0., 1.),
                ..default()
            },
        }),
        size: Some((primary_window.width(), primary_window.height())),
        readonly: false,
        bg_image: None,
    };
    let cosmic_edit = spawn_cosmic_edit(&mut commands, &mut cosmic_fonts, cosmic_edit_meta);
    commands.insert_resource(ActiveEditor {
        entity: Some(cosmic_edit),
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CosmicEditPlugin)
        .add_systems(Startup, setup)
        .run();
}
