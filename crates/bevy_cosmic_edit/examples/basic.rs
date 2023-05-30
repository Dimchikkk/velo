use std::path::Path;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_cosmic_edit::{
    create_cosmic_font_system, spawn_cosmic_edit, ActiveEditor, CosmicEditEventer, CosmicEditMeta,
    CosmicEditPlugin, CosmicFont, CosmicFontConfig, CosmicTextPos,
};

fn setup(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cosmic_edit_eventer: EventWriter<CosmicEditEventer>,
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
        custom_font_data: None,
        load_system_fonts: true,
        monospace_family: Some("Fira Code".to_string()),
        sans_serif_family: Some("Fira Code".to_string()),
        serif_family: Some("Fira Code".to_string()),
    };
    let font_system = create_cosmic_font_system(cosmic_font_config);
    let font_system_handle = cosmic_fonts.add(CosmicFont(font_system));
    let cosmic_edit_meta = CosmicEditMeta {
        text: "😀😀😀 x => y".to_string(),
        font_size: 18.,
        line_height: 20.,
        scale_factor: primary_window.scale_factor() as f32,
        font_system_handle,
        display_none: false,
        initial_background: None,
        text_pos: CosmicTextPos::Center,
        initial_size: None,
    };
    let cosmic_edit = spawn_cosmic_edit(
        &mut commands,
        &mut cosmic_edit_eventer,
        &mut cosmic_fonts,
        cosmic_edit_meta,
    );
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
