use bevy::{prelude::*, ui::FocusPolicy};
use bevy_cosmic_edit::{spawn_cosmic_edit, CosmicEditMeta};
use bevy_ui_borders::BorderColor;
use cosmic_text::FontSystem;

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
    font_system: &mut FontSystem,
    scale_factor: f32,
) -> Entity {
    let id = ReflectableUuid::generate();
    let root = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Percent(80.), Val::Percent(8.)),
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                ..default()
            },
            GenericButton,
        ))
        .id();
    let button = commands
        .spawn((
            ButtonBundle {
                focus_policy: FocusPolicy::Pass,
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            SearchButton { id },
        ))
        .id();
    let cosmic_edit_meta = CosmicEditMeta {
        text: "".to_string(),
        text_pos: to_cosmic_text_pos(TextPos::Center),
        initial_background: None,
        font_size: 14.,
        line_height: 18.,
        scale_factor,
        font_system,
        is_visible: true,
        initial_size: None,
    };
    let cosmic_edit = spawn_cosmic_edit(commands, cosmic_edit_meta);
    commands
        .entity(cosmic_edit)
        .insert(BorderColor(Color::GRAY.with_a(0.5)));
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

    commands.entity(button).add_child(cosmic_edit);
    commands.entity(root).add_child(button);
    commands.entity(root).add_child(tooltip);
    root
}
