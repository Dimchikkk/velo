use bevy::prelude::*;
use bevy_cosmic_edit::{
    spawn_cosmic_edit, CosmicEditMeta, CosmicFont, CosmicMetrics, CosmicNode, CosmicText,
};
use cosmic_text::AttrsOwned;

use crate::{
    themes::Theme,
    ui_plugin::TextPos,
    utils::{bevy_color_to_cosmic, ReflectableUuid},
};

use super::{DeleteTab, EditableText, GenericButton, TabButton, TabContainer};

pub fn add_tab(
    commands: &mut Commands,
    cosmic_fonts: &mut ResMut<Assets<CosmicFont>>,
    cosmic_font_handle: Handle<CosmicFont>,
    theme: &Res<Theme>,
    asset_server: &Res<AssetServer>,
    name: String,
    id: ReflectableUuid,
    is_active: bool,
    scale_factor: f32,
) -> Entity {
    let icon_font = asset_server.load("fonts/MaterialIcons-Regular.ttf");
    let root = commands
        .spawn((
            NodeBundle {
                background_color: theme.add_tab_bg.into(),
                style: Style {
                    width: Val::Percent(8.),
                    height: Val::Percent(90.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
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
            TabContainer { id },
        ))
        .id();
    let mut attrs = cosmic_text::Attrs::new();
    attrs = attrs.family(cosmic_text::Family::Name(theme.font_name.as_str()));
    attrs = attrs.color(bevy_color_to_cosmic(theme.font));
    let cosmic_edit_meta = CosmicEditMeta {
        text: CosmicText::OneStyle(name),
        attrs: AttrsOwned::new(attrs),
        font_system_handle: cosmic_font_handle,
        text_pos: TextPos::Center.into(),
        size: None,
        metrics: CosmicMetrics {
            font_size: theme.font_size,
            line_height: theme.line_height,
            scale_factor,
        },
        bg: theme.add_tab_bg,
        node: CosmicNode::Ui,
        readonly: true,
        bg_image: None,
    };
    let cosmic_edit = spawn_cosmic_edit(commands, cosmic_fonts, cosmic_edit_meta);
    commands
        .entity(cosmic_edit)
        .insert(EditableText { id })
        .insert(GenericButton)
        .insert(TabButton { id });
    let del_button = commands
        .spawn((
            ButtonBundle {
                background_color: theme.add_tab_bg.into(),
                visibility: if is_active {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                },
                style: Style {
                    margin: UiRect {
                        left: Val::Px(3.),
                        right: Val::Px(3.),
                        top: Val::Px(0.),
                        bottom: Val::Px(0.),
                    },
                    width: Val::Percent(10.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            GenericButton,
            DeleteTab { id },
        ))
        .id();
    let del_label = commands
        .spawn((
            TextBundle {
                style: Style { ..default() },
                text: Text {
                    sections: vec![TextSection {
                        value: "\u{e14c}".to_string(),
                        style: TextStyle {
                            font_size: 18.,
                            color: theme.del_button,
                            font: icon_font,
                        },
                    }],
                    ..default()
                },
                ..default()
            },
            Label,
        ))
        .id();
    commands.entity(del_button).add_child(del_label);
    commands.entity(root).add_child(cosmic_edit);
    commands.entity(root).add_child(del_button);
    root
}
