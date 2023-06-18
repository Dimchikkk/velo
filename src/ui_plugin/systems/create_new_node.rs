use bevy::{prelude::*, window::PrimaryWindow};
use bevy_cosmic_edit::CosmicFont;

use crate::{
    resources::{AppState, FontSystemState},
    themes::Theme,
    utils::ReflectableUuid,
};

use super::{ui_helpers::spawn_sprite_node, AddRect, NodeMeta, UiState};

pub fn create_new_node(
    mut commands: Commands,
    mut events: EventReader<AddRect>,
    mut ui_state: ResMut<UiState>,
    app_state: Res<AppState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    font_system_state: ResMut<FontSystemState>,
    theme: Res<Theme>,
    mut z_index_local: Local<f32>,
    mut shaders: ResMut<Assets<Shader>>,
) {
    let window = windows.single_mut();
    for event in events.iter() {
        let current_document = app_state.current_document.unwrap();
        let tab = app_state
            .docs
            .get(&current_document)
            .unwrap()
            .tabs
            .iter()
            .find(|x| x.is_active)
            .unwrap();
        *z_index_local += (tab.z_index + 1.) % f32::MAX;
        *ui_state = UiState::default();
        ui_state.entity_to_edit = Some(ReflectableUuid(event.node.id));
        let _ = spawn_sprite_node(
            &mut shaders,
            &mut commands,
            &theme,
            &mut cosmic_fonts,
            font_system_state.0.clone().unwrap(),
            window.scale_factor() as f32,
            NodeMeta {
                id: ReflectableUuid(event.node.id),
                size: (event.node.width, event.node.height),
                node_type: event.node.node_type.clone(),
                image: event.image.clone(),
                text: event.node.text.text.clone(),
                bg_color: event.node.bg_color,
                position: (event.node.x, event.node.y, *z_index_local),
                text_pos: event.node.text.pos.clone(),
                is_active: true,
            },
        );
    }
}
