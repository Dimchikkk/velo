use std::path::Path;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_cosmic_edit::{
    bevy_color_to_cosmic, create_cosmic_font_system, get_cosmic_text, spawn_cosmic_edit,
    ActiveEditor, CosmicEdit, CosmicEditMeta, CosmicEditPlugin, CosmicFont, CosmicFontConfig,
    CosmicTextPos,
};

fn setup(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
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
    let cosmic_font_config = CosmicFontConfig {
        fonts_dir_path: Some(Path::new("assets/fonts").into()),
        font_bytes: None,
        load_system_fonts: true,
    };
    let font_system = create_cosmic_font_system(cosmic_font_config);
    let font_system_handle = cosmic_fonts.add(CosmicFont(font_system));

    let mut attrs_1 = cosmic_text::Attrs::new();
    attrs_1 = attrs_1.family(cosmic_text::Family::Name("Fira Code"));
    attrs_1 = attrs_1.color(bevy_color_to_cosmic(Color::BLACK));
    let metrics_1 = cosmic_text::Metrics::new(14., 18.).scale(primary_window.scale_factor() as f32);
    let cosmic_edit_meta_1 = CosmicEditMeta {
        text: "😀😀😀 x => y".to_string(),
        text_pos: CosmicTextPos::Center,
        attrs: attrs_1,
        metrics: metrics_1,
        font_system_handle: font_system_handle.clone(),
        display_none: false,
        initial_background: None,
        initial_size: None,
    };
    let cosmic_edit_1 = spawn_cosmic_edit(&mut commands, &mut cosmic_fonts, cosmic_edit_meta_1);

    let mut attrs_2 = cosmic_text::Attrs::new();
    attrs_2 = attrs_2.family(cosmic_text::Family::Name("Source Code Pro"));
    attrs_2 = attrs_2.weight(cosmic_text::Weight::BOLD);
    let metrics_2 = cosmic_text::Metrics::new(18., 20.).scale(primary_window.scale_factor() as f32);
    let cosmic_edit_meta_2 = CosmicEditMeta {
        text: "Widget 2.\nClick on me".to_string(),
        attrs: attrs_2,
        metrics: metrics_2,
        font_system_handle: font_system_handle.clone(),
        display_none: false,
        initial_background: None,
        text_pos: CosmicTextPos::Center,
        initial_size: None,
    };
    let cosmic_edit_2 = spawn_cosmic_edit(&mut commands, &mut cosmic_fonts, cosmic_edit_meta_2);

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
