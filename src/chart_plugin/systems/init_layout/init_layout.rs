use bevy::prelude::*;

use bevy_pkv::PkvStore;

use crate::{AppState, MainCamera, TextPos};

use super::ui_helpers::{
    self, add_rectangle_txt, AddTab, ArrowMode, ArrowType, BottomPanel, ButtonAction, DeleteDoc,
    DeleteTab, LeftPanel, LeftPanelControls, LeftPanelExplorer, MainPanel, Menu, NewDoc, RenameDoc,
    RenameTab, Root, SaveDoc, TextManipulation, TextManipulationAction, TextPosMode,
};

#[path = "add_arrow.rs"]
mod add_arrow;
use add_arrow::*;

#[path = "add_color.rs"]
mod add_color;
use add_color::*;

#[path = "add_front_back.rs"]
mod add_front_back;
use add_front_back::*;

#[path = "list_systems.rs"]
mod list_systems;
pub use list_systems::*;

#[path = "add_text_manipulation.rs"]
mod add_text_manipulation;
use add_text_manipulation::*;

#[path = "add_text_pos.rs"]
mod add_text_pos;
use add_text_pos::*;

#[path = "add_new_or_delete_rec.rs"]
mod add_new_or_delete_rec;
use add_new_or_delete_rec::*;

#[path = "add_menu_button.rs"]
mod add_menu_button;
use add_menu_button::*;

