use std::path::Path;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_cosmic_edit::{
    create_cosmic_font_system, spawn_cosmic_edit, ActiveEditor, CosmicEditMeta, CosmicEditPlugin,
    CosmicFont, CosmicFontConfig, CosmicTextPos,
};

fn setup(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
) {
    let primary_window = windows.single();
    commands.spawn(Camera2dBundle::default());
    let root = commands
        .spawn(NodeBundle {
            background_color: Color::WHITE.into(),
            style: Style {
                display: Display::Flex,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
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
    attrs = attrs.family(cosmic_text::Family::Name("Fira Code"));
    attrs = attrs.color(cosmic_text::Color::rgb(0x94, 0x00, 0xD3));
    let metrics = cosmic_text::Metrics::new(14., 18.).scale(primary_window.scale_factor() as f32);
    let cosmic_edit_meta = CosmicEditMeta {
        text: "😀😀😀 x => y".to_string(),
        text_pos: CosmicTextPos::Center,
        attrs,
        metrics,
        font_system_handle,
        display_none: false,
        initial_background: None,
        initial_size: None,
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
        .add_plugin(CosmicEditPlugin)
        .add_startup_system(setup)
        .run();
}
