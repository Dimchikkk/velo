use std::path::Path;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_cosmic_edit::{
    create_cosmic_font_system, get_cosmic_text, spawn_cosmic_edit, ActiveEditor, CosmicEdit,
    CosmicEditMeta, CosmicEditPlugin, CosmicEditUi, CosmicFont, CosmicFontConfig, CosmicMetrics,
    CosmicNode, CosmicText, CosmicTextPos,
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

    let attrs = Attrs::new();
    let serif_attrs = attrs.family(Family::Serif);
    let mono_attrs = attrs.family(Family::Monospace);
    let comic_attrs = attrs.family(Family::Name("Comic Neue"));
    let lines: Vec<Vec<(&str, Attrs)>> = vec![
        vec![
            ("B", attrs.weight(cosmic_text::Weight::BOLD)),
            ("old ", attrs),
            ("I", attrs.style(cosmic_text::Style::Italic)),
            ("talic ", attrs),
            ("f", attrs),
            ("i ", attrs),
            ("f", attrs.weight(Weight::BOLD)),
            ("i ", attrs),
            ("f", attrs.style(cosmic_text::Style::Italic)),
            ("i ", attrs),
        ],
        vec![
            ("Sans-Serif Normal ", attrs),
            ("Sans-Serif Bold ", attrs.weight(Weight::BOLD)),
            (
                "Sans-Serif Italic ",
                attrs.style(cosmic_text::Style::Italic),
            ),
            (
                "Sans-Serif Bold Italic",
                attrs.weight(Weight::BOLD).style(cosmic_text::Style::Italic),
            ),
        ],
        vec![
            ("Serif Normal ", serif_attrs),
            ("Serif Bold ", serif_attrs.weight(Weight::BOLD)),
            (
                "Serif Italic ",
                serif_attrs.style(cosmic_text::Style::Italic),
            ),
            (
                "Serif Bold Italic",
                serif_attrs
                    .weight(Weight::BOLD)
                    .style(cosmic_text::Style::Italic),
            ),
        ],
        vec![
            ("Mono Normal ", mono_attrs),
            ("Mono Bold ", mono_attrs.weight(Weight::BOLD)),
            ("Mono Italic ", mono_attrs.style(cosmic_text::Style::Italic)),
            (
                "Mono Bold Italic",
                mono_attrs
                    .weight(Weight::BOLD)
                    .style(cosmic_text::Style::Italic),
            ),
        ],
        vec![
            ("Comic Normal ", comic_attrs),
            ("Comic Bold ", comic_attrs.weight(Weight::BOLD)),
            (
                "Comic Italic ",
                comic_attrs.style(cosmic_text::Style::Italic),
            ),
            (
                "Comic Bold Italic",
                comic_attrs
                    .weight(Weight::BOLD)
                    .style(cosmic_text::Style::Italic),
            ),
        ],
        vec![
            ("R", attrs.color(cosmic_text::Color::rgb(0xFF, 0x00, 0x00))),
            ("A", attrs.color(cosmic_text::Color::rgb(0xFF, 0x7F, 0x00))),
            ("I", attrs.color(cosmic_text::Color::rgb(0xFF, 0xFF, 0x00))),
            ("N", attrs.color(cosmic_text::Color::rgb(0x00, 0xFF, 0x00))),
            ("B", attrs.color(cosmic_text::Color::rgb(0x00, 0x00, 0xFF))),
            ("O", attrs.color(cosmic_text::Color::rgb(0x4B, 0x00, 0x82))),
            ("W ", attrs.color(cosmic_text::Color::rgb(0x94, 0x00, 0xD3))),
            (
                "Red ",
                attrs.color(cosmic_text::Color::rgb(0xFF, 0x00, 0x00)),
            ),
            (
                "Orange ",
                attrs.color(cosmic_text::Color::rgb(0xFF, 0x7F, 0x00)),
            ),
            (
                "Yellow ",
                attrs.color(cosmic_text::Color::rgb(0xFF, 0xFF, 0x00)),
            ),
            (
                "Green ",
                attrs.color(cosmic_text::Color::rgb(0x00, 0xFF, 0x00)),
            ),
            (
                "Blue ",
                attrs.color(cosmic_text::Color::rgb(0x00, 0x00, 0xFF)),
            ),
            (
                "Indigo ",
                attrs.color(cosmic_text::Color::rgb(0x4B, 0x00, 0x82)),
            ),
            (
                "Violet ",
                attrs.color(cosmic_text::Color::rgb(0x94, 0x00, 0xD3)),
            ),
            ("U", attrs.color(cosmic_text::Color::rgb(0x94, 0x00, 0xD3))),
            ("N", attrs.color(cosmic_text::Color::rgb(0x4B, 0x00, 0x82))),
            ("I", attrs.color(cosmic_text::Color::rgb(0x00, 0x00, 0xFF))),
            ("C", attrs.color(cosmic_text::Color::rgb(0x00, 0xFF, 0x00))),
            ("O", attrs.color(cosmic_text::Color::rgb(0xFF, 0xFF, 0x00))),
            ("R", attrs.color(cosmic_text::Color::rgb(0xFF, 0x7F, 0x00))),
            ("N", attrs.color(cosmic_text::Color::rgb(0xFF, 0x00, 0x00))),
        ],
        vec![(
            "生活,삶,जिंदगी 😀 FPS",
            attrs.color(cosmic_text::Color::rgb(0xFF, 0x00, 0x00)),
        )],
    ];
    let cosmic_edit_meta_1 = CosmicEditMeta {
        text: CosmicText::MultiStyle((lines, attrs)),
        text_pos: CosmicTextPos::Center,
        metrics: CosmicMetrics {
            font_size: 18.,
            line_height: 22.,
            scale_factor: primary_window.scale_factor() as f32,
        },
        font_system_handle: font_system_handle.clone(),
        node: CosmicNode::Ui(CosmicEditUi {
            display_none: false,
        }),
        size: None,
        bg: bevy::prelude::Color::WHITE,
        readonly: false,
    };
    let cosmic_edit_1 = spawn_cosmic_edit(&mut commands, &mut cosmic_fonts, cosmic_edit_meta_1);

    let mut attrs_2 = cosmic_text::Attrs::new();
    attrs_2 = attrs_2.family(cosmic_text::Family::Name("Fira Code"));
    let cosmic_edit_meta_2 = CosmicEditMeta {
        text: CosmicText::OneStyle(("Widget 2.\nClick on me =>".to_string(), attrs_2)),
        metrics: CosmicMetrics {
            font_size: 14.,
            line_height: 18.,
            scale_factor: primary_window.scale_factor() as f32,
        },
        font_system_handle: font_system_handle.clone(),
        node: CosmicNode::Ui(CosmicEditUi {
            display_none: false,
        }),
        text_pos: CosmicTextPos::Center,
        size: None,
        bg: bevy::prelude::Color::WHITE.with_a(0.8),
        readonly: false,
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
