use bevy::{prelude::*, text::BreakLineOn, window::PrimaryWindow};
use serde::{Deserialize, Serialize};

use std::{collections::VecDeque, path::PathBuf};
use uuid::Uuid;

#[path = "ui_helpers/ui_helpers.rs"]
mod ui_helpers;
use ui_helpers::*;
#[path = "systems/save.rs"]
mod save_systems;
use save_systems::*;
#[path = "systems/load.rs"]
mod load_systems;
use load_systems::*;
#[path = "systems/keyboard.rs"]
mod keyboard_systems;
use keyboard_systems::*;
#[path = "systems/path_modal.rs"]
mod path_modal_systems;
use path_modal_systems::*;
#[path = "systems/init_layout.rs"]
mod init_layout;
use init_layout::*;
#[path = "systems/resize.rs"]
mod resize;
use resize::*;
#[path = "systems/arrows.rs"]
mod arrows;
use arrows::*;

pub struct ChartPlugin;

pub struct AddRect {
    pub node: JsonNode,
    pub image: Option<UiImage>,
}

pub struct SetWindowIcon {
    pub image: Handle<Image>,
}

pub struct RedrawArrow {
    pub id: ReflectableUuid,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource, Debug)]
pub struct SaveRequest {
    pub path: Option<PathBuf>,
}

#[derive(Resource, Debug)]
pub struct LoadRequest {
    pub path: Option<PathBuf>,
}

#[derive(Serialize, Deserialize)]
pub enum NodeType {
    RECT,
}

#[derive(Serialize, Deserialize)]
pub struct JsonNode {
    pub id: Uuid,
    pub node_type: NodeType,
    pub left: Val,
    pub bottom: Val,
    pub width: Val,
    pub height: Val,
    pub text: String,
    pub bg_color: Color,
}

#[derive(Resource, Default)]
pub struct AppState {
    pub path_modal_id: Option<ReflectableUuid>,
    pub entity_to_edit: Option<ReflectableUuid>,
    pub hold_entity: Option<ReflectableUuid>,
    pub entity_to_resize: Option<(ReflectableUuid, ResizeMarker)>,
    pub arrow_to_draw_start: Option<ArrowConnect>,
    pub checkpoints: VecDeque<String>,
    pub main_panel: Option<Entity>,
}

impl Plugin for ChartPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppState>();

        app.register_type::<Rectangle>();
        app.register_type::<EditableText>();
        app.register_type::<ArrowConnect>();
        app.register_type::<ResizeMarker>();
        app.register_type::<ReflectableUuid>();
        app.register_type_data::<ReflectableUuid, ReflectSerialize>();
        app.register_type_data::<ReflectableUuid, ReflectDeserialize>();
        app.register_type::<ArrowConnectPos>();

        app.register_type::<BreakLineOn>();

        app.add_event::<AddRect>();
        app.add_event::<SetWindowIcon>();
        app.add_event::<CreateArrow>();
        app.add_event::<RedrawArrow>();

        app.add_startup_system(init_layout);

        app.add_systems((
            update_rectangle_position,
            create_new_rectangle,
            create_entity_event,
            resize_entity_start,
            resize_entity_end,
            create_arrow_start,
            create_arrow_end,
            set_focused_entity,
            redraw_arrows,
            keyboard_input_system,
            cancel_path_modal,
            path_modal_keyboard_input_system,
            set_focused_modal,
            confirm_path_modal,
            open_path_modal,
        ));

        app.add_systems(
            (save_json, remove_save_request)
                .chain()
                .distributive_run_if(should_save),
        );

        app.add_systems(
            (load_json, remove_load_request)
                .chain()
                .distributive_run_if(should_load),
        );

        app.add_system(delete_entity);
    }
}