pub fn init_layout(
    mut commands: Commands,
    mut state: ResMut<AppState>,
    asset_server: Res<AssetServer>,
    mut pkv: ResMut<PkvStore>,
) {
    let font = asset_server.load("fonts/iosevka-regular.ttf");

    state.font = Some(font.clone());
    commands.spawn((Camera2dBundle::default(), MainCamera));
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
    let rename_tab = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgba(0.8, 0.8, 0.8, 0.5).into(),
                style: Style {
                    size: Size::new(Val::Px(60.), Val::Px(30.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    margin: UiRect {
                        left: Val::Px(0.),
                        right: Val::Px(10.),
                        top: Val::Px(0.),
                        bottom: Val::Px(0.),
                    },
                    ..default()
                },

                ..default()
            },
            RenameTab,
        ))
        .with_children(|builder| {
            builder.spawn(add_rectangle_txt(font.clone(), "Rename".to_string()));
        })
        .id();
    let del_tab = commands
        .spawn((
            ButtonBundle {
                background_color: Color::rgba(0.8, 0.8, 0.8, 0.5).into(),
                style: Style {
                    size: Size::new(Val::Px(60.), Val::Px(30.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    margin: UiRect {
                        left: Val::Px(0.),
                        right: Val::Px(20.),
                        top: Val::Px(0.),
                        bottom: Val::Px(0.),
                    },
                    ..default()
                },

                ..default()
            },
            DeleteTab,
        ))
        .with_children(|builder| {
            builder.spawn(add_rectangle_txt(font.clone(), "Delete".to_string()));
        })
        .id();
    commands.entity(bottom_panel).add_child(add_tab);
    commands.entity(bottom_panel).add_child(rename_tab);
    commands.entity(bottom_panel).add_child(del_tab);

    let docs = add_list(
        bottom_panel,
        &mut commands,
        &mut state,
        &mut pkv,
        font.clone(),
    );

    let root_ui = commands
        .spawn((
            NodeBundle {
                style: Style {
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
    let new_doc = add_menu_button(&mut commands, font.clone(), "New Doc".to_string(), NewDoc);
    let rename_doc = add_menu_button(&mut commands, font.clone(), "Rename".to_string(), RenameDoc);
    let delete_doc = add_menu_button(&mut commands, font.clone(), "Delete".to_string(), DeleteDoc);
    let save_doc = add_menu_button(&mut commands, font.clone(), "Save".to_string(), SaveDoc);

    commands.entity(menu).add_child(new_doc);
    commands.entity(menu).add_child(rename_doc);
    commands.entity(menu).add_child(delete_doc);
    commands.entity(menu).add_child(save_doc);

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
                    size: Size::new(Val::Percent(100.), Val::Percent(40.)),
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
                    size: Size::new(Val::Percent(100.), Val::Percent(60.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            LeftPanelExplorer,
        ))
        .id();
    commands.entity(left_panel_explorer).add_child(docs);

    commands.entity(left_panel).add_child(left_panel_controls);
    commands.entity(left_panel).add_child(left_panel_explorer);

    let creation = add_new_delete_rec(
        &mut commands,
        font.clone(),
        "New Rec".to_string(),
        "Del Rec".to_string(),
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::Add,
        },
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::Del,
        },
    );
    let fron_back = commands
        .spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(100.), Val::Percent(10.)),
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
    let front = add_front_back(
        &mut commands,
        &asset_server,
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::Front,
        },
        font.clone(),
    );
    let back = add_front_back(
        &mut commands,
        &asset_server,
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::Back,
        },
        font.clone(),
    );
    commands.entity(fron_back).add_child(front);
    commands.entity(fron_back).add_child(back);

    let color_picker = commands
        .spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(90.), Val::Percent(10.)),
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
                size: Size::new(Val::Percent(100.), Val::Percent(10.)),
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
        font.clone(),
    );
    let arrow2 = add_arrow(
        &mut commands,
        &asset_server,
        ArrowMode {
            arrow_type: ArrowType::Arrow,
        },
        font.clone(),
    );
    let arrow3 = add_arrow(
        &mut commands,
        &asset_server,
        ArrowMode {
            arrow_type: ArrowType::DoubleArrow,
        },
        font.clone(),
    );
    let arrow4 = add_arrow(
        &mut commands,
        &asset_server,
        ArrowMode {
            arrow_type: ArrowType::ParallelLine,
        },
        font.clone(),
    );
    let arrow5 = add_arrow(
        &mut commands,
        &asset_server,
        ArrowMode {
            arrow_type: ArrowType::ParallelArrow,
        },
        font.clone(),
    );
    let arrow6 = add_arrow(
        &mut commands,
        &asset_server,
        ArrowMode {
            arrow_type: ArrowType::ParallelDoubleArrow,
        },
        font.clone(),
    );
    commands.entity(arrow_modes).add_child(arrow1);
    commands.entity(arrow_modes).add_child(arrow2);
    commands.entity(arrow_modes).add_child(arrow3);
    commands.entity(arrow_modes).add_child(arrow4);
    commands.entity(arrow_modes).add_child(arrow5);
    commands.entity(arrow_modes).add_child(arrow6);

    let text_modes = commands
        .spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(100.), Val::Percent(10.)),
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
        TextPosMode {
            text_pos: TextPos::Center,
        },
    );
    let text_pos2 = add_text_pos(
        &mut commands,
        &asset_server,
        TextPosMode {
            text_pos: TextPos::BottomRight,
        },
    );
    let text_pos3 = add_text_pos(
        &mut commands,
        &asset_server,
        TextPosMode {
            text_pos: TextPos::BottomLeft,
        },
    );
    let text_pos4 = add_text_pos(
        &mut commands,
        &asset_server,
        TextPosMode {
            text_pos: TextPos::TopLeft,
        },
    );
    let text_pos5 = add_text_pos(
        &mut commands,
        &asset_server,
        TextPosMode {
            text_pos: TextPos::TopRight,
        },
    );
    commands.entity(text_modes).add_child(text_pos1);
    commands.entity(text_modes).add_child(text_pos2);
    commands.entity(text_modes).add_child(text_pos3);
    commands.entity(text_modes).add_child(text_pos4);
    commands.entity(text_modes).add_child(text_pos5);

    let text_manipulation = commands
        .spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(100.), Val::Percent(10.)),
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
    let cut = add_text_manipulation(
        &mut commands,
        &asset_server,
        TextManipulationAction {
            action_type: TextManipulation::Cut,
        },
        font.clone(),
    );

    #[cfg(not(target_arch = "wasm32"))]
    {
        let copy = add_text_manipulation(
            &mut commands,
            &asset_server,
            TextManipulationAction {
                action_type: TextManipulation::Copy,
            },
            font.clone(),
        );
        let paste = add_text_manipulation(
            &mut commands,
            &asset_server,
            TextManipulationAction {
                action_type: TextManipulation::Paste,
            },
            font.clone(),
        );
        let open_all_links = add_text_manipulation(
            &mut commands,
            &asset_server,
            TextManipulationAction {
                action_type: TextManipulation::OpenAllLinks,
            },
            font,
        );
        commands.entity(text_manipulation).add_child(copy);
        commands.entity(text_manipulation).add_child(paste);
        commands.entity(text_manipulation).add_child(open_all_links);
    }
    commands.entity(text_manipulation).add_child(cut);

    commands.entity(left_panel_controls).add_child(creation);
    commands.entity(left_panel_controls).add_child(color_picker);
    commands.entity(left_panel_controls).add_child(arrow_modes);
    commands.entity(left_panel_controls).add_child(text_modes);
    commands
        .entity(left_panel_controls)
        .add_child(text_manipulation);
    commands.entity(left_panel_controls).add_child(fron_back);

    commands.entity(main_bottom).add_child(left_panel);
    commands.entity(main_bottom).add_child(right_panel);
    commands.entity(root_ui).add_child(menu);
    commands.entity(root_ui).add_child(main_bottom);

    state.main_panel = Some(main_panel);
}
