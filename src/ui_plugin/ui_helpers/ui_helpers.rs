use bevy::{prelude::*, text::BreakLineOn};

use crate::themes::Theme;
#[path = "components.rs"]
mod components;
pub use components::*;
#[path = "spawn_node.rs"]
mod spawn_node;
pub use spawn_node::*;
#[path = "spawn_modal.rs"]
mod spawn_modal;
pub use spawn_modal::*;
#[path = "add_tab.rs"]
mod add_tab;
pub use add_tab::*;
#[path = "add_list_item.rs"]
mod add_list_item;
pub use add_list_item::*;

#[path = "spawn_shadow.rs"]
mod spawn_shadow;
pub use spawn_shadow::*;

pub fn add_rectangle_txt(theme: &Res<Theme>, text: String) -> TextBundle {
    let text_style = TextStyle {
        font_size: 18.0,
        color: theme.font,
        ..default()
    };
    TextBundle::from_section(text, text_style).with_style(Style {
        position_type: PositionType::Relative,
        ..default()
    })
}
pub enum TooltipPosition {
    Top,
    Bottom,
}

pub fn get_tooltip(
    theme: &Res<Theme>,
    text: String,
    tooltip_position: TooltipPosition,
) -> TextBundle {
    let text = Text {
        sections: vec![TextSection {
            value: text,
            style: TextStyle {
                font_size: theme.font_size,
                color: theme.font,
                ..default()
            },
        }],
        alignment: TextAlignment::Center,
        linebreak_behavior: BreakLineOn::WordBoundary,
    };
    let position = match tooltip_position {
        TooltipPosition::Bottom => UiRect {
            left: Val::Px(30.),
            right: Val::Auto,
            top: Val::Px(40.),
            bottom: Val::Auto,
        },
        TooltipPosition::Top => UiRect {
            left: Val::Px(30.),
            right: Val::Auto,
            top: Val::Px(-40.),
            bottom: Val::Auto,
        },
    };
    let text_bundle_style = Style {
        position_type: PositionType::Absolute,
        left: position.left,
        right: position.right,
        top: position.top,
        bottom: position.bottom,
        display: Display::None,
        width: Val::Auto,
        height: Val::Auto,
        ..default()
    };
    TextBundle {
        z_index: ZIndex::Global(1),
        background_color: theme.tooltip_bg.into(),
        text,
        style: text_bundle_style,
        ..default()
    }
}
