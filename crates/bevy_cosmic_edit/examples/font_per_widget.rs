use std::path::Path;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_cosmic_edit::{
    create_cosmic_font_system, get_cosmic_text, spawn_cosmic_edit, ActiveEditor, CosmicEdit,
    CosmicEditEventer, CosmicEditMeta, CosmicEditPlugin, CosmicFont, CosmicFontConfig,
    CosmicTextPos,
};

fn setup(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    mut cosmic_edit_eventer: EventWriter<CosmicEditEventer>,
) {
    commands.spawn(Camera2dBundle::default());
    let root = commands
        .spawn(NodeBundle {
            background_color: Color::WHITE.into(),
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..default()
            },
            ..default()
        })
        .id();
    let primary_window = windows.single();

    let cosmic_font_config_1 = CosmicFontConfig {
        fonts_dir_path: Some(Path::new("assets/fonts").into()),
        custom_font_data: None,
        load_system_fonts: true,
        monospace_family: Some("Fira Code".to_string()),
        sans_serif_family: Some("Fira Code".to_string()),
        serif_family: Some("Fira Code".to_string()),
    };
    let font_system_1 = create_cosmic_font_system(cosmic_font_config_1);
    let font_system_handle_1 = cosmic_fonts.add(CosmicFont(font_system_1));
    let cosmic_edit_meta_1 = CosmicEditMeta {
        text: "😀😀😀 x => y".to_string(),
        font_size: 18.,
        line_height: 20.,
        scale_factor: primary_window.scale_factor() as f32,
        font_system_handle: font_system_handle_1,
        display_none: false,
        initial_background: None,
        text_pos: CosmicTextPos::Center,
        initial_size: None,
    };
    let cosmic_edit_1 = spawn_cosmic_edit(
        &mut commands,
        &mut cosmic_edit_eventer,
        &mut cosmic_fonts,
        cosmic_edit_meta_1,
    );

    let cosmic_font_config_2 = CosmicFontConfig {
        fonts_dir_path: Some(Path::new("assets/fonts").into()),
        custom_font_data: None,
        load_system_fonts: true,
        monospace_family: Some("Source Code Pro".to_string()),
        sans_serif_family: Some("Source Code Pro".to_string()),
        serif_family: Some("Source Code Pro".to_string()),
    };
    let font_system_2 = create_cosmic_font_system(cosmic_font_config_2);
    let font_system_handle_2 = cosmic_fonts.add(CosmicFont(font_system_2));
    let cosmic_edit_meta_2 = CosmicEditMeta {
        text: "Widget 2.\nClick on me".to_string(),
        font_size: 20.,
        line_height: 22.,
        scale_factor: primary_window.scale_factor() as f32,
        font_system_handle: font_system_handle_2,
        display_none: false,
        initial_background: None,
        text_pos: CosmicTextPos::Center,
        initial_size: None,
    };
    let cosmic_edit_2 = spawn_cosmic_edit(
        &mut commands,
        &mut cosmic_edit_eventer,
        &mut cosmic_fonts,
        cosmic_edit_meta_2,
    );

    commands.entity(root).add_child(cosmic_edit_1);
    commands.entity(root).add_child(cosmic_edit_2);

    commands.insert_resource(ActiveEditor {
        entity: Some(cosmic_edit_1),
    });
}

fn change_active_editor(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut CosmicEdit, Entity),
        (Changed<Interaction>, With<CosmicEdit>),
    >,
) {
    for (interaction, cosmic_edit, entity) in interaction_query.iter_mut() {
        if let Interaction::Clicked = interaction {
            commands.insert_resource(ActiveEditor {
                entity: Some(entity),
            });
            info!("Widget text: {}", get_cosmic_text(&cosmic_edit.editor));
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CosmicEditPlugin)
        .add_startup_system(setup)
        .add_system(change_active_editor)
        .run();
}
