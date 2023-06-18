use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::*,
};
use bevy_cosmic_edit::{
    spawn_cosmic_edit, CosmicEditMeta, CosmicFont, CosmicMetrics, CosmicNode, CosmicText,
};
use cosmic_text::AttrsOwned;

use crate::{
    themes::Theme,
    ui_plugin::TextPos,
    utils::{bevy_color_to_cosmic, ReflectableUuid},
};

use super::{DeleteDoc, DocListItemButton, DocListItemContainer, EditableText, GenericButton};

pub fn add_list_item(
    commands: &mut Commands,
    cosmic_fonts: &mut ResMut<Assets<CosmicFont>>,
    cosmic_font_handle: Handle<CosmicFont>,
    theme: &Res<Theme>,
    asset_server: &Res<AssetServer>,
    id: ReflectableUuid,
    name: String,
    scale_factor: f32,
) -> Entity {
    let icon_font = asset_server.load("fonts/MaterialIcons-Regular.ttf");
    let root = commands
        .spawn((
            ButtonBundle {
                border_color: theme.btn_border.into(),
                background_color: theme.doc_list_bg.into(),
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.)),
                    ..default()
                },
                ..default()
            },
            GenericButton,
            DocListItemContainer { id },
            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
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
        bg: theme.doc_list_bg,
        node: CosmicNode::Ui,
        readonly: true,
        bg_image: None,
    };
    let cosmic_edit = spawn_cosmic_edit(commands, cosmic_fonts, cosmic_edit_meta);
    commands
        .entity(cosmic_edit)
        .insert(EditableText { id })
        .insert(Label)
        .insert(GenericButton)
        .insert(DocListItemButton { id });

    let del_button = commands
        .spawn((
            ButtonBundle {
                visibility: Visibility::Hidden,
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
                    padding: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                ..default()
            },
            DeleteDoc { id },
            GenericButton,
        ))
        .id();
    let del_label = commands
        .spawn((
            TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "\u{e14c}".to_string(),
                        style: TextStyle {
                            font_size: 24.,
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
