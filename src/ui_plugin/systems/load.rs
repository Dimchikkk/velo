use base64::{engine::general_purpose, Engine};
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    window::PrimaryWindow,
};
use bevy_cosmic_edit::CosmicFont;
use bevy_prototype_lyon::prelude::{PathBuilder, ShapeBundle, Stroke};

use super::{
    ui_helpers::{
        add_tab, spawn_sprite_node, BottomPanel, Drawing, InteractiveNode, NodeMeta, TabContainer,
        VeloNode,
    },
    DeleteDoc, DeleteTab, DrawingJsonNode,
};
use crate::{canvas::arrow::events::CreateArrow, utils::load_doc_to_memory};
use crate::{
    canvas::{arrow::components::ArrowMeta, shadows::CustomShadowMaterial},
    resources::{FontSystemState, LoadTabRequest},
    themes::Theme,
};

use crate::resources::{AppState, LoadDocRequest};
use crate::utils::ReflectableUuid;
use crate::{JsonNode, UiState};
use bevy_pkv::PkvStore;
use image::{load_from_memory_with_format, ImageFormat};
use serde_json::{Map, Value};

pub fn should_load_doc(request: Option<Res<LoadDocRequest>>) -> bool {
    request.is_some()
}

pub fn should_load_tab(request: Option<Res<LoadTabRequest>>) -> bool {
    request.is_some()
}

pub fn remove_load_tab_request(world: &mut World) {
    world.remove_resource::<LoadTabRequest>().unwrap();
}

pub fn remove_load_doc_request(world: &mut World) {
    world.remove_resource::<LoadDocRequest>().unwrap();
}

