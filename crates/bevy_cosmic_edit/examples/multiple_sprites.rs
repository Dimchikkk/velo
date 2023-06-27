use std::path::Path;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, window::PrimaryWindow};
use bevy_cosmic_edit::{
    create_cosmic_font_system, spawn_cosmic_edit, ActiveEditor, CosmicEdit, CosmicEditMeta,
    CosmicEditPlugin, CosmicEditSprite, CosmicFont, CosmicFontConfig, CosmicMetrics, CosmicNode,
    CosmicText, CosmicTextPos,
};
use bevy_pancam::{PanCam, PanCamPlugin};
use cosmic_text::AttrsOwned;

#[derive(Component)]
pub struct MainCamera;

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
    commands.spawn((camera_bundle, MainCamera)).insert(PanCam {
        grab_buttons: vec![MouseButton::Right],
        enabled: true,
        zoom_to_cursor: false,
        min_scale: 1.,
        max_scale: Some(3.),
        ..default()
    });
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
    let metrics = CosmicMetrics {
        font_size: 14.,
        line_height: 18.,
        scale_factor: primary_window.scale_factor() as f32,
    };
    let cosmic_edit_meta = CosmicEditMeta {
        text: CosmicText::OneStyle("😀😀😀 x => y".to_string()),
        attrs: AttrsOwned::new(attrs),
        text_pos: CosmicTextPos::Center,
        metrics: metrics.clone(),
        font_system_handle: font_system_handle.clone(),
        node: CosmicNode::Sprite(CosmicEditSprite {
            transform: Transform {
                translation: Vec3::new(-primary_window.width() / 4., 0., 1.),
                ..default()
            },
        }),
        size: Some((primary_window.width() / 2., primary_window.height())),
        bg: Color::WHITE,
        readonly: false,
        bg_image: None,
    };
    let cosmic_edit_1 = spawn_cosmic_edit(&mut commands, &mut cosmic_fonts, cosmic_edit_meta);
    let cosmic_edit_meta = CosmicEditMeta {
        text: CosmicText::OneStyle("Widget_2. Click on me".to_string()),
        attrs: AttrsOwned::new(attrs),
        text_pos: CosmicTextPos::Center,
        metrics: metrics.clone(),
        font_system_handle: font_system_handle.clone(),
        node: CosmicNode::Sprite(CosmicEditSprite {
            transform: Transform {
                translation: Vec3::new(
                    primary_window.width() / 4.,
                    -primary_window.height() / 4.,
                    1.,
                ),
                ..default()
            },
        }),
        size: Some((primary_window.width() / 2., primary_window.height() / 2.)),
        bg: Color::GRAY.with_a(0.5),
        readonly: false,
        bg_image: None,
    };
    let _ = spawn_cosmic_edit(&mut commands, &mut cosmic_fonts, cosmic_edit_meta);
    let cosmic_edit_meta = CosmicEditMeta {
        text: CosmicText::OneStyle("Widget_3. Click on me".to_string()),
        attrs: AttrsOwned::new(attrs),
        text_pos: CosmicTextPos::Center,
        metrics: metrics.clone(),
        font_system_handle: font_system_handle.clone(),
        node: CosmicNode::Sprite(CosmicEditSprite {
            transform: Transform {
                translation: Vec3::new(
                    primary_window.width() / 4.,
                    primary_window.height() / 4.,
                    1.,
                ),
                ..default()
            },
        }),
        size: Some((primary_window.width() / 2., primary_window.height() / 2.)),
        bg: Color::GRAY.with_a(0.8),
        readonly: false,
        bg_image: None,
    };
    let _ = spawn_cosmic_edit(&mut commands, &mut cosmic_fonts, cosmic_edit_meta);
    commands.insert_resource(ActiveEditor {
        entity: Some(cosmic_edit_1),
    });
}

fn change_active_editor(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    mut cosmic_edit_query: Query<(&mut CosmicEdit, &GlobalTransform, Entity), With<CosmicEdit>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();
    if buttons.just_pressed(MouseButton::Left) {
        for (cosmic_edit, node_transform, entity) in &mut cosmic_edit_query.iter_mut() {
            let size = (cosmic_edit.width, cosmic_edit.height);
            let x_min = node_transform.affine().translation.x - size.0 / 2.;
            let y_min = node_transform.affine().translation.y - size.1 / 2.;
            let x_max = node_transform.affine().translation.x + size.0 / 2.;
            let y_max = node_transform.affine().translation.y + size.1 / 2.;
            window.cursor_position().and_then(|pos| {
                if let Some(pos) = camera.viewport_to_world_2d(camera_transform, pos) {
                    Some({
                        if x_min < pos.x && pos.x < x_max && y_min < pos.y && pos.y < y_max {
                            commands.insert_resource(ActiveEditor {
                                entity: Some(entity),
                            });
                        };
                    })
                } else {
                    None
                }
            });
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CosmicEditPlugin)
        .add_plugins(PanCamPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, change_active_editor)
        .run();
}
