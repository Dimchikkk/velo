use base64::{engine::general_purpose, Engine};
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use image::{load_from_memory_with_format, ImageFormat};
use serde_json::Value;

use crate::{AppState, JsonNode, LoadRequest, Tab};

use super::ui_helpers::{
    add_rectangle_txt, spawn_node, ArrowMeta, BottomPanel, CreateArrow, NodeMeta, Rectangle,
    ReflectableUuid, SelectedTab,
};

pub fn should_load(request: Option<Res<LoadRequest>>) -> bool {
    request.is_some()
}

pub fn remove_load_request(world: &mut World) {
    world.remove_resource::<LoadRequest>().unwrap();
}

pub fn load_json(
    old_nodes: Query<Entity, With<Rectangle>>,
    old_arrows: Query<Entity, With<ArrowMeta>>,
    request: Res<LoadRequest>,
    mut state: ResMut<AppState>,
    mut commands: Commands,
    mut res_images: ResMut<Assets<Image>>,
    mut create_arrow: EventWriter<CreateArrow>,
    asset_server: Res<AssetServer>,
    mut selected_tabs_query: Query<Entity, With<SelectedTab>>,
    mut bottom_panel: Query<Entity, With<BottomPanel>>,
) {
    state.entity_to_edit = None;
    state.hold_entity = None;
    state.entity_to_resize = None;
    state.arrow_to_draw_start = None;

    let font = asset_server.load("fonts/iosevka-regular.ttf");

    for entity in old_arrows.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in old_nodes.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in selected_tabs_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }

    if let Some(path) = &request.path {
        let json = std::fs::read_to_string(path).expect("Error reading state from file");
        let json: Value = serde_json::from_str(&json).unwrap();
        let tabs: Vec<Tab> = serde_json::from_value(json["tabs"].clone()).unwrap();
        state.tabs = tabs;
    }

    let bottom_panel = bottom_panel.single_mut();
    for tab in state.tabs.iter() {
        let color = if tab.is_active {
            Color::rgba(0.8, 0.8, 0.8, 0.5)
        } else {
            Color::rgba(0.8, 0.8, 0.8, 0.8)
        };
        let tab_view = commands
            .spawn((
                ButtonBundle {
                    background_color: color.into(),
                    style: Style {
                        size: Size::new(Val::Px(60.), Val::Px(30.)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect {
                            left: Val::Px(10.),
                            right: Val::Px(10.),
                            top: Val::Px(0.),
                            bottom: Val::Px(0.),
                        },
                        ..default()
                    },

                    ..default()
                },
                SelectedTab { id: tab.id },
            ))
            .with_children(|builder| {
                builder.spawn(add_rectangle_txt(font.clone(), tab.name.clone()));
            })
            .id();
        commands.entity(bottom_panel).add_child(tab_view);
    }

    for tab in state.tabs.iter_mut() {
        if tab.is_active {
            let json = if request.drop_last {
                tab.checkpoints.pop_back().unwrap()
            } else {
                tab.checkpoints.back().unwrap().clone()
            };
            let mut json: Value = serde_json::from_str(&json).unwrap();
            let images = json["images"].as_object().unwrap();
            let nodes = json["nodes"].as_array().unwrap();
            for node in nodes.iter() {
                let json_node: JsonNode = serde_json::from_value(node.clone()).unwrap();
                let image: Option<UiImage> = match images.get(&json_node.id.to_string()) {
                    Some(image) => {
                        let image_bytes = general_purpose::STANDARD
                            .decode(image.as_str().unwrap().as_bytes())
                            .unwrap();
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let img = load_from_memory_with_format(&image_bytes, ImageFormat::Png)
                                .unwrap();
                            let size: Extent3d = Extent3d {
                                width: img.width(),
                                height: img.height(),
                                ..Default::default()
                            };
                            let image = Image::new(
                                size,
                                TextureDimension::D2,
                                img.into_bytes(),
                                TextureFormat::Rgba8UnormSrgb,
                            );
                            let image_handle = res_images.add(image);
                            Some(image_handle.into())
                        }
                        #[cfg(target_arch = "wasm32")]
                        None
                    }
                    None => None,
                };
                // ideally AddRect event should be fired instead of calling spawn_node directly
                let entity = spawn_node(
                    &mut commands,
                    NodeMeta {
                        font: font.clone(),
                        size: (json_node.width, json_node.height),
                        id: ReflectableUuid(json_node.id),
                        image: image.clone(),
                        text: json_node.text.text.clone(),
                        bg_color: json_node.bg_color,
                        position: (json_node.left, json_node.bottom),
                        tags: json_node.tags,
                        text_pos: json_node.text.pos,
                        z_index: json_node.z_index,
                    },
                );
                commands.entity(state.main_panel.unwrap()).add_child(entity);
            }

            let arrows = json["arrows"].as_array_mut().unwrap();
            for arrow in arrows.iter() {
                let arrow_meta: ArrowMeta = serde_json::from_value(arrow.clone()).unwrap();
                create_arrow.send(CreateArrow {
                    start: arrow_meta.start,
                    end: arrow_meta.end,
                    arrow_type: arrow_meta.arrow_type,
                });
            }
            break;
        }
    }
}