pub fn load_doc(
    request: Res<LoadDocRequest>,
    mut app_state: ResMut<AppState>,
    mut commands: Commands,
    mut bottom_panel: Query<Entity, With<BottomPanel>>,
    mut pkv: ResMut<PkvStore>,
    asset_server: Res<AssetServer>,
    mut tabs_query: Query<Entity, With<TabContainer>>,
    mut delete_doc: Query<(&mut Visibility, &DeleteDoc), With<DeleteDoc>>,
    theme: Res<Theme>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    font_system_state: ResMut<FontSystemState>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let primary_window = windows.single();
    let scale_factor = primary_window.scale_factor() as f32;
    let bottom_panel = bottom_panel.single_mut();
    let doc_id = request.doc_id;
    for (mut visibility, doc) in delete_doc.iter_mut() {
        if doc.id == doc_id {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
    load_doc_to_memory(doc_id, &mut app_state, &mut pkv);

    let mut tabs = vec![];
    for entity in tabs_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
    for tab in app_state.docs.get_mut(&doc_id).unwrap().tabs.iter() {
        let tab_view: Entity = add_tab(
            &mut commands,
            &mut cosmic_fonts,
            font_system_state.0.clone().unwrap(),
            &theme,
            &asset_server,
            tab.name.clone(),
            tab.id,
            tab.is_active,
            scale_factor,
        );
        tabs.push(tab_view);
        if tab.is_active {
            commands.insert_resource(LoadTabRequest {
                doc_id,
                tab_id: tab.id,
                drop_last_checkpoint: false,
            });
        }
    }
    commands.entity(bottom_panel).insert_children(0, &tabs);
}

pub fn load_tab(
    old_nodes: Query<Entity, With<VeloNode>>,
    mut old_arrows: Query<Entity, With<ArrowMeta>>,
    mut old_drawings: Query<Entity, With<Drawing<(String, Color)>>>,
    request: Res<LoadTabRequest>,
    mut app_state: ResMut<AppState>,
    mut ui_state: ResMut<UiState>,
    mut commands: Commands,
    mut res_images: ResMut<Assets<Image>>,
    mut create_arrow: EventWriter<CreateArrow>,
    mut delete_tab: Query<(&mut Visibility, &DeleteTab), (With<DeleteTab>, Without<ArrowMeta>)>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    font_system_state: ResMut<FontSystemState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    theme: Res<Theme>,
    mut local_theme: Local<Option<Map<String, Value>>>,
    mut materials_meshes: (ResMut<Assets<CustomShadowMaterial>>, ResMut<Assets<Mesh>>),
) {
    *ui_state = UiState::default();
    let value = serde_json::to_value(&*theme).unwrap();
    if local_theme.is_none() || theme.is_changed() {
        *local_theme = Some(value.as_object().unwrap().clone());
    }

    commands.insert_resource(bevy_cosmic_edit::ActiveEditor { entity: None });
    let primary_window = windows.single_mut();
    let scale_factor = primary_window.scale_factor() as f32;

    for entity in &mut old_arrows.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &mut old_drawings.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in old_nodes.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let doc_id = request.doc_id;
    for (mut visibility, tab) in delete_tab.iter_mut() {
        if tab.id == request.tab_id {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
    for tab in app_state.docs.get_mut(&doc_id).unwrap().tabs.iter_mut() {
        if tab.id == request.tab_id {
            if tab.checkpoints.is_empty() {
                break;
            }

            let json = if request.drop_last_checkpoint && tab.checkpoints.len() > 1 {
                tab.checkpoints.pop_back().unwrap()
            } else {
                tab.checkpoints.back().unwrap().clone()
            };
            let mut json: Value = serde_json::from_str(&json).unwrap();
            let images = json["images"].as_object().unwrap();
            let nodes = json["nodes"].as_array().unwrap();
            for node in nodes.iter() {
                let json_node: JsonNode<String> = serde_json::from_value(node.clone()).unwrap();
                let image: Option<Handle<Image>> = match images.get(&json_node.id.to_string()) {
                    Some(image) => {
                        let image_bytes = general_purpose::STANDARD
                            .decode(image.as_str().unwrap().as_bytes())
                            .unwrap();
                        let img =
                            load_from_memory_with_format(&image_bytes, ImageFormat::Png).unwrap();
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
                        Some(image_handle)
                    }
                    None => None,
                };
                let theme_color = local_theme
                    .as_ref()
                    .unwrap()
                    .get(json_node.bg_color.as_str())
                    .unwrap();
                let pair_bg_color = (
                    json_node.bg_color,
                    serde_json::from_value(theme_color.clone()).unwrap(),
                );
                let _ = spawn_sprite_node(
                    &mut commands,
                    &mut materials_meshes.0,
                    &mut materials_meshes.1,
                    &theme,
                    &mut cosmic_fonts,
                    font_system_state.0.clone().unwrap(),
                    scale_factor,
                    NodeMeta {
                        size: (json_node.width, json_node.height),
                        node_type: json_node.node_type,
                        id: ReflectableUuid(json_node.id),
                        image,
                        text: json_node.text.text.clone(),
                        pair_bg_color,
                        position: (json_node.x, json_node.y, json_node.z),
                        text_pos: json_node.text.pos,
                        is_active: false,
                    },
                );
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
            let drawings = json["drawings"].as_array_mut().unwrap();
            for drawing in drawings.iter() {
                let drawing_json_node: DrawingJsonNode<String> =
                    serde_json::from_value(drawing.clone()).unwrap();
                let mut path_builder = PathBuilder::new();
                let mut points_iter = drawing_json_node.points.iter();
                let start = points_iter.next().unwrap();
                path_builder.move_to(*start);
                path_builder.line_to(*start);
                for point in points_iter {
                    path_builder.line_to(*point);
                }
                let path = path_builder.build();
                let theme_color = local_theme
                    .as_ref()
                    .unwrap()
                    .get(drawing_json_node.drawing_color.as_str())
                    .unwrap();
                let pair_color = (
                    drawing_json_node.drawing_color,
                    serde_json::from_value(theme_color.clone()).unwrap(),
                );
                commands.spawn((
                    ShapeBundle {
                        path,
                        transform: Transform::from_xyz(
                            drawing_json_node.x,
                            drawing_json_node.y,
                            drawing_json_node.z,
                        ),
                        ..Default::default()
                    },
                    Stroke::new(pair_color.1, 2.),
                    Drawing {
                        id: drawing_json_node.id,
                        points: drawing_json_node.points.clone(),
                        drawing_color: pair_color,
                    },
                    InteractiveNode,
                ));
            }
            break;
        }
    }
}
