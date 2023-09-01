#![allow(clippy::duplicate_mod)]
use bevy::prelude::*;

use bevy::window::PrimaryWindow;
use bevy_cosmic_edit::{create_cosmic_font_system, CosmicFont, CosmicFontConfig};

use bevy_pkv::PkvStore;

use super::ui_helpers::{
    self, AddTab, BottomPanel, ButtonAction, ChangeTheme, DrawPencil, LeftPanel, LeftPanelControls,
    LeftPanelExplorer, MainPanel, Menu, NewDoc, ParticlesEffect, Root, SaveDoc, TextPosMode,
    TwoPointsDraw,
};
use super::{CommChannels, ExportToFile, ImportFromFile, ImportFromUrl, ShareDoc};
use crate::canvas::arrow::components::{ArrowMode, ArrowType};
use crate::resources::{AppState, FontSystemState};
use crate::themes::Theme;
use crate::utils::get_theme_key;
use crate::TextPos;

#[path = "../../../macros.rs"]
#[macro_use]
mod macros;

#[path = "add_arrow.rs"]
mod add_arrow;
use add_arrow::*;

#[path = "add_color.rs"]
mod add_color;
use add_color::*;

#[path = "add_front_back.rs"]
mod add_front_back;
use add_front_back::*;

#[path = "add_text_pos.rs"]
mod add_text_pos;
use add_text_pos::*;

#[path = "node_manipulation.rs"]
mod node_manipulation;
use node_manipulation::*;

#[path = "add_menu_button.rs"]
mod add_menu_button;
use add_menu_button::*;

#[path = "add_list.rs"]
mod add_list;
use add_list::*;

#[path = "add_effect.rs"]
mod add_effect;
use add_effect::*;

#[path = "add_pencil.rs"]
mod add_pencil;
use add_pencil::*;

#[path = "add_two_points_draw.rs"]
mod add_two_points_draw;
use add_two_points_draw::*;

#[path = "add_text.rs"]
mod add_text;
use add_text::*;

#[path = "add_search_box.rs"]
mod add_search_box;
use add_search_box::*;

#[path = "add_visibility.rs"]
mod add_visibility;
use add_visibility::*;

