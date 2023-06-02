use bevy::prelude::*;
use bevy_cosmic_edit::{spawn_cosmic_edit, CosmicEditMeta, CosmicFont};
use bevy_ui_borders::BorderColor;

use crate::{
    ui_plugin::{
        ui_helpers::{
            get_tooltip, GenericButton, SearchButton, SearchText, Tooltip, TooltipPosition,
        },
        TextPos,
    },
    utils::{to_cosmic_text_pos, ReflectableUuid},
};

pub fn add_search_box(
    commands: &mut Commands,
    cosmic_fonts: &mut ResMut<Assets<CosmicFont>>,
    cosmic_font_handle: Handle<CosmicFont>,
    scale_factor: f32,
) -> Entity {
    let id = ReflectableUuid::generate();
    let root = commands
        .spawn((NodeBundle {
            background_color: Color::WHITE.into(),
            style: Style {
                size: Size::new(Val::Percent(80.), Val::Percent(8.)),
                flex_direction: FlexDirection::Column,
                margin: UiRect::all(Val::Px(5.)),
                ..default()
            },
            ..default()
        },))
        .id();
    let cosmic_edit_meta = CosmicEditMeta {
        text: "".to_string(),
        text_pos: to_cosmic_text_pos(TextPos::Center),
        initial_background: None,
        font_size: 14.,
        line_height: 18.,
        scale_factor,
        font_system_handle: cosmic_font_handle,
        display_none: false,
        initial_size: None,
    };
    let cosmic_edit = spawn_cosmic_edit(commands, cosmic_fonts, cosmic_edit_meta);
    commands
        .entity(cosmic_edit)
        .insert(BorderColor(Color::GRAY.with_a(0.5)))
        .insert(SearchButton { id })
        .insert(GenericButton);
    commands.entity(cosmic_edit).insert(SearchText { id });
    let tooltip = commands
        .spawn((
            get_tooltip(
                "Filter documents by text in nodes".to_string(),
                14.,
                TooltipPosition::Top,
            ),
            Tooltip,
        ))
        .id();

    commands.entity(cosmic_edit).add_child(tooltip);
    commands.entity(root).add_child(cosmic_edit);
    root
}
