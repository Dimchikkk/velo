use bevy::{prelude::*, window::PrimaryWindow};
use bevy_cosmic_edit::CosmicFont;

use crate::{resources::FontSystemState, themes::Theme, utils::ReflectableUuid};

use super::{ui_helpers::spawn_sprite_node, AddRectEvent, NodeMeta, UiState};

pub fn create_new_node(
    mut commands: Commands,
    mut events: EventReader<AddRectEvent>,
    mut ui_state: ResMut<UiState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    font_system_state: ResMut<FontSystemState>,
    theme: Res<Theme>,
    mut z_index_local: Local<i32>,
    mut shaders: ResMut<Assets<Shader>>,
) {
    let window = windows.single_mut();
    for event in events.iter() {
        *z_index_local += 1;
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
                size: (200., 200.),
                // size: (event.node.width, event.node.height),
                id: ReflectableUuid(event.node.id),
                // node_type: event.node.node_type.clone(),
                node_type: super::NodeType::Paper,
                image: event.image.clone(),
                text: event.node.text.text.clone(),
                // bg_color: event.node.bg_color,
                bg_color: Color::rgb(255. / 255., 236. / 255., 172. / 255.),
                position: (event.node.left, event.node.bottom),
                text_pos: event.node.text.pos.clone(),
                z_index: *z_index_local as i32,
                is_active: true,
            },
        );
    }
}
