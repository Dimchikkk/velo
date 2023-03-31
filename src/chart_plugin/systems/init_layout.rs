use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_ui_borders::BorderColor;
use uuid::Uuid;

use crate::{AppState, MainCamera, SaveRequest, Tab, TextPos};

use super::ui_helpers::{
    self, add_rectangle_txt, create_rectangle_txt, AddTab, ArrowMode, ArrowType, BottomPanel,
    ButtonAction, ChangeColor, LeftPanel, LeftPanelControls, LeftPanelExplorer, LoadState,
    MainPanel, Menu, ReflectableUuid, Root, SaveState, SelectedTab, TextPodMode,
};

pub fn init_layout(
    mut commands: Commands,
    mut state: ResMut<AppState>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/iosevka-regular.ttf");
    commands.spawn((Camera2dBundle::default(), MainCamera));
    let tab_id = ReflectableUuid(Uuid::new_v4());
    state.tabs.push(Tab {
        id: tab_id,
        name: "Tab 1".to_string(),
        checkpoints: VecDeque::new(),
        is_active: true,
    });
    commands.insert_resource(SaveRequest {
        path: None,
        tab_id: Some(tab_id),
    });

    let root_ui = commands
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
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            Root,
        ))
        .id();

    let menu = commands
        .spawn((
            NodeBundle {
                background_color: Color::rgb(0.64, 0.64, 0.64).into(),
                style: Style {
                    border: UiRect::all(Val::Px(2.0)),
                    size: Size::new(Val::Percent(100.0), Val::Percent(5.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .id();

    let save = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                style: Style {
                    justify_content: JustifyContent::Center,
                    margin: UiRect {
                        left: Val::Px(10.),
                        right: Val::Px(0.),
                        top: Val::Px(0.),
                        bottom: Val::Px(0.),
                    },
                    padding: UiRect {
                        left: Val::Px(5.),
                        right: Val::Px(5.),
                        top: Val::Px(5.),
                        bottom: Val::Px(5.),
                    },
                    align_items: AlignItems::Center,
                    // overflow: Overflow::Hidden,
                    ..default()
                },
                ..default()
            },
            SaveState,
        ))
        .with_children(|builder| {
            builder.spawn(create_rectangle_txt(font.clone(), "Save".to_string()));
        })
        .id();
    let load = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                style: Style {
                    justify_content: JustifyContent::Center,
                    margin: UiRect {
                        left: Val::Px(10.),
                        right: Val::Px(0.),
                        top: Val::Px(0.),
                        bottom: Val::Px(0.),
                    },
                    padding: UiRect {
                        left: Val::Px(5.),
                        right: Val::Px(5.),
                        top: Val::Px(5.),
                        bottom: Val::Px(5.),
                    },
                    align_items: AlignItems::Center,
                    // overflow: Overflow::Hidden,
                    ..default()
                },
                ..default()
            },
            LoadState,
        ))
        .with_children(|builder| {
            builder.spawn(create_rectangle_txt(font.clone(), "Load".to_string()));
        })
        .id();
    commands.entity(menu).add_child(save);
    commands.entity(menu).add_child(load);

    let main_bottom = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(95.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .id();
    let left_panel = commands
        .spawn((
            NodeBundle {
                background_color: BackgroundColor(Color::Rgba {
                    red: 192. / 255.,
                    green: 192. / 255.,
                    blue: 192. / 255.,
                    alpha: 0.5,
                }),
                style: Style {
                    size: Size::new(Val::Percent(15.), Val::Percent(100.)),
                    align_items: AlignItems::Start,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            LeftPanel,
        ))
        .id();
    let right_panel = commands
        .spawn((NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(85.), Val::Percent(100.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },))
        .id();
    let main_panel = commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    overflow: Overflow::Hidden,
                    ..default()
                },
                ..default()
            },
            MainPanel,
        ))
        .id();
    let bottom_panel = commands
        .spawn((
            NodeBundle {
                background_color: Color::rgba(0.29, 0.0, 0.51, 0.5).into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Percent(0.),
                        right: Val::Percent(0.),
                        bottom: Val::Percent(0.),
                        top: Val::Percent(96.),
                    },
                    size: Size::new(Val::Percent(100.), Val::Percent(4.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    overflow: Overflow::Hidden,
                    ..default()
                },
                ..default()
            },
            BottomPanel,
        ))
        .id();
    let add_tab = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgba(0.8, 0.8, 0.8, 0.5).into(),
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
            AddTab,
        ))
        .with_children(|builder| {
            builder.spawn(add_rectangle_txt(font.clone(), "New Tab".to_string()));
        })
        .id();
    let tab1 = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgba(0.8, 0.8, 0.8, 0.5).into(),
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
            SelectedTab { id: tab_id },
        ))
        .with_children(|builder| {
            builder.spawn(add_rectangle_txt(
                font.clone(),
                state.tabs.last().unwrap().name.clone(),
            ));
        })
        .id();
    commands.entity(bottom_panel).add_child(add_tab);
    commands.entity(bottom_panel).add_child(tab1);

    commands.entity(right_panel).add_child(main_panel);
    commands.entity(right_panel).add_child(bottom_panel);

    let left_panel_controls = commands
        .spawn((
            NodeBundle {
                style: Style {
                    padding: UiRect {
                        left: Val::Px(10.),
                        right: Val::Px(10.),
                        top: Val::Px(10.),
                        bottom: Val::Px(10.),
                    },
                    size: Size::new(Val::Percent(100.), Val::Percent(30.)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            LeftPanelControls,
        ))
        .id();
    let left_panel_explorer = commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(70.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            LeftPanelExplorer,
        ))
        .id();

    commands.entity(left_panel).add_child(left_panel_controls);
    commands.entity(left_panel).add_child(left_panel_explorer);

    let creation = add_two_buttons(
        &mut commands,
        font.clone(),
        "New Rec".to_string(),
        "Delete".to_string(),
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::Add,
        },
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::Del,
        },
    );
    let z_index = add_two_buttons(
        &mut commands,
        font,
        "Front".to_string(),
        "Back".to_string(),
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::Front,
        },
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::Back,
        },
    );
    // let tagging = add_two_buttons(
    //     &mut commands,
    //     font,
    //     "TAG".to_string(),
    //     "UNTAG".to_string(),
    //     ButtonAction {
    //         button_type: ui_helpers::ButtonTypes::Tag,
    //     },
    //     ButtonAction {
    //         button_type: ui_helpers::ButtonTypes::Untag,
    //     },
    // );

    let color_picker = commands
        .spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(80.), Val::Percent(12.)),
                margin: UiRect {
                    left: Val::Px(5.),
                    right: Val::Px(5.),
                    top: Val::Px(5.),
                    bottom: Val::Px(5.),
                },
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },))
        .id();
    let color1 = add_color(
        &mut commands,
        Color::rgb(251. / 255., 232. / 255., 166. / 255.),
    );
    let color2 = add_color(&mut commands, Color::WHITE);
    let color3 = add_color(&mut commands, Color::RED);
    let color4 = add_color(&mut commands, Color::GREEN);
    let color5 = add_color(&mut commands, Color::GRAY);

    commands.entity(color_picker).add_child(color1);
    commands.entity(color_picker).add_child(color2);
    commands.entity(color_picker).add_child(color3);
    commands.entity(color_picker).add_child(color4);
    commands.entity(color_picker).add_child(color5);

    let arrow_modes = commands
        .spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(100.), Val::Percent(12.)),
                margin: UiRect {
                    left: Val::Px(5.),
                    right: Val::Px(5.),
                    top: Val::Px(5.),
                    bottom: Val::Px(5.),
                },
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },))
        .id();
    let arrow1 = add_arrow(
        &mut commands,
        &asset_server,
        ArrowMode {
            arrow_type: ArrowType::Line,
        },
    );
    let arrow2 = add_arrow(
        &mut commands,
        &asset_server,
        ArrowMode {
            arrow_type: ArrowType::Arrow,
        },
    );
    let arrow3 = add_arrow(
        &mut commands,
        &asset_server,
        ArrowMode {
            arrow_type: ArrowType::DoubleArrow,
        },
    );
    // let arrow4 = add_arrow(
    //     &mut commands,
    //     &asset_server,
    //     ArrowMode {
    //         arrow_type: ArrowType::ParallelLine,
    //     },
    // );
    // let arrow5 = add_arrow(
    //     &mut commands,
    //     &asset_server,
    //     ArrowMode {
    //         arrow_type: ArrowType::ParallelArrow,
    //     },
    // );
    // let arrow6 = add_arrow(
    //     &mut commands,
    //     &asset_server,
    //     ArrowMode {
    //         arrow_type: ArrowType::ParallelDoubleArrow,
    //     },
    // );

    commands.entity(arrow_modes).add_child(arrow1);
    commands.entity(arrow_modes).add_child(arrow2);
    commands.entity(arrow_modes).add_child(arrow3);
    // commands.entity(arrow_modes).add_child(arrow4);
    // commands.entity(arrow_modes).add_child(arrow5);
    // commands.entity(arrow_modes).add_child(arrow6);

    let text_modes = commands
        .spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(100.), Val::Percent(12.)),
                margin: UiRect {
                    left: Val::Px(5.),
                    right: Val::Px(5.),
                    top: Val::Px(5.),
                    bottom: Val::Px(5.),
                },
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },))
        .id();
    let text_pos1 = add_text_pos(
        &mut commands,
        &asset_server,
        TextPodMode {
            text_pos: TextPos::Center,
        },
    );
    let text_pos2 = add_text_pos(
        &mut commands,
        &asset_server,
        TextPodMode {
            text_pos: TextPos::BottomRight,
        },
    );
    let text_pos3 = add_text_pos(
        &mut commands,
        &asset_server,
        TextPodMode {
            text_pos: TextPos::BottomLeft,
        },
    );
    let text_pos4 = add_text_pos(
        &mut commands,
        &asset_server,
        TextPodMode {
            text_pos: TextPos::TopLeft,
        },
    );
    let text_pos5 = add_text_pos(
        &mut commands,
        &asset_server,
        TextPodMode {
            text_pos: TextPos::TopRight,
        },
    );
    commands.entity(text_modes).add_child(text_pos1);
    commands.entity(text_modes).add_child(text_pos2);
    commands.entity(text_modes).add_child(text_pos3);
    commands.entity(text_modes).add_child(text_pos4);
    commands.entity(text_modes).add_child(text_pos5);

    commands.entity(left_panel_controls).add_child(creation);
    commands.entity(left_panel_controls).add_child(z_index);
    // commands.entity(left_panel_controls).add_child(tagging);
    commands.entity(left_panel_controls).add_child(color_picker);
    commands.entity(left_panel_controls).add_child(arrow_modes);
    commands.entity(left_panel_controls).add_child(text_modes);

    commands.entity(main_bottom).add_child(left_panel);
    commands.entity(main_bottom).add_child(right_panel);
    commands.entity(root_ui).add_child(menu);
    commands.entity(root_ui).add_child(main_bottom);

    state.main_panel = Some(main_panel);
}

