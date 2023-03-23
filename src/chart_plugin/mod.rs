use base64::{engine::general_purpose, Engine};
use bevy::{
    ecs::schedule::SystemConfig,
    input::mouse::MouseMotion,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    text::BreakLineOn,
    window::PrimaryWindow,
};
use bevy::{
    reflect::erased_serde::__private::serde::de::DeserializeSeed, scene::serde::SceneDeserializer,
};
#[cfg(not(target_arch = "wasm32"))]
use image::*;
use moonshine_save::{
    load::{self, load, unload},
    prelude::{LoadSet, SaveSet, Unload},
    save::{self, save, Save, Saved},
};
use regex::Regex;
use ron::Deserializer;
use serde_json::{json, Value};
use std::{
    collections::{HashSet, VecDeque},
    convert::TryInto,
    io::Cursor,
    path::PathBuf,
};
use uuid::Uuid;
#[path = "ui_helpers.rs"]
mod ui_helpers;
pub use ron::de::SpannedError as ParseError;
pub use ron::Error as DeserializeError;
pub use ui_helpers::*;

pub struct ChartPlugin;

struct AddRect;

struct RedrawArrow {
    pub id: ReflectableUuid,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct CreateArrow {
    pub start: ArrowConnect,
    pub end: ArrowConnect,
}

const MAX_AMOUNT_OF_CHECKPOINTS: usize = 30;

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
            update_rectangle_pos,
            create_new_rectangle,
            create_entity_event,
            resize_entity_start,
            resize_entity_end,
            create_arrow_start,
            create_arrow_end,
            set_focused_entity,
            redraw_arrows,
            keyboard_input_system,
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

pub fn save_ron_as_checkpoint(
    In(saved): In<Saved>,
    type_registry: Res<AppTypeRegistry>,
    mut state: ResMut<AppState>,
) -> Result<Saved, save::Error> {
    let input = saved.scene.serialize_ron(&type_registry)?;
    let re = Regex::new(r"generation: (\d+)").unwrap();
    // lol
    let input = re.replace_all(&input, "generation: 0").to_string();
    let ron = general_purpose::STANDARD.encode(input);
    if state.checkpoints.len() > MAX_AMOUNT_OF_CHECKPOINTS {
        state.checkpoints.pop_front();
    }
    state.checkpoints.push_back(ron);
    Ok(saved)
}

fn post_save(
    images: Res<Assets<Image>>,
    rec: Query<(&Rectangle, &UiImage), With<Rectangle>>,
    request: Res<SaveRequest>,
    mut state: ResMut<AppState>,
) {
    eprintln!("post save: {:?}", request);
    let ron = state.checkpoints.pop_back().unwrap();
    let mut json = json!({
        "bevy_version": "0.10",
        "images": {},
        "ron": ron,
    });
    let json_images = json["images"].as_object_mut().unwrap();
    for (rect, image) in rec.iter() {
        if let Some(image) = images.get(&image.texture) {
            if let Ok(img) = image.clone().try_into_dynamic() {
                let mut image_data: Vec<u8> = Vec::new();
                #[cfg(not(target_arch = "wasm32"))]
                img.write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
                    .unwrap();
                let res_base64 = general_purpose::STANDARD.encode(image_data);
                json_images.insert(rect.id.0.to_string(), json!(res_base64));
            }
        }
    }

    if let Some(path) = request.path.clone() {
        std::fs::write(path, json.to_string()).expect("Error saving state to file")
    } else {
        state.checkpoints.push_back(json.to_string());
    }
}

fn should_save(request: Option<Res<SaveRequest>>) -> bool {
    request.is_some()
}

fn remove_save_request(world: &mut World) {
    world.remove_resource::<SaveRequest>().unwrap();
}

fn should_load(request: Option<Res<LoadRequest>>) -> bool {
    request.is_some()
}

fn remove_load_request(world: &mut World) {
    world.remove_resource::<LoadRequest>().unwrap();
}

pub fn save_ron() -> SystemConfig {
    save::<With<Save>>
        .pipe(save_ron_as_checkpoint)
        .pipe(save::finish)
        .in_set(SaveSet::Save)
}

pub fn load_ron() -> SystemConfig {
    from_file_or_memory
        .pipe(unload::<Or<(With<Save>, With<Unload>)>>)
        .pipe(load)
        .pipe(load::finish)
        .in_set(LoadSet::Load)
}

fn post_load(
    mut rec: Query<(&Rectangle, &mut UiImage), With<Rectangle>>,
    request: Res<LoadRequest>,
    mut state: ResMut<AppState>,
    mut res_images: ResMut<Assets<Image>>,
) {
    let mut json: Value = match &request.path {
        Some(path) => {
            let json = std::fs::read_to_string(path).expect("Error reading state from file");
            serde_json::from_str(&json).unwrap()
        }
        None => {
            let json = if state.checkpoints.len() == 1 {
                state.checkpoints.back().unwrap().clone()
            } else {
                state.checkpoints.pop_back().unwrap()
            };
            serde_json::from_str(&json).unwrap()
        }
    };
    let images = json["images"].as_object_mut().unwrap();
    for (rect, mut ui_image) in rec.iter_mut() {
        if images.contains_key(&rect.id.0.to_string()) {
            let image = images.get(&rect.id.0.to_string()).unwrap();
            let image_bytes = general_purpose::STANDARD
                .decode(image.as_str().unwrap().as_bytes())
                .unwrap();
            let img = load_from_memory_with_format(&image_bytes, ImageFormat::Png).unwrap();
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
            ui_image.texture = image_handle;
        }
    }
}

pub fn from_file_or_memory(
    type_registry: Res<AppTypeRegistry>,
    request: Res<LoadRequest>,
    mut state: ResMut<AppState>,
) -> Result<Saved, load::Error> {
    eprintln!("load: {:?}", request);

    state.entity_to_edit = None;
    state.hold_entity = None;
    state.entity_to_resize = None;
    state.arrow_to_draw_start = None;

    let ron;
    match &request.path {
        Some(path) => {
            let json_bytes = std::fs::read(path).unwrap();
            let json: Value = serde_json::from_slice(&json_bytes).unwrap();
            ron = json["ron"].as_str().unwrap().to_string();
            state.checkpoints = VecDeque::new();
        }
        None => {
            let json: Value = serde_json::from_str(state.checkpoints.back().unwrap()).unwrap();
            ron = json["ron"].as_str().unwrap().to_string();
        }
    }
    let ron = general_purpose::STANDARD.decode(ron.as_bytes()).unwrap();
    let mut deserializer = Deserializer::from_bytes(&ron)?;
    let scene = {
        let type_registry = &type_registry.read();
        let scene_deserializer = SceneDeserializer { type_registry };
        scene_deserializer.deserialize(&mut deserializer)
    }?;
    Ok(Saved { scene })
}

fn init_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/iosevka-regular.ttf");

