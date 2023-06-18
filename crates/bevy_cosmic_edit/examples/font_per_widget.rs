use std::path::Path;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_cosmic_edit::{
    create_cosmic_font_system, get_cosmic_text, spawn_cosmic_edit, ActiveEditor, CosmicEdit,
    CosmicEditMeta, CosmicEditPlugin, CosmicFont, CosmicFontConfig, CosmicMetrics, CosmicNode,
    CosmicText, CosmicTextPos,
};
use cosmic_text::*;

fn setup(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
) {
    commands.spawn(Camera2dBundle::default());
    let root = commands
        .spawn(NodeBundle {
            style: bevy::prelude::Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
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

    let attrs = Attrs::new();
    let serif_attrs = attrs.family(Family::Serif);
    let mono_attrs = attrs.family(Family::Monospace);
    let comic_attrs = attrs.family(Family::Name("Comic Neue"));
    let lines: Vec<Vec<(String, AttrsOwned)>> = vec![
        vec![
            (
                String::from("B"),
                AttrsOwned::new(attrs.weight(cosmic_text::Weight::BOLD)),
            ),
            (String::from("old "), AttrsOwned::new(attrs)),
            (
                String::from("I"),
                AttrsOwned::new(attrs.style(cosmic_text::Style::Italic)),
            ),
            (String::from("talic "), AttrsOwned::new(attrs)),
            (String::from("f"), AttrsOwned::new(attrs)),
            (String::from("i "), AttrsOwned::new(attrs)),
            (
                String::from("f"),
                AttrsOwned::new(attrs.weight(cosmic_text::Weight::BOLD)),
            ),
            (String::from("i "), AttrsOwned::new(attrs)),
            (
                String::from("f"),
                AttrsOwned::new(attrs.style(cosmic_text::Style::Italic)),
            ),
            (String::from("i "), AttrsOwned::new(attrs)),
        ],
        vec![
            (String::from("Sans-Serif Normal "), AttrsOwned::new(attrs)),
            (
                String::from("Sans-Serif Bold "),
                AttrsOwned::new(attrs.weight(cosmic_text::Weight::BOLD)),
            ),
            (
                String::from("Sans-Serif Italic "),
                AttrsOwned::new(attrs.style(cosmic_text::Style::Italic)),
            ),
            (
                String::from("Sans-Serif Bold Italic"),
                AttrsOwned::new(
                    attrs
                        .weight(cosmic_text::Weight::BOLD)
                        .style(cosmic_text::Style::Italic),
                ),
            ),
        ],
        vec![
            (String::from("Serif Normal "), AttrsOwned::new(serif_attrs)),
            (
                String::from("Serif Bold "),
                AttrsOwned::new(serif_attrs.weight(cosmic_text::Weight::BOLD)),
            ),
            (
                String::from("Serif Italic "),
                AttrsOwned::new(serif_attrs.style(cosmic_text::Style::Italic)),
            ),
            (
                String::from("Serif Bold Italic"),
                AttrsOwned::new(
                    serif_attrs
                        .weight(cosmic_text::Weight::BOLD)
                        .style(cosmic_text::Style::Italic),
                ),
            ),
        ],
        vec![
            (String::from("Mono Normal "), AttrsOwned::new(mono_attrs)),
            (
                String::from("Mono Bold "),
                AttrsOwned::new(mono_attrs.weight(cosmic_text::Weight::BOLD)),
            ),
            (
                String::from("Mono Italic "),
                AttrsOwned::new(mono_attrs.style(cosmic_text::Style::Italic)),
            ),
            (
                String::from("Mono Bold Italic"),
                AttrsOwned::new(
                    mono_attrs
                        .weight(cosmic_text::Weight::BOLD)
                        .style(cosmic_text::Style::Italic),
                ),
            ),
        ],
        vec![
            (String::from("Comic Normal "), AttrsOwned::new(comic_attrs)),
            (
                String::from("Comic Bold "),
                AttrsOwned::new(comic_attrs.weight(cosmic_text::Weight::BOLD)),
            ),
            (
                String::from("Comic Italic "),
                AttrsOwned::new(comic_attrs.style(cosmic_text::Style::Italic)),
            ),
            (
                String::from("Comic Bold Italic"),
                AttrsOwned::new(
                    comic_attrs
                        .weight(cosmic_text::Weight::BOLD)
                        .style(cosmic_text::Style::Italic),
                ),
            ),
        ],
        vec![
            (
                String::from("R"),
                AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0xFF, 0x00, 0x00))),
            ),
            (
                String::from("A"),
                AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0xFF, 0x7F, 0x00))),
            ),
            (
                String::from("I"),
                AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0xFF, 0xFF, 0x00))),
            ),
            (
                String::from("N"),
                AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0x00, 0xFF, 0x00))),
            ),
            (
                String::from("B"),
                AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0x00, 0x00, 0xFF))),
            ),
            (
                String::from("O"),
                AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0x4B, 0x00, 0x82))),
            ),
            (
                String::from("W "),
                AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0x94, 0x00, 0xD3))),
            ),
            (
                String::from("Red "),
                AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0xFF, 0x00, 0x00))),
            ),
            (
                String::from("Orange "),
                AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0xFF, 0x7F, 0x00))),
            ),
            (
                String::from("Yellow "),
                AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0xFF, 0xFF, 0x00))),
            ),
            (
                String::from("Green "),
                AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0x00, 0xFF, 0x00))),
            ),
            (
                String::from("Blue "),
                AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0x00, 0x00, 0xFF))),
            ),
            (
                String::from("Indigo "),
                AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0x4B, 0x00, 0x82))),
            ),
            (
                String::from("Violet "),
                AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0x94, 0x00, 0xD3))),
            ),
            (
                String::from("U"),
                AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0x94, 0x00, 0xD3))),
            ),
            (
                String::from("N"),
                AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0x4B, 0x00, 0x82))),
            ),
            (
                String::from("I"),
                AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0x00, 0x00, 0xFF))),
            ),
            (
                String::from("C"),
                AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0x00, 0xFF, 0x00))),
            ),
            (
                String::from("O"),
                AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0xFF, 0xFF, 0x00))),
            ),
            (
                String::from("R"),
                AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0xFF, 0x7F, 0x00))),
            ),
            (
                String::from("N"),
                AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0xFF, 0x00, 0x00))),
            ),
        ],
        vec![(
            String::from("生活,삶,जिंदगी 😀 FPS"),
            AttrsOwned::new(attrs.color(cosmic_text::Color::rgb(0xFF, 0x00, 0x00))),
        )],
    ];

    let cosmic_edit_meta_1 = CosmicEditMeta {
        text: CosmicText::MultiStyle(lines),
        attrs: AttrsOwned::new(attrs),
        text_pos: CosmicTextPos::Center,
        metrics: CosmicMetrics {
            font_size: 18.,
            line_height: 22.,
            scale_factor: primary_window.scale_factor() as f32,
        },
        font_system_handle: font_system_handle.clone(),
        node: CosmicNode::Ui,
        size: None,
        bg: bevy::prelude::Color::WHITE,
        readonly: false,
        bg_image: None,
    };
    let cosmic_edit_1 = spawn_cosmic_edit(&mut commands, &mut cosmic_fonts, cosmic_edit_meta_1);

    let mut attrs_2 = cosmic_text::Attrs::new();
    attrs_2 = attrs_2.family(cosmic_text::Family::Name("Victor Mono"));
    let cosmic_edit_meta_2 = CosmicEditMeta {
        text: CosmicText::OneStyle("Widget 2.\nClick on me =>".to_string()),
        attrs: AttrsOwned::new(attrs_2),
        metrics: CosmicMetrics {
            font_size: 14.,
            line_height: 18.,
            scale_factor: primary_window.scale_factor() as f32,
        },
        font_system_handle: font_system_handle.clone(),
        node: CosmicNode::Ui,
        text_pos: CosmicTextPos::Center,
        size: None,
        bg: bevy::prelude::Color::WHITE.with_a(0.8),
        readonly: false,
        bg_image: None,
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
            info!(
                "Widget text: {}",
                get_cosmic_text(&cosmic_edit.editor.buffer())
            );
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CosmicEditPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, change_active_editor)
        .run();
}
