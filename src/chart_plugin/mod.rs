use bevy::{input::mouse::MouseMotion, prelude::*, text::BreakLineOn, window::PrimaryWindow};
use moonshine_save::prelude::{LoadSet, SaveSet};

pub use ron::de::SpannedError as ParseError;
pub use ron::Error as DeserializeError;
use std::{
    collections::{HashSet, VecDeque},
    path::PathBuf,
};
use uuid::Uuid;

#[path = "ui_helpers.rs"]
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

pub struct ChartPlugin;

struct AddRect;

struct RedrawArrow {
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

#[derive(Resource, Default)]
pub struct AppState {
    pub path_modal_id: Option<ReflectableUuid>,
    pub entity_to_edit: Option<ReflectableUuid>,
    pub hold_entity: Option<ReflectableUuid>,
    pub entity_to_resize: Option<(ReflectableUuid, ResizeMarker)>,
    pub arrow_to_draw_start: Option<ArrowConnect>,
    pub checkpoints: VecDeque<String>,
}

impl Plugin for ChartPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppState>();

        app.register_type::<Rectangle>();
        app.register_type::<EditableText>();
        app.register_type::<Top>();
        app.register_type::<ArrowConnect>();
        app.register_type::<ResizeMarker>();
        app.register_type::<ReflectableUuid>();
        app.register_type_data::<ReflectableUuid, ReflectSerialize>();
        app.register_type_data::<ReflectableUuid, ReflectDeserialize>();
        app.register_type::<ArrowConnectPos>();

        app.register_type::<BreakLineOn>();

        app.add_event::<AddRect>();
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
        ));

        app.add_systems(
            (
                save_ron(),
                post_save.in_set(SaveSet::PostSave),
                remove_save_request,
            )
                .chain()
                .distributive_run_if(should_save),
        );

        app.add_systems(
            (
                load_ron(),
                post_load.in_set(LoadSet::PostLoad),
                remove_load_request,
            )
                .chain()
                .distributive_run_if(should_load),
        );
    }
}

fn create_arrow_end(
    mut commands: Commands,
    mut events: EventReader<CreateArrow>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    labelled: Query<&GlobalTransform>,
    mut arrow_markers: Query<(Entity, &ArrowConnect), With<ArrowConnect>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let primary_window = windows.single_mut();
    let (camera, camera_transform) = camera_q.single();
    for event in events.iter() {
        let mut start = None;
        let mut end = None;
        for (entity, arrow_connect) in &mut arrow_markers.iter_mut() {
            if *arrow_connect == event.start {
                if let Ok(global_transform) = labelled.get(entity) {
                    let world_position = global_transform.affine().translation;
                    start = Some(Vec2::new(
                        world_position.x,
                        primary_window.height() - world_position.y,
                    ));
                }
            }
            if *arrow_connect == event.end {
                if let Ok(global_transform) = labelled.get(entity) {
                    let world_position = global_transform.affine().translation;
                    end = Some(Vec2::new(
                        world_position.x,
                        primary_window.height() - world_position.y,
                    ));
                }
            }
        }

        if let (Some(start), Some(end)) = (start, end) {
            let start = camera.viewport_to_world_2d(camera_transform, start);
            let end = camera.viewport_to_world_2d(camera_transform, end);
            if let (Some(start), Some(end)) = (start, end) {
                create_arrow(
                    &mut commands,
                    start,
                    end,
                    ArrowMeta {
                        start: event.start,
                        end: event.end,
                    },
                );
            }
        }
    }
}

fn create_arrow_start(
    mut interaction_query: Query<
        (&Interaction, &ArrowConnect),
        (Changed<Interaction>, With<ArrowConnect>),
    >,
    mut state: ResMut<AppState>,
    mut create_arrow: EventWriter<CreateArrow>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = windows.single_mut();
    for (interaction, arrow_connect) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => match state.arrow_to_draw_start {
                Some(start_arrow) => {
                    if start_arrow.id == arrow_connect.id {
                        continue;
                    }
                    state.arrow_to_draw_start = None;
                    create_arrow.send(CreateArrow {
                        start: start_arrow,
                        end: *arrow_connect,
                    });
                }
                None => {
                    state.arrow_to_draw_start = Some(*arrow_connect);
                }
            },
            Interaction::Hovered => {
                primary_window.cursor.icon = CursorIcon::Crosshair;
            }
            Interaction::None => {
                primary_window.cursor.icon = CursorIcon::Default;
            }
        }
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
                events.send(AddRect);
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

fn redraw_arrows(
    mut redraw_arrow: EventReader<RedrawArrow>,
    mut create_arrow: EventWriter<CreateArrow>,
    mut arrow_query: Query<(Entity, &ArrowMeta), With<ArrowMeta>>,
    mut commands: Commands,
) {
    let mut despawned: HashSet<ArrowMeta> = HashSet::new();

    for event in redraw_arrow.iter() {
        for (entity, arrow) in &mut arrow_query.iter_mut() {
            if despawned.contains(arrow) {
                continue;
            }
            if arrow.start.id == event.id || arrow.end.id == event.id {
                if let Some(entity) = commands.get_entity(entity) {
                    despawned.insert(*arrow);
                    entity.despawn_recursive();
                }
            }
        }
    }

    for arrow_meta in despawned {
        create_arrow.send(CreateArrow {
            start: arrow_meta.start,
            end: arrow_meta.end,
        });
    }
}