fn create_entity_event(
    mut events: EventWriter<AddRect>,
    interaction_query: Query<
        (&Interaction, &CreateRectButton),
        (Changed<Interaction>, With<CreateRectButton>),
    >,
) {
    for (interaction, _) in &interaction_query {
        match *interaction {
            Interaction::Clicked => {
                events.send(AddRect {
                    node: JsonNode {
                        id: Uuid::new_v4(),
                        node_type: NodeType::RECT,
                        left: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                        width: Val::Px(100.0),
                        height: Val::Px(100.0),
                        text: "".to_string(),
                        bg_color: Color::WHITE,
                    },
                    image: None,
                });
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn set_focused_entity(
    mut interaction_query: Query<
        (&Interaction, &Rectangle),
        (Changed<Interaction>, With<Rectangle>),
    >,
    mut state: ResMut<AppState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
) {
    let mut window = windows.single_mut();
    for (interaction, rectangle) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                window.cursor.icon = CursorIcon::Text;
                state.hold_entity = Some(rectangle.id);
                state.entity_to_edit = Some(rectangle.id);
            }
            Interaction::Hovered => {
                if state.hold_entity.is_none() {
                    window.cursor.icon = CursorIcon::Move;
                }
            }
            Interaction::None => {
                window.cursor.icon = CursorIcon::Default;
            }
        }
    }
    if buttons.just_released(MouseButton::Left) {
        state.hold_entity = None;
        state.entity_to_resize = None;
    }
}

fn update_rectangle_position(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut node_position: Query<(&mut Style, &Rectangle), With<Rectangle>>,
    state: Res<AppState>,
    mut query: Query<(&Style, &LeftPanel), Without<Rectangle>>,
    mut events: EventWriter<RedrawArrow>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let primary_window = windows.single();
    for event in cursor_moved_events.iter() {
        for (mut style, top) in &mut node_position.iter_mut() {
            if Some(top.id) == state.hold_entity {
                let size = query.single_mut().0.size;
                if let (Val::Percent(x), Val::Px(element_width)) = (size.width, style.size.width) {
                    let width = (primary_window.width() * x) / 100.;
                    style.position.left = Val::Px(event.position.x - width - element_width / 2.);
                }
                if let Val::Px(element_height) = style.size.height {
                    style.position.bottom = Val::Px(event.position.y - element_height / 2.);
                }
                events.send(RedrawArrow { id: top.id });
            }
        }
    }
}

fn create_new_rectangle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<AddRect>,
    mut state: ResMut<AppState>,
) {
    for event in events.iter() {
        let font = asset_server.load("fonts/iosevka-regular.ttf");
        state.entity_to_edit = Some(ReflectableUuid(event.node.id));
        let entity = spawn_node(
            &mut commands,
            NodeMeta {
                font,
                size: (event.node.width, event.node.height),
                id: ReflectableUuid(event.node.id),
                image: event.image.clone(),
                text: event.node.text.clone(),
                bg_color: event.node.bg_color,
                position: (event.node.left, event.node.bottom),
            },
        );
        commands.entity(state.main_panel.unwrap()).add_child(entity);
    }
}

fn delete_entity(
    mut commands: Commands,
    mut state: ResMut<AppState>,
    interaction_query: Query<
        (&Interaction, &DelRectButton),
        (Changed<Interaction>, With<DelRectButton>),
    >,
    nodes: Query<(Entity, &Rectangle), With<Rectangle>>,
    arrows: Query<(Entity, &ArrowMeta), With<ArrowMeta>>,
) {
    for (interaction, _) in &interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if let Some(id) = state.entity_to_edit {
                    state.entity_to_edit = None;
                    state.entity_to_resize = None;
                    state.hold_entity = None;
                    state.arrow_to_draw_start = None;
                    for (entity, node) in nodes.iter() {
                        if node.id == id {
                            commands.entity(entity).despawn_recursive();
                        }
                    }
                    for (entity, arrow) in arrows.iter() {
                        if arrow.start.id == id || arrow.end.id == id {
                            commands.entity(entity).despawn_recursive();
                        }
                    }
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
