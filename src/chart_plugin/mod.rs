#[cfg(not(target_arch = "wasm32"))]
use arboard::*;
use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    utils::HashSet,
    window::PrimaryWindow,
};
#[cfg(not(target_arch = "wasm32"))]
use image::*;
use std::convert::TryInto;
#[path = "ui_helpers.rs"]
mod ui_helpers;
pub use ui_helpers::*;

pub struct ChartPlugin;

struct AddRect;

struct RedrawArrow {
    pub id: u32,
}

#[derive(Resource, Default)]
pub struct AppState {
    pub entity_to_edit: Option<u32>,
    pub hold_entity: Option<u32>,
    pub entity_counter: u32,
    pub entity_to_resize: Option<(u32, ResizeMarker)>,
    pub line_to_draw_start: Option<(ArrowConnect, Vec2)>,
}

impl Plugin for ChartPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppState>();

        app.add_event::<AddRect>();
        app.add_event::<RedrawArrow>();

        app.add_startup_system(init_layout);

        app.add_systems((
            update_rectangle_pos,
            create_new_rectangle,
            create_entity_event,
            resize_entity_start,
            resize_entity_end,
            connect_rectangles,
            set_focused_entity,
            redraw_arrows,
            keyboard_input_system,
        ));
    }
}

fn init_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/iosevka-regular.ttf");

    commands
        .spawn((add_rectangle_btn(), CreateRectButton))
        .with_children(|builder| {
            builder.spawn(add_rectangle_txt(font.clone()));
        });
}

fn connect_rectangles(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &ArrowConnect, Entity),
        (Changed<Interaction>, With<ArrowConnect>),
    >,
    mut state: ResMut<AppState>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    labelled: Query<&GlobalTransform>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let primary_window = windows.single_mut();
    let (camera, camera_transform) = camera_q.single();
    for (interaction, arrow_connect, entity) in interaction_query.iter_mut() {
        if let Interaction::Clicked = interaction {
            match state.line_to_draw_start {
                Some(start_arrow) => {
                    if start_arrow.0.id == arrow_connect.id {
                        continue;
                    }
                    if let Ok(global_transform) = labelled.get(entity) {
                        let end = global_transform.affine().translation;
                        let end = Vec2::new(end.x, primary_window.height() - end.y);
                        let start = camera.viewport_to_world_2d(camera_transform, start_arrow.1);
                        let end = camera.viewport_to_world_2d(camera_transform, end);
                        if let (Some(start), Some(end)) = (start, end) {
                            create_arrow(
                                &mut commands,
                                start,
                                end,
                                ArrowMeta {
                                    start: start_arrow.0,
                                    end: *arrow_connect,
                                },
                            );
                            state.line_to_draw_start = None;
                        }
                    }
                }
                None => {
                    if let Ok(global_transform) = labelled.get(entity) {
                        let world_position = global_transform.affine().translation;
                        state.line_to_draw_start = Some((
                            *arrow_connect,
                            Vec2::new(world_position.x, primary_window.height() - world_position.y),
                        ));
                    }
                }
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
                state.entity_to_edit = None;
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
    mut commands: Commands,
    mut events: EventReader<RedrawArrow>,
    mut arrow_query: Query<(Entity, &ArrowMeta), With<ArrowMeta>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    labelled: Query<&GlobalTransform>,
    mut arrow_markers: Query<(Entity, &ArrowConnect), With<ArrowConnect>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let primary_window = windows.single_mut();
    let (camera, camera_transform) = camera_q.single();
    let mut despawned: HashSet<ArrowMeta> = HashSet::new();

    for event in events.iter() {
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
        let mut start = None;
        let mut end = None;
        for (entity, arrow_connect) in &mut arrow_markers.iter_mut() {
            if arrow_connect.id == arrow_meta.start.id && arrow_connect.pos == arrow_meta.start.pos
            {
                if let Ok(global_transform) = labelled.get(entity) {
                    let world_position = global_transform.affine().translation;
                    start = Some(Vec2::new(
                        world_position.x,
                        primary_window.height() - world_position.y,
                    ));
                }
            }
            if arrow_connect.id == arrow_meta.end.id && arrow_connect.pos == arrow_meta.end.pos {
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
                create_arrow(&mut commands, start, end, arrow_meta);
            }
        }
    }
}

fn resize_entity_end(
    mut mouse_motion_events: EventReader<MouseMotion>,
    state: Res<AppState>,
    mut top_query: Query<(&Rectangle, &mut Style), With<Rectangle>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut events: EventWriter<RedrawArrow>,
) {
    let primary_window = windows.single_mut();
    for event in mouse_motion_events.iter() {
        if let Some((id, resize_marker)) = state.entity_to_resize {
            for (rectangle, mut button_style) in &mut top_query {
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
            Interaction::None => {}
        }
    }
}

fn update_rectangle_pos(
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
        state.entity_counter += 1;
        spawn_item(
            &mut commands,
            ItemMeta {
                font,
                size: Vec2::new(100., 100.),
                id: state.entity_counter,
                image: None,
            },
        );
    }
}

fn keyboard_input_system(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut state: ResMut<AppState>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Text, &EditableText), With<EditableText>>,
    mut char_evr: EventReader<ReceivedCharacter>,
) {
    let ctrl = input.any_pressed([KeyCode::RWin, KeyCode::LWin]);

    if ctrl && input.just_pressed(KeyCode::V) {
        #[cfg(not(target_arch = "wasm32"))]
        insert_from_clipboard(&mut commands, &mut images, &mut state, &mut query);
    } else {
        for (mut text, editable_text) in &mut query.iter_mut() {
            if Some(editable_text.id) == state.entity_to_edit {
                if input.just_pressed(KeyCode::Back) {
                    let mut str = text.sections[0].value.clone();
                    str.pop();
                    text.sections[0].value = str;
                } else {
                    for ev in char_evr.iter() {
                        text.sections[0].value = format!("{}{}", text.sections[0].value, ev.char);
                    }
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn insert_from_clipboard(
    commands: &mut Commands,
    images: &mut ResMut<Assets<Image>>,
    state: &mut ResMut<AppState>,
    query: &mut Query<(&mut Text, &EditableText), With<EditableText>>,
) {
    let mut clipboard = Clipboard::new().unwrap();
    if let Ok(image) = clipboard.get_image() {
        let image: RgbaImage = ImageBuffer::from_raw(
            image.width.try_into().unwrap(),
            image.height.try_into().unwrap(),
            image.bytes.into_owned(),
        )
        .unwrap();
        let size: Extent3d = Extent3d {
            width: image.width(),
            height: image.height(),
            ..Default::default()
        };
        let image = Image::new(
            size,
            TextureDimension::D2,
            image.to_vec(),
            TextureFormat::Rgba8UnormSrgb,
        );
        let image = images.add(image);
        state.entity_counter += 1;
        spawn_item(
            commands,
            ItemMeta {
                font: Handle::default(),
                size: Vec2::new(size.width as f32, size.height as f32),
                id: state.entity_counter,
                image: Some(image.into()),
            },
        );
    }

    if let Ok(clipboard_text) = clipboard.get_text() {
        for (mut text, editable_text) in &mut query.iter_mut() {
            if Some(editable_text.id) == state.entity_to_edit {
                text.sections[0].value =
                    format!("{}{}", text.sections[0].value, clipboard_text.clone());
            }
        }
    }
}
