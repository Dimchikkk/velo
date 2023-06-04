use bevy::prelude::*;
use bevy_cosmic_edit::{bevy_color_to_cosmic, spawn_cosmic_edit, CosmicEditMeta, CosmicFont};
use bevy_ui_borders::BorderColor;

use crate::{
    themes::Theme,
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
    theme: &Res<Theme>,
    cosmic_fonts: &mut ResMut<Assets<CosmicFont>>,
    cosmic_font_handle: Handle<CosmicFont>,
    scale_factor: f32,
) -> Entity {
    let id = ReflectableUuid::generate();
    let root = commands
        .spawn((NodeBundle {
            background_color: theme.search_box_bg.into(),
            style: Style {
                size: Size::new(Val::Percent(80.), Val::Percent(8.)),
                flex_direction: FlexDirection::Column,
                margin: UiRect::all(Val::Px(5.)),
                ..default()
            },
            ..default()
        },))
        .id();
    let mut attrs = cosmic_text::Attrs::new();
    attrs = attrs.family(cosmic_text::Family::Name(theme.font_name.as_str()));
    attrs = attrs.color(bevy_color_to_cosmic(theme.font));
    let metrics = cosmic_text::Metrics::new(14., 18.).scale(scale_factor);
    let cosmic_edit_meta = CosmicEditMeta {
        text: "".to_string(),
        text_pos: to_cosmic_text_pos(TextPos::Center),
        initial_background: None,
        font_system_handle: cosmic_font_handle,
        display_none: false,
        initial_size: None,
        attrs,
        metrics,
    };
    let cosmic_edit = spawn_cosmic_edit(commands, cosmic_fonts, cosmic_edit_meta);
    commands
        .entity(cosmic_edit)
        .insert(BorderColor(theme.search_box_border))
        .insert(SearchButton { id })
        .insert(GenericButton);
    commands.entity(cosmic_edit).insert(SearchText { id });
    let tooltip = commands
        .spawn((
            get_tooltip(
                theme,
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
