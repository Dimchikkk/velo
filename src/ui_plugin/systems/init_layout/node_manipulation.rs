use bevy::{prelude::*, text::BreakLineOn};

use crate::{
    themes::Theme,
    ui_plugin::ui_helpers::{get_tooltip, ButtonAction, GenericButton, Tooltip, TooltipPosition},
};

pub fn node_manipulation(
    commands: &mut Commands,
    theme: &Res<Theme>,
    icon_font: &Handle<Font>,
    create_rec_component: ButtonAction,
    create_circle_component: ButtonAction,
    papernote_component: ButtonAction,
    delete_component: ButtonAction,
) -> Entity {
    let node = commands
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                width: Val::Percent(90.),
                height: Val::Percent(12.),
                margin: UiRect::all(Val::Px(5.)),
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        })
        .id();

    let top_new_paper = add_button_action(
        commands,
        theme,
        "New Papernote".to_string(),
        icon_font,
        "\u{eb54}".to_string(),
        theme.paper_node_bg,
        papernote_component,
    );

    let top_new_rec = add_button_action(
        commands,
        theme,
        "New Rectangle".to_string(),
        icon_font,
        "\u{eb54}".to_string(),
        theme.node_manipulation,
        create_rec_component,
    );

    let top_new_circle = add_button_action(
        commands,
        theme,
        "New Circle".to_string(),
        icon_font,
        "\u{ef4a}".to_string(),
        theme.node_manipulation,
        create_circle_component,
    );

    let top_del = add_button_action(
        commands,
        theme,
        "Delete element".to_string(),
        icon_font,
        "\u{e872}".to_string(),
        theme.node_manipulation,
        delete_component,
    );

    commands.entity(node).add_child(top_del);
    commands.entity(node).add_child(top_new_circle);
    commands.entity(node).add_child(top_new_rec);
    commands.entity(node).add_child(top_new_paper);
    node
}

fn add_button_action(
    commands: &mut Commands,
    theme: &Res<Theme>,
    label: String,
    icon_font: &Handle<Font>,
    icon_code: String,
    icon_color: Color,
    button_action: ButtonAction,
) -> Entity {
    let top = commands
        .spawn(NodeBundle {
            background_color: theme.shadow.into(),
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                margin: UiRect::all(Val::Px(5.)),
                width: Val::Percent(23.),
                height: Val::Percent(100.),
                ..default()
            },
            ..default()
        })
        .id();
    let new_button_action = commands
        .spawn((
            ButtonBundle {
                background_color: theme.node_manipulation_bg.into(),
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    left: Val::Px(1.),
                    right: Val::Auto,
                    top: Val::Px(-1.),
                    bottom: Val::Auto,
                    ..default()
                },
                ..default()
            },
            button_action,
            GenericButton,
        ))
        .with_children(|builder| {
            builder.spawn((get_tooltip(theme, label, TooltipPosition::Bottom), Tooltip));

            let text_style = TextStyle {
                font_size: 25.0,
                color: icon_color,
                font: icon_font.clone(),
            };
            let text = Text {
                sections: vec![TextSection {
                    value: icon_code,
                    style: text_style,
                }],
                alignment: TextAlignment::Left,
                linebreak_behavior: BreakLineOn::WordBoundary,
            };
            let text_bundle_style = Style {
                position_type: PositionType::Absolute,
                padding: UiRect::all(Val::Px(5.)),
                margin: UiRect::all(Val::Px(3.)),
                ..default()
            };

            builder.spawn(TextBundle {
                text,
                style: text_bundle_style,
                ..default()
            });
        })
        .id();
    commands.entity(top).add_child(new_button_action);
    top
}
