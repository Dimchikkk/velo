use bevy::prelude::*;

use crate::{AppState, MainCamera, SaveRequest};

use super::ui_helpers::{
    add_rectangle_txt, create_rectangle_txt, CreateRectButton, LeftPanel, LeftPanelControls,
    LeftPanelExplorer, LoadState, MainPanel, Menu, Root, SaveState, DelRectButton,
};

pub fn init_layout(
    mut commands: Commands,
    mut state: ResMut<AppState>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/iosevka-regular.ttf");
    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands.insert_resource(SaveRequest { path: None });

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
                    red: 192.,
                    green: 192.,
                    blue: 192.,
                    alpha: 1.,
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
    let main_panel = commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(85.), Val::Percent(100.)),
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

    let left_panel_controls = commands
        .spawn((
            NodeBundle {
                style: Style {
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

    let add_rect = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                style: Style {
                    size: Size::new(Val::Percent(50.), Val::Percent(30.)),
                    align_items: AlignItems::Center,
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
            },
            CreateRectButton,
        ))
        .with_children(|builder| {
            builder.spawn(add_rectangle_txt(font.clone(), "NEW".to_string()));
        })
        .id();
    
    let del_rect = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                style: Style {
                    size: Size::new(Val::Percent(50.), Val::Percent(30.)),
                    align_items: AlignItems::Center,
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
            },
            DelRectButton,
        ))
        .with_children(|builder| {
            builder.spawn(add_rectangle_txt(font.clone(), "DEL".to_string()));
        })
        .id();
    commands.entity(left_panel_controls).add_child(add_rect);
    commands.entity(left_panel_controls).add_child(del_rect);

    commands.entity(main_bottom).add_child(left_panel);
    commands.entity(main_bottom).add_child(main_panel);
    commands.entity(root_ui).add_child(menu);
    commands.entity(root_ui).add_child(main_bottom);

    state.main_panel = Some(main_panel);
}
