#[cfg(not(target_arch = "wasm32"))]
use arboard::*;
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    window::PrimaryWindow,
};
#[cfg(not(target_arch = "wasm32"))]
use image::*;
use std::convert::TryInto;
#[path = "structs.rs"]
mod structs;
pub use structs::*;
#[path = "ui_helpers.rs"]
mod ui_helpers;
pub use ui_helpers::*;

pub struct ChartPlugin;

impl Plugin for ChartPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppState>();

        app.add_event::<AddRect>();

        app.add_startup_system(init_layout);

        app.add_systems((
            update_rectangle_pos,
            update_text_on_typing,
            create_new_rectangle,
            create_entity_event,
            resize_entity_start,
            resize_entity_end,
        ));

        #[cfg(not(target_arch = "wasm32"))]
        app.add_system(insert_image_from_clipboard);

        app.add_system(set_focused_entity);
    }
}

fn init_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/SourceCodePro-Regular.ttf");
    let text_style = TextStyle {
        font,
        font_size: 18.0,
        color: Color::BLACK,
    };
    commands
        .spawn((
            ButtonBundle {
                z_index: ZIndex::Global(1),
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(10.),
                        top: Val::Px(10.),
                        ..Default::default()
                    },
                    size: Size::new(Val::Px(100.), Val::Px(100.)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            CreateRectButton,
        ))
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section("NEW RECT", text_style.clone()).with_style(Style {
                    position_type: PositionType::Relative,
                    ..default()
                }),
            ));
        });
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
) {
    for (interaction, rectangle) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if state.focused_id == Some(rectangle.id) {
                    state.focused_id = None;
                } else {
                    state.focused_id = Some(rectangle.id);
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn resize_entity_end(
    buttons: Res<Input<MouseButton>>,
    mut state: ResMut<AppState>,
    mut top_query: Query<(&Rectangle, &mut Style), With<Rectangle>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let primary_window = windows.single_mut();
    let (camera, camera_transform) = camera_q.single();

    if buttons.just_released(MouseButton::Left) {
        for (rectangle, mut button_style) in &mut top_query {
            match state.entity_to_resize {
                Some((_id, _prev_cursor_pos, _resize_marker)) => {
                    let (id, prev_cursor_pos, resize_marker) = state.entity_to_resize.unwrap();
                    let current_cursor_pos = primary_window.cursor_position();
                    if id == rectangle.id && current_cursor_pos.is_some() {
                        if let Some(world_position) = camera
                            .viewport_to_world_2d(camera_transform, current_cursor_pos.unwrap())
                        {
                            let delta = world_position - prev_cursor_pos;
                            match resize_marker {
                                ResizeMarker::TopLeft => {
                                    match button_style.size.width {
                                        Val::Px(width) => {
                                            button_style.size.width = Val::Px(
                                                width
                                                    - (primary_window.resolution.scale_factor()
                                                        as f32
                                                        * delta.x),
                                            );
                                        }
                                        _ => {}
                                    }
                                    match button_style.size.height {
                                        Val::Px(height) => {
                                            button_style.size.height = Val::Px(
                                                height
                                                    + (primary_window.resolution.scale_factor()
                                                        as f32
                                                        * delta.y),
                                            );
                                        }
                                        _ => {}
                                    }
                                }
                                ResizeMarker::TopRight => {
                                    match button_style.size.width {
                                        Val::Px(width) => {
                                            button_style.size.width = Val::Px(
                                                width
                                                    + (primary_window.resolution.scale_factor()
                                                        as f32
                                                        * delta.x),
                                            );
                                        }
                                        _ => {}
                                    }
                                    match button_style.size.height {
                                        Val::Px(height) => {
                                            button_style.size.height = Val::Px(
                                                height
                                                    + (primary_window.resolution.scale_factor()
                                                        as f32
                                                        * delta.y),
                                            );
                                        }
                                        _ => {}
                                    }
                                }
                                ResizeMarker::BottomLeft => {
                                    match button_style.size.width {
                                        Val::Px(width) => {
                                            button_style.size.width = Val::Px(
                                                width
                                                    - (primary_window.resolution.scale_factor()
                                                        as f32
                                                        * delta.x),
                                            );
                                        }
                                        _ => {}
                                    }
                                    match button_style.size.height {
                                        Val::Px(height) => {
                                            button_style.size.height = Val::Px(
                                                height
                                                    - (primary_window.resolution.scale_factor()
                                                        as f32
                                                        * delta.y),
                                            );
                                        }
                                        _ => {}
                                    }
                                }
                                ResizeMarker::BottomRight => {
                                    match button_style.size.width {
                                        Val::Px(width) => {
                                            button_style.size.width = Val::Px(
                                                width
                                                    + (primary_window.resolution.scale_factor()
                                                        as f32
                                                        * delta.x),
                                            );
                                        }
                                        _ => {}
                                    }
                                    match button_style.size.height {
                                        Val::Px(height) => {
                                            button_style.size.height = Val::Px(
                                                height
                                                    - (primary_window.resolution.scale_factor()
                                                        as f32
                                                        * delta.y),
                                            );
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        state.entity_to_resize = None;
                    }
                }
                None => {}
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
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let primary_window = windows.single_mut();
    let (camera, camera_transform) = camera_q.single();
    for (interaction, parent, resize_marker) in &mut interaction_query {
        let rectangle = button_query.get_mut(parent.get()).unwrap();
        match *interaction {
            Interaction::Clicked => {
                if primary_window.cursor_position().is_none() {
                    return;
                }
                if let Some(world_position) = camera.viewport_to_world_2d(
                    camera_transform,
                    primary_window.cursor_position().unwrap(),
                ) {
                    state.entity_to_resize = Some((rectangle.id, world_position, *resize_marker));
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn update_rectangle_pos(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut sprite_position: Query<(&mut Style, &Top)>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    state: Res<AppState>,
) {
    if state.focused_id.is_none() {
        return;
    }
    let (camera, camera_transform) = camera_q.single();
    for event in cursor_moved_events.iter() {
        for (mut style, top) in &mut sprite_position.iter_mut() {
            if Some(top.id) == state.focused_id {
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

fn update_text_on_typing(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Text, &EditableText), With<EditableText>>,
    state: Res<AppState>,
) {
    if state.focused_id.is_none() {
        return;
    }
    for (mut text, editable_text) in &mut query.iter_mut() {
        if Some(editable_text.id) == state.focused_id {
            if keys.just_pressed(KeyCode::Back) {
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

fn create_new_rectangle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<AddRect>,
    mut state: ResMut<AppState>,
) {
    for _ in events.iter() {
        state.entity_counter += 1;
        let font = asset_server.load("fonts/SourceCodePro-Regular.ttf");
        let text_style = TextStyle {
            font,
            font_size: 18.0,
            color: Color::BLACK,
        };
        let box_size = Vec2::new(100.0, 100.0);
        // Rectangle
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                            ..Default::default()
                        },
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                Top {
                    id: state.entity_counter,
                },
            ))
            .with_children(|builder| {
                builder
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(box_size.x), Val::Px(box_size.y)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        },
                        Rectangle {
                            id: state.entity_counter,
                        },
                    ))
                    .with_children(|builder| {
                        builder.spawn((
                            ButtonBundle {
                                style: get_marker_style(UiRect {
                                    left: Val::Percent(50.),
                                    right: Val::Percent(0.),
                                    top: Val::Percent(0.),
                                    bottom: Val::Percent(0.),
                                }),
                                background_color: Color::rgb(0.9, 0.9, 1.0).into(),
                                ..default()
                            },
                            ArrowConnectMarker,
                        ));
                        builder.spawn((
                            ButtonBundle {
                                style: get_marker_style(UiRect {
                                    left: Val::Percent(0.),
                                    right: Val::Percent(0.),
                                    top: Val::Percent(50.),
                                    bottom: Val::Percent(0.),
                                }),
                                background_color: Color::rgb(0.9, 0.9, 1.0).into(),
                                ..default()
                            },
                            ArrowConnectMarker,
                        ));
                        builder.spawn((
                            ButtonBundle {
                                style: get_marker_style(UiRect {
                                    left: Val::Percent(50.),
                                    right: Val::Percent(0.),
                                    top: Val::Percent(100.),
                                    bottom: Val::Percent(0.),
                                }),
                                background_color: Color::rgb(0.9, 0.9, 1.0).into(),
                                ..default()
                            },
                            ArrowConnectMarker,
                        ));
                        builder.spawn((
                            ButtonBundle {
                                style: get_marker_style(UiRect {
                                    left: Val::Percent(100.),
                                    right: Val::Percent(0.),
                                    top: Val::Percent(50.),
                                    bottom: Val::Percent(0.),
                                }),
                                background_color: Color::rgb(0.9, 0.9, 1.0).into(),
                                ..default()
                            },
                            ArrowConnectMarker,
                        ));
                        builder.spawn((
                            ButtonBundle {
                                style: get_marker_style(UiRect {
                                    left: Val::Percent(0.),
                                    right: Val::Percent(0.),
                                    top: Val::Percent(0.),
                                    bottom: Val::Percent(0.),
                                }),
                                background_color: Color::rgb(0.8, 0.8, 1.0).into(),
                                ..default()
                            },
                            ResizeMarker::TopLeft,
                        ));
                        builder.spawn((
                            ButtonBundle {
                                style: get_marker_style(UiRect {
                                    left: Val::Percent(100.),
                                    right: Val::Percent(0.),
                                    top: Val::Percent(0.),
                                    bottom: Val::Percent(0.),
                                }),
                                background_color: Color::rgb(0.8, 0.8, 1.0).into(),
                                ..default()
                            },
                            ResizeMarker::TopRight,
                        ));
                        builder.spawn((
                            ButtonBundle {
                                style: get_marker_style(UiRect {
                                    left: Val::Percent(100.),
                                    right: Val::Percent(0.),
                                    top: Val::Percent(100.),
                                    bottom: Val::Percent(0.),
                                }),
                                background_color: Color::rgb(0.8, 0.8, 1.0).into(),
                                ..default()
                            },
                            ResizeMarker::BottomRight,
                        ));
                        builder.spawn((
                            ButtonBundle {
                                style: get_marker_style(UiRect {
                                    left: Val::Percent(0.),
                                    right: Val::Percent(0.),
                                    top: Val::Percent(100.),
                                    bottom: Val::Percent(0.),
                                }),
                                background_color: Color::rgb(0.8, 0.8, 1.0).into(),
                                ..default()
                            },
                            ResizeMarker::BottomLeft,
                        ));
                        builder.spawn((
                            TextBundle::from_section("", text_style.clone()).with_style(Style {
                                position_type: PositionType::Relative,
                                ..default()
                            }),
                            EditableText {
                                id: state.entity_counter,
                            },
                        ));
                    });
            });
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn insert_image_from_clipboard(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut state: ResMut<AppState>,
) {
    let mut clipboard = Clipboard::new().unwrap();
    match clipboard.get_image() {
        Ok(image) => {
            state.entity_counter += 1;
            clipboard.clear().unwrap();
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
            commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },
                    Top {
                        id: state.entity_counter,
                    },
                ))
                .with_children(|builder| {
                    builder
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    size: Size::new(
                                        Val::Px(size.width as f32),
                                        Val::Px(size.height as f32),
                                    ),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            Rectangle {
                                id: state.entity_counter,
                            },
                        ))
                        .with_children(|builder| {
                            builder.spawn(ImageBundle {
                                image: image.into(),
                                style: Style {
                                    position_type: PositionType::Relative,
                                    size: Size {
                                        width: Val::Px(size.width as f32),
                                        height: Val::Px(size.height as f32),
                                    },
                                    ..default()
                                },
                                ..Default::default()
                            });
                        });
                });
        }
        Err(_) => {}
    }
}
