use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
mod image_from_clipboard;
#[cfg(not(target_arch = "wasm32"))]
pub use image_from_clipboard::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct IRectangle {
    id: u32,
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IChartState>();

        app.add_systems((
            update_rectangle_pos,
            update_text_on_typing,
            create_new_rectangle,
            set_focused_entity,
        ));

        #[cfg(not(target_arch = "wasm32"))]
        app.add_system(insert_image_from_clipboard);
    }
}

#[derive(Resource, Default)]
struct IChartState {
    focused_id: Option<u32>,
}

fn set_focused_entity(
    mut interaction_query: Query<(&Interaction, &IRectangle), (Changed<Interaction>, With<IRectangle>)>,
    mut state: ResMut<IChartState>,
    keys: Res<Input<KeyCode>>,
) {
    for (interaction, irectangle) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state.focused_id = Some(irectangle.id);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        state.focused_id = None;
    }
}

fn update_rectangle_pos(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut sprite_position: Query<(&mut Style, &Top)>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    state: Res<IChartState>,
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

#[derive(Component)]
struct EditableText {
    id: u32,
}

#[derive(Component)]
struct Top {
    id: u32,
}

fn update_text_on_typing(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Text, &EditableText), With<EditableText>>,
    state: Res<IChartState>,
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

#[derive(Default)]
struct Counter {
    count: u32,
}

fn create_new_rectangle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut local_state: Local<Counter>,
    mut state: ResMut<IChartState>,
) {
    let (camera, camera_transform) = camera_q.single();
    let mut x = 0.0;
    let mut y = 0.0;
    for event in cursor_moved_events.iter() {
        if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, event.position)
        {
            x = world_position.x;
            y = world_position.y;
        }
    }
    if mouse_button_input.just_pressed(MouseButton::Right) {
        local_state.count += 1;
        state.focused_id = Some(local_state.count);
        let font = asset_server.load("fonts/SourceCodePro-Regular.ttf");
        let text_style = TextStyle {
            font,
            font_size: 18.0,
            color: Color::BLACK,
        };
        let box_size = Vec2::new(200.0, 200.0);
        // Rectangle
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        position: UiRect {
                            left: Val::Px(x),
                            bottom: Val::Px(y),
                            ..Default::default()
                        },
                        position_type: PositionType::Absolute,
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                Top { id: local_state.count },
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
                        IRectangle { id: local_state.count },
                    ))
                    .with_children(|builder| {
                        builder.spawn((
                            TextBundle::from_section("", text_style.clone()).with_style(Style {
                                position_type: PositionType::Relative,
                                ..default()
                            }),
                            EditableText { id: local_state.count },
                        ));
                    });
            });
    }
}
