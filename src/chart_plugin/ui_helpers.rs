use bevy::prelude::*;

pub fn get_marker_style(position: UiRect) -> Style {
    Style {
        position_type: PositionType::Absolute,
        position,
        size: Size::new(Val::Px(5.), Val::Px(5.)),
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        ..default()
    }
}