    commands.insert_resource(SaveRequest { path: None });

    commands
        .spawn((add_rectangle_btn(), CreateRectButton))
        .with_children(|builder| {
            builder.spawn(add_rectangle_txt(font.clone()));
        });
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
    mut connect_arrow: EventWriter<CreateArrow>,
) {
    for (interaction, arrow_connect) in interaction_query.iter_mut() {
        if let Interaction::Clicked = interaction {
            match state.arrow_to_draw_start {
                Some(start_arrow) => {
                    if start_arrow.id == arrow_connect.id {
                        continue;
                    }
                    state.arrow_to_draw_start = None;
                    connect_arrow.send(CreateArrow {
                        start: start_arrow,
                        end: *arrow_connect,
                    });
                }
                None => {
                    state.arrow_to_draw_start = Some(*arrow_connect);
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
) {
    for _ in events.iter() {
        let font = asset_server.load("fonts/iosevka-regular.ttf");
        spawn_item(
            &mut commands,
            ItemMeta {
                font,
                size: Vec2::new(100., 100.),
                id: ReflectableUuid(Uuid::new_v4()),
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
    let command = input.any_pressed([KeyCode::RWin, KeyCode::LWin]);
    let shift = input.any_pressed([KeyCode::RShift, KeyCode::LShift]);

    if command && input.just_pressed(KeyCode::V) {
        #[cfg(not(target_arch = "wasm32"))]
        insert_from_clipboard(&mut commands, &mut images, &mut state, &mut query);
    } else if command && shift && input.just_pressed(KeyCode::S) {
        commands.insert_resource(SaveRequest {
            path: Some(PathBuf::from("ichart.json")),
        });
    } else if command && shift && input.just_pressed(KeyCode::L) {
        commands.insert_resource(LoadRequest {
            path: Some(PathBuf::from("ichart.json")),
        });
    } else if command && input.just_pressed(KeyCode::S) {
        commands.insert_resource(SaveRequest { path: None });
    } else if command && input.just_pressed(KeyCode::L) {
        commands.insert_resource(LoadRequest { path: None });
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
    let mut clipboard = arboard::Clipboard::new().unwrap();
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
        spawn_item(
            commands,
            ItemMeta {
                font: Handle::default(),
                size: Vec2::new(size.width as f32, size.height as f32),
                id: ReflectableUuid(Uuid::new_v4()),
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