fn resize_entity_end(
    mut mouse_motion_events: EventReader<MouseMotion>,
    state: Res<AppState>,
    mut rectangle_query: Query<(&Rectangle, &mut Style), With<Rectangle>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut events: EventWriter<RedrawArrow>,
) {
    let primary_window = windows.single_mut();
    for event in mouse_motion_events.iter() {
        if let Some((id, resize_marker)) = state.entity_to_resize {
            for (rectangle, mut button_style) in &mut rectangle_query {
                if id == rectangle.id {
                    events.send(RedrawArrow { id });
                    let delta = event.delta;
                    match resize_marker {
                        ResizeMarker::TopLeft => {
                            if let Val::Px(width) = button_style.size.width {
                                let scale_factor = primary_window.resolution.scale_factor() as f32;
                                button_style.size.width = Val::Px(width - scale_factor * delta.x);
                            }

                            if let Val::Px(height) = button_style.size.height {
                                let scale_factor = primary_window.resolution.scale_factor() as f32;
                                button_style.size.height = Val::Px(height - scale_factor * delta.y);
                            }
                        }
                        ResizeMarker::TopRight => {
                            if let Val::Px(width) = button_style.size.width {
                                let scale_factor = primary_window.resolution.scale_factor() as f32;
                                button_style.size.width = Val::Px(width + scale_factor * delta.x);
                            }

                            if let Val::Px(height) = button_style.size.height {
                                let scale_factor = primary_window.resolution.scale_factor() as f32;
                                button_style.size.height = Val::Px(height - scale_factor * delta.y);
                            }
                        }
                        ResizeMarker::BottomLeft => {
                            if let Val::Px(width) = button_style.size.width {
                                let scale_factor = primary_window.resolution.scale_factor() as f32;
                                button_style.size.width = Val::Px(width - scale_factor * delta.x);
                            }

                            if let Val::Px(height) = button_style.size.height {
                                let scale_factor = primary_window.resolution.scale_factor() as f32;
                                button_style.size.height = Val::Px(height + scale_factor * delta.y);
                            }
                        }
                        ResizeMarker::BottomRight => {
                            if let Val::Px(width) = button_style.size.width {
                                let scale_factor = primary_window.resolution.scale_factor() as f32;
                                button_style.size.width = Val::Px(width + scale_factor * delta.x);
                            }

                            if let Val::Px(height) = button_style.size.height {
                                let scale_factor = primary_window.resolution.scale_factor() as f32;
                                button_style.size.height = Val::Px(height + scale_factor * delta.y);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn resize_entity_start(
    mut interaction_query: Query<
        (&Interaction, &Parent, &ResizeMarker),
        (Changed<Interaction>, With<ResizeMarker>),
    >,
    mut button_query: Query<&Rectangle, With<Rectangle>>,
    mut state: ResMut<AppState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = windows.single_mut();
    for (interaction, parent, resize_marker) in &mut interaction_query {
        let rectangle = button_query.get_mut(parent.get()).unwrap();
        match *interaction {
            Interaction::Clicked => {
                state.entity_to_resize = Some((rectangle.id, *resize_marker));
            }
            Interaction::Hovered => match *resize_marker {
                ResizeMarker::TopLeft => {
                    primary_window.cursor.icon = CursorIcon::NwseResize;
                }
                ResizeMarker::TopRight => {
                    primary_window.cursor.icon = CursorIcon::NeswResize;
                }
                ResizeMarker::BottomLeft => {
                    primary_window.cursor.icon = CursorIcon::NeswResize;
                }
                ResizeMarker::BottomRight => {
                    primary_window.cursor.icon = CursorIcon::NwseResize;
                }
            },
            Interaction::None => {
                primary_window.cursor.icon = CursorIcon::Default;
            }
        }
    }
}

fn update_rectangle_position(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut sprite_position: Query<(&mut Style, &Top)>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    state: ResMut<AppState>,
    mut events: EventWriter<RedrawArrow>,
) {
    let (camera, camera_transform) = camera_q.single();
    for event in cursor_moved_events.iter() {
        for (mut style, top) in &mut sprite_position.iter_mut() {
            if Some(top.id) == state.hold_entity {
                events.send(RedrawArrow { id: top.id });
                if let Some(world_position) =
                    camera.viewport_to_world_2d(camera_transform, event.position)
                {
                    style.position.left = Val::Px(world_position.x);
                    style.position.bottom = Val::Px(world_position.y);
                }
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
    for _ in events.iter() {
        let font = asset_server.load("fonts/iosevka-regular.ttf");
        let id = ReflectableUuid(Uuid::new_v4());
        state.entity_to_edit = Some(id);
        spawn_node(
            &mut commands,
            NodeMeta {
                font,
                size: Vec2::new(100., 100.),
                id,
                image: None,
            },
        );
    }
}
