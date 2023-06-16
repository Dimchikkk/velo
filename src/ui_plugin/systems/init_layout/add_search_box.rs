use bevy::prelude::*;
use bevy_cosmic_edit::{
    spawn_cosmic_edit, CosmicEditMeta, CosmicFont, CosmicMetrics, CosmicNode, CosmicText,
};
use cosmic_text::AttrsOwned;

use crate::{
    themes::Theme,
    ui_plugin::{
        ui_helpers::{
            get_tooltip, GenericButton, SearchButton, SearchText, Tooltip, TooltipPosition,
        },
        TextPos,
    },
    utils::{bevy_color_to_cosmic, ReflectableUuid},
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
                width: Val::Percent(80.),
                height: Val::Px(8.),
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
    let cosmic_edit_meta = CosmicEditMeta {
        text: CosmicText::OneStyle("".to_string()),
        attrs: AttrsOwned::new(attrs),
        text_pos: TextPos::Center.into(),
        font_system_handle: cosmic_font_handle,
        node: CosmicNode::Ui,
        size: None,
        metrics: CosmicMetrics {
            font_size: 14.,
            line_height: 18.,
            scale_factor,
        },
        bg: theme.search_box_bg,
        readonly: false,
        bg_image: None,
    };
    let cosmic_edit = spawn_cosmic_edit(commands, cosmic_fonts, cosmic_edit_meta);
    commands
        .entity(cosmic_edit)
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