// Think about splitting this function to wasm and native
pub fn init_layout(
    mut commands: Commands,
    mut app_state: ResMut<AppState>,
    asset_server: Res<AssetServer>,
    mut pkv: ResMut<PkvStore>,
    mut cosmic_fonts: ResMut<Assets<CosmicFont>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut fonts: ResMut<Assets<Font>>,
    theme: Res<Theme>,
) {
    // font setup
    let font_bytes_regular = include_bytes!("../../../../assets/fonts/VictorMono-Regular.ttf");
    let font_bytes_bold = include_bytes!("../../../../assets/fonts/VictorMono-Bold.ttf");
    let font_bytes_italic = include_bytes!("../../../../assets/fonts/VictorMono-Italic.ttf");
    let font_bytes_bold_italic =
        include_bytes!("../../../../assets/fonts/VictorMono-BoldItalic.ttf");
    let font_bytes_medium = include_bytes!("../../../../assets/fonts/VictorMono-Medium.ttf");
    let font_bytes_semibold = include_bytes!("../../../../assets/fonts/VictorMono-SemiBold.ttf");
    let font = Font::try_from_bytes(font_bytes_regular.to_vec()).unwrap();
    let text_style = TextStyle {
        font: TextStyle::default().font,
        font_size: 14.0,
        color: theme.font,
    };
    fonts.set_untracked(text_style.font, font);
    let cosmic_font_config = CosmicFontConfig {
        fonts_dir_path: None,
        load_system_fonts: true,
        font_bytes: Some(vec![
            font_bytes_regular,
            font_bytes_italic,
            font_bytes_bold,
            font_bytes_bold_italic,
            font_bytes_medium,
            font_bytes_semibold,
        ]),
    };
    let font_system = create_cosmic_font_system(cosmic_font_config);
    let cosmic_font_handle = cosmic_fonts.add(CosmicFont(font_system));
    commands.insert_resource(FontSystemState(Some(cosmic_font_handle.clone())));

    let primary_window: &Window = windows.single();
    #[cfg(not(target_arch = "wasm32"))]
    {
        let (tx, rx) = async_channel::bounded(1);
        commands.insert_resource(CommChannels { tx, rx });
    }
    let icon_font = asset_server.load("fonts/MaterialIcons-Regular.ttf");
    let bottom_panel = commands
        .spawn((
            NodeBundle {
                border_color: theme.btn_border.into(),
                background_color: theme.bottom_panel_bg.into(),
                style: Style {
                    border: UiRect::all(Val::Px(1.0)),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(0.),
                    right: Val::Percent(0.),
                    bottom: Val::Percent(0.),
                    top: Val::Percent(96.),
                    width: Val::Percent(100.),
                    height: Val::Percent(4.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    overflow: Overflow::clip(),
                    ..default()
                },
                ..default()
            },
            BottomPanel,
        ))
        .id();
    let add_tab = add_menu_button(
        &mut commands,
        &theme,
        "New Tab".to_string(),
        &icon_font,
        AddTab,
    );
    commands.entity(bottom_panel).add_child(add_tab);

    let docs = add_list(&mut commands, &theme, &mut app_state, &mut pkv);

    let root_ui = commands
        .spawn((
            NodeBundle {
                style: Style {
                    left: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
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
                background_color: theme.menu_bg.into(),
                border_color: theme.btn_border.into(),
                style: Style {
                    border: UiRect::all(Val::Px(1.0)),
                    width: Val::Percent(100.),
                    height: Val::Percent(5.),
                    padding: UiRect {
                        left: Val::Px(10.),
                        ..default()
                    },
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .id();
    let new_doc = add_menu_button(
        &mut commands,
        &theme,
        "New Document".to_string(),
        &icon_font,
        NewDoc,
    );
    let save_doc = add_menu_button(
        &mut commands,
        &theme,
        "Save Document".to_string(),
        &icon_font,
        SaveDoc,
    );
    #[cfg(not(target_arch = "wasm32"))]
    let export_file = add_menu_button(
        &mut commands,
        &theme,
        "Export To File".to_string(),
        &icon_font,
        ExportToFile,
    );
    #[cfg(not(target_arch = "wasm32"))]
    let import_file = add_menu_button(
        &mut commands,
        &theme,
        "Import From File".to_string(),
        &icon_font,
        ImportFromFile,
    );
    #[cfg(not(target_arch = "wasm32"))]
    let import_url = add_menu_button(
        &mut commands,
        &theme,
        "Import From URL".to_string(),
        &icon_font,
        ImportFromUrl,
    );
    #[cfg(target_arch = "wasm32")]
    let set_window_prop = add_menu_button(
        &mut commands,
        &theme,
        "Save Document to window.velo object".to_string(),
        &icon_font,
        super::SetWindowProperty,
    );
    commands.entity(menu).add_child(new_doc);
    commands.entity(menu).add_child(save_doc);
    #[cfg(not(target_arch = "wasm32"))]
    commands.entity(menu).add_child(export_file);
    #[cfg(not(target_arch = "wasm32"))]
    commands.entity(menu).add_child(import_file);
    #[cfg(not(target_arch = "wasm32"))]
    commands.entity(menu).add_child(import_url);
    if app_state.github_token.is_some() {
        let share_doc = add_menu_button(
            &mut commands,
            &theme,
            "Share Document (copy URL to clipboard)".to_string(),
            &icon_font,
            ShareDoc,
        );
        commands.entity(menu).add_child(share_doc);
    }
    #[cfg(target_arch = "wasm32")]
    commands.entity(menu).add_child(set_window_prop);
    let theme_key = get_theme_key(&pkv);
    let theme_msg = if theme_key == "light" {
        "Enable dark theme (restart is required for now)".to_string()
    } else {
        "Enable light theme (restart is required for now)".to_string()
    };

    let change_theme = add_menu_button(&mut commands, &theme, theme_msg, &icon_font, ChangeTheme);
    commands.entity(menu).add_child(change_theme);

    let main_bottom = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(95.),
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
                background_color: theme.left_panel_bg.into(),
                border_color: theme.btn_border.into(),
                style: Style {
                    border: UiRect::all(Val::Px(1.0)),
                    width: Val::Percent(15.),
                    height: Val::Percent(100.),
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
                width: Val::Percent(85.),
                height: Val::Percent(100.),
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
            ButtonBundle {
                background_color: Color::NONE.into(),
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    overflow: Overflow::clip(),
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
                    padding: UiRect::all(Val::Px(10.)),
                    width: Val::Percent(100.),
                    height: Val::Percent(40.),
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
    #[cfg(not(target_arch = "wasm32"))]
    let search_box = add_search_box(
        &mut commands,
        &theme,
        &mut cosmic_fonts,
        cosmic_font_handle,
        primary_window.scale_factor() as f32,
    );
    let left_panel_explorer = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(60.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            LeftPanelExplorer,
        ))
        .id();
    #[cfg(not(target_arch = "wasm32"))]
    commands.entity(left_panel_explorer).add_child(search_box);
    commands.entity(left_panel_explorer).add_child(docs);

    commands.entity(left_panel).add_child(left_panel_controls);
    commands.entity(left_panel).add_child(left_panel_explorer);

    let rectangle_creation = node_manipulation(
        &mut commands,
        &theme,
        &icon_font,
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::AddRec,
        },
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::AddCircle,
        },
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::AddPaper,
        },
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::Del,
        },
    );
    let fron_back = commands
        .spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                width: Val::Percent(90.),
                height: Val::Percent(9.),
                margin: UiRect::all(Val::Px(5.)),
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        },))
        .id();
    let front = add_front_back(
        &mut commands,
        &theme,
        &asset_server,
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::Front,
        },
    );
    let back = add_front_back(
        &mut commands,
        &theme,
        &asset_server,
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::Back,
        },
    );
    commands.entity(fron_back).add_child(front);
    commands.entity(fron_back).add_child(back);

    let color_picker = commands
        .spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                width: Val::Percent(90.),
                height: Val::Percent(9.),
                margin: UiRect::all(Val::Px(5.)),
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        },))
        .id();
    let color1 = add_color(&mut commands, &theme, pair_struct!(theme.color_change_1));
    let color2 = add_color(&mut commands, &theme, pair_struct!(theme.color_change_2));
    let color3 = add_color(&mut commands, &theme, pair_struct!(theme.color_change_3));
    let color4 = add_color(&mut commands, &theme, pair_struct!(theme.color_change_4));
    let color5 = add_color(&mut commands, &theme, pair_struct!(theme.color_change_5));

    commands.entity(color_picker).add_child(color1);
    commands.entity(color_picker).add_child(color2);
    commands.entity(color_picker).add_child(color3);
    commands.entity(color_picker).add_child(color4);
    commands.entity(color_picker).add_child(color5);

    let arrow_modes = commands
        .spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                width: Val::Percent(90.),
                height: Val::Percent(9.),
                margin: UiRect::all(Val::Px(5.)),
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        },))
        .id();
    let arrow1 = add_arrow(
        &mut commands,
        &theme,
        &asset_server,
        ArrowMode {
            arrow_type: ArrowType::Line,
        },
    );
    let arrow2 = add_arrow(
        &mut commands,
        &theme,
        &asset_server,
        ArrowMode {
            arrow_type: ArrowType::Arrow,
        },
    );
    let arrow3 = add_arrow(
        &mut commands,
        &theme,
        &asset_server,
        ArrowMode {
            arrow_type: ArrowType::DoubleArrow,
        },
    );
    let arrow4 = add_arrow(
        &mut commands,
        &theme,
        &asset_server,
        ArrowMode {
            arrow_type: ArrowType::ParallelLine,
        },
    );
    let arrow5 = add_arrow(
        &mut commands,
        &theme,
        &asset_server,
        ArrowMode {
            arrow_type: ArrowType::ParallelArrow,
        },
    );
    let arrow6 = add_arrow(
        &mut commands,
        &theme,
        &asset_server,
        ArrowMode {
            arrow_type: ArrowType::ParallelDoubleArrow,
        },
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
                width: Val::Percent(90.),
                height: Val::Percent(9.),
                margin: UiRect::all(Val::Px(5.)),
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        },))
        .id();
    let text_pos1 = add_text_pos(
        &mut commands,
        &theme,
        TextPosMode {
            text_pos: TextPos::Center,
        },
        "Center Text".to_string(),
        &icon_font,
    );
    let text_pos2 = add_text_pos(
        &mut commands,
        &theme,
        TextPosMode {
            text_pos: TextPos::TopLeft,
        },
        "Top Left Text".to_string(),
        &icon_font,
    );
    commands.entity(text_modes).add_child(text_pos1);
    commands.entity(text_modes).add_child(text_pos2);

    let visibility = commands
        .spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                width: Val::Percent(90.),
                height: Val::Percent(9.),
                margin: UiRect::all(Val::Px(5.)),
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        },))
        .id();
    let show_children = add_visibility(
        &mut commands,
        &theme,
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::ShowChildren,
        },
        "Show children notes".to_string(),
        &icon_font,
    );
    let hide_notes = add_visibility(
        &mut commands,
        &theme,
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::HideChildren,
        },
        "Hide children notes".to_string(),
        &icon_font,
    );
    commands.entity(visibility).add_child(show_children);
    commands.entity(visibility).add_child(hide_notes);

    let left_panel_bottom = commands
        .spawn((NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(90.),
                height: Val::Percent(10.),
                margin: UiRect {
                    left: Val::Px(5.),
                    right: Val::Px(5.),
                    top: Val::Px(5.),
                    bottom: Val::Px(20.),
                },
                ..default()
            },
            ..default()
        },))
        .id();
    let pencil_panel = commands.spawn(NodeBundle::default()).id();
    #[cfg(not(target_arch = "wasm32"))]
    {
        let effect = add_effect(&mut commands, &theme, &icon_font, ParticlesEffect);
        commands.entity(pencil_panel).add_child(effect);
    }
    let pencil = add_pencil(&mut commands, &theme, &icon_font, DrawPencil);
    commands.entity(pencil_panel).add_child(pencil);
    commands.entity(left_panel_bottom).add_child(pencil_panel);

    let two_points_draw = commands.spawn(NodeBundle::default()).id();

    let draw_line = add_two_points_draw(
        &mut commands,
        &theme,
        &icon_font,
        TwoPointsDraw {
            drawing_type: ui_helpers::TwoPointsDrawType::Line,
        },
    );
    commands.entity(two_points_draw).add_child(draw_line);
    let draw_circle = add_two_points_draw(
        &mut commands,
        &theme,
        &icon_font,
        TwoPointsDraw {
            drawing_type: ui_helpers::TwoPointsDrawType::Rhombus,
        },
    );
    commands.entity(two_points_draw).add_child(draw_circle);
    let draw_rect = add_two_points_draw(
        &mut commands,
        &theme,
        &icon_font,
        TwoPointsDraw {
            drawing_type: ui_helpers::TwoPointsDrawType::Rectangle,
        },
    );
    commands.entity(two_points_draw).add_child(draw_rect);
    let draw_arrow = add_two_points_draw(
        &mut commands,
        &theme,
        &icon_font,
        TwoPointsDraw {
            drawing_type: ui_helpers::TwoPointsDrawType::Arrow,
        },
    );
    commands.entity(two_points_draw).add_child(draw_arrow);
    let add_text = add_text(
        &mut commands,
        &theme,
        &icon_font,
        ButtonAction {
            button_type: ui_helpers::ButtonTypes::AddText,
        },
    );
    commands.entity(two_points_draw).add_child(add_text);
    commands
        .entity(left_panel_bottom)
        .add_child(two_points_draw);

    commands
        .entity(left_panel_controls)
        .add_child(rectangle_creation);
    commands.entity(left_panel_controls).add_child(color_picker);
    commands.entity(left_panel_controls).add_child(arrow_modes);
    commands.entity(left_panel_controls).add_child(text_modes);
    commands.entity(left_panel_controls).add_child(fron_back);
    commands.entity(left_panel_controls).add_child(visibility);
    commands
        .entity(left_panel_controls)
        .add_child(left_panel_bottom);

    commands.entity(main_bottom).add_child(left_panel);
    commands.entity(main_bottom).add_child(right_panel);
    commands.entity(root_ui).add_child(menu);
    commands.entity(root_ui).add_child(main_bottom);
}
