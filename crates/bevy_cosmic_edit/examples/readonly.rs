use std::path::Path;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_cosmic_edit::{
    create_cosmic_font_system, spawn_cosmic_edit, ActiveEditor, CosmicEditMeta, CosmicEditPlugin,
    CosmicFont, CosmicFontConfig, CosmicMetrics, CosmicNode, CosmicText, CosmicTextPos,
};
use cosmic_text::AttrsOwned;

fn setup(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
) {
    let primary_window = windows.single();
    commands.spawn(Camera2dBundle::default());
    let root = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            ..default()
        })
        .id();

    let cosmic_font_config = CosmicFontConfig {
        fonts_dir_path: Some(Path::new("assets/fonts").into()),
        font_bytes: None,
        load_system_fonts: true,
    };

    let font_system = create_cosmic_font_system(cosmic_font_config);
    let font_system_handle = cosmic_fonts.add(CosmicFont(font_system));
    let mut attrs = cosmic_text::Attrs::new();
    attrs = attrs.family(cosmic_text::Family::Name("Victor Mono"));
    attrs = attrs.color(cosmic_text::Color::rgb(0x94, 0x00, 0xD3));
    let cosmic_edit_meta = CosmicEditMeta {
        text: CosmicText::OneStyle("😀😀😀 x => y\nRead only widget".to_string()),
        attrs: AttrsOwned::new(attrs),
        text_pos: CosmicTextPos::Center,
        bg: Color::WHITE,
        metrics: CosmicMetrics {
            font_size: 14.,
            line_height: 18.,
            scale_factor: primary_window.scale_factor() as f32,
        },
        font_system_handle,
        node: CosmicNode::Ui,
        size: None,
        readonly: true,
        bg_image: None,
    };
    let cosmic_edit = spawn_cosmic_edit(&mut commands, &mut cosmic_fonts, cosmic_edit_meta);
    commands.entity(root).add_child(cosmic_edit);
    commands.insert_resource(ActiveEditor {
        entity: Some(cosmic_edit),
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CosmicEditPlugin)
        .add_systems(Startup, setup)
        .run();
}