fn add_color(commands: &mut Commands, color: Color) -> Entity {
    commands
        .spawn((
            ButtonBundle {
                background_color: color.into(),
                style: Style {
                    size: Size::new(Val::Percent(15.), Val::Percent(100.)),
                    align_items: AlignItems::Center,
                    margin: UiRect {
                        left: Val::Px(5.),
                        right: Val::Px(5.),
                        top: Val::Px(5.),
                        bottom: Val::Px(5.),
                    },
                    border: UiRect::all(Val::Px(1.)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            BorderColor(Color::BLACK),
            ChangeColor { color },
        ))
        .id()
}

fn add_arrow(
    commands: &mut Commands,
    arrow_server: &Res<AssetServer>,
    arrow_mode: ArrowMode,
) -> Entity {
    let image = match arrow_mode.arrow_type {
        ArrowType::Line => arrow_server.load("line.png"),
        ArrowType::Arrow => arrow_server.load("arrow.png"),
        ArrowType::DoubleArrow => arrow_server.load("double-arrow.png"),
        ArrowType::ParallelLine => arrow_server.load("parallel-line.png"),
        ArrowType::ParallelArrow => arrow_server.load("parallel-arrow.png"),
        ArrowType::ParallelDoubleArrow => arrow_server.load("parallel-double-arrow.png"),
    };
    commands
        .spawn((
            ButtonBundle {
                background_color: Color::Rgba {
                    red: 1.,
                    green: 1.,
                    blue: 1.,
                    alpha: 0.5,
                }
                .into(),
                image: image.into(),
                style: Style {
                    size: Size::new(Val::Percent(12.), Val::Percent(100.)),
                    align_items: AlignItems::Center,
                    margin: UiRect {
                        left: Val::Px(5.),
                        right: Val::Px(5.),
                        top: Val::Px(5.),
                        bottom: Val::Px(5.),
                    },
                    border: UiRect::all(Val::Px(1.)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            BorderColor(Color::BLACK),
            arrow_mode,
        ))
        .id()
}

fn add_text_pos(
    commands: &mut Commands,
    arrow_server: &Res<AssetServer>,
    text_pos_mode: TextPodMode,
) -> Entity {
    let image = match text_pos_mode.text_pos {
        crate::TextPos::Center => arrow_server.load("text-center.png"),
        crate::TextPos::BottomRight => arrow_server.load("text-right-bottom.png"),
        crate::TextPos::BottomLeft => arrow_server.load("text-left-bottom.png"),
        crate::TextPos::TopRight => arrow_server.load("text-right-top.png"),
        crate::TextPos::TopLeft => arrow_server.load("text-left-top.png"),
    };
    commands
        .spawn((
            ButtonBundle {
                background_color: Color::Rgba {
                    red: 1.,
                    green: 1.,
                    blue: 1.,
                    alpha: 0.5,
                }
                .into(),
                image: image.into(),
                style: Style {
                    size: Size::new(Val::Percent(12.), Val::Percent(100.)),
                    align_items: AlignItems::Center,
                    margin: UiRect {
                        left: Val::Px(5.),
                        right: Val::Px(5.),
                        top: Val::Px(5.),
                        bottom: Val::Px(5.),
                    },
                    border: UiRect::all(Val::Px(1.)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            BorderColor(Color::BLACK),
            text_pos_mode,
        ))
        .id()
}

fn add_two_buttons(
    commands: &mut Commands,
    font: Handle<Font>,
    label_do: String,
    label_undo: String,
    component_do: impl Component,
    component_undo: impl Component,
) -> Entity {
    let node = commands
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(85.), Val::Percent(12.)),
                margin: UiRect {
                    left: Val::Px(5.),
                    right: Val::Px(5.),
                    top: Val::Px(5.),
                    bottom: Val::Px(5.),
                },
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .id();
    let do_button = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                style: Style {
                    size: Size::new(Val::Percent(40.), Val::Percent(100.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            component_do,
        ))
        .with_children(|builder| {
            builder.spawn(add_rectangle_txt(font.clone(), label_do));
        })
        .id();

    let undo_button = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                style: Style {
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(40.), Val::Percent(100.)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            component_undo,
        ))
        .with_children(|builder| {
            builder.spawn(add_rectangle_txt(font.clone(), label_undo));
        })
        .id();
    commands.entity(node).add_child(do_button);
    commands.entity(node).add_child(undo_button);
    node
}
