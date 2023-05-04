use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Default)]
pub struct BevyMarkdown {
    pub text: String,
    pub regular_font: Option<Handle<Font>>,
    pub bold_font: Option<Handle<Font>>,
    pub italic_font: Option<Handle<Font>>,
    pub semi_bold_italic_font: Option<Handle<Font>>,
    pub max_size: Option<(Val, Val)>,
}

#[derive(Debug)]
pub enum BevyMarkdownError {
    Transform { info: String },
    Parsing { info: String },
}

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct BevyMarkdownNode {
    pub id: Uuid,
    pub link_sections: Vec<Option<String>>,
}

pub fn render_bevy_markdown(
    commands: &mut Commands,
    bevy_markdown: BevyMarkdown,
) -> Result<Entity, Vec<BevyMarkdownError>> {
    let node = markdown::to_mdast(bevy_markdown.text.as_str(), &markdown::ParseOptions::gfm());
    match node {
        Ok(node) => {
            let mut text_sections = Vec::new();
            let mut errors = Vec::new();
            match node {
                markdown::mdast::Node::Root(root) => {
                    root.children.iter().for_each(|child| match child {
                        markdown::mdast::Node::Paragraph(paragraph) => {
                            paragraph.children.iter().for_each(|child| match child {
                                markdown::mdast::Node::Text(text) => {
                                    let text_section = TextSection {
                                        value: text.value.clone(),
                                        style: TextStyle {
                                            font: bevy_markdown.regular_font.clone().unwrap(),
                                            font_size: 18.0,
                                            color: Color::BLACK,
                                            ..default()
                                        },
                                    };
                                    text_sections.push((text_section, None));
                                }
                                markdown::mdast::Node::Strong(strong) => {
                                    strong.children.iter().for_each(|child| match child {
                                        markdown::mdast::Node::Text(text) => {
                                            let text_section = TextSection {
                                                value: text.value.clone(),
                                                style: TextStyle {
                                                    font: bevy_markdown.bold_font.clone().unwrap(),
                                                    font_size: 18.0,
                                                    color: Color::BLACK,
                                                    ..default()
                                                },
                                            };
                                            text_sections.push((text_section, None));
                                        }
                                        _ => errors.push(BevyMarkdownError::Transform {
                                            info: "nesting in bold is not implemented".to_string(),
                                        }),
                                    });
                                }
                                markdown::mdast::Node::Emphasis(emphasis) => {
                                    emphasis.children.iter().for_each(|child| match child {
                                        markdown::mdast::Node::Text(text) => {
                                            let text_section = TextSection {
                                                value: text.value.clone(),
                                                style: TextStyle {
                                                    font: bevy_markdown
                                                        .italic_font
                                                        .clone()
                                                        .unwrap(),
                                                    font_size: 18.0,
                                                    color: Color::BLACK,
                                                    ..default()
                                                },
                                            };
                                            text_sections.push((text_section, None));
                                        }
                                        _ => errors.push(BevyMarkdownError::Transform {
                                            info: "nesting in italic is not implemented"
                                                .to_string(),
                                        }),
                                    });
                                }
                                markdown::mdast::Node::Link(link) => {
                                    link.children.iter().for_each(|child| match child {
                                        markdown::mdast::Node::Text(text) => {
                                            let text_section = TextSection {
                                                value: text.value.clone(),
                                                style: TextStyle {
                                                    font: bevy_markdown
                                                        .semi_bold_italic_font
                                                        .clone()
                                                        .unwrap(),
                                                    font_size: 18.0,
                                                    color: Color::BLUE,
                                                    ..default()
                                                },
                                            };
                                            text_sections
                                                .push((text_section, Some(link.url.clone())));
                                        }
                                        _ => errors.push(BevyMarkdownError::Transform {
                                            info: "nesting in link is not implemented".to_string(),
                                        }),
                                    });
                                }
                                node => errors.push(BevyMarkdownError::Transform {
                                    info: format!(
                                        "{:?} node is not implemented for paragraph",
                                        node
                                    ),
                                }),
                            });
                        }
                        node => errors.push(BevyMarkdownError::Transform {
                            info: format!("{:?} node is not implemented for root", node),
                        }),
                    });
                }
                node => errors.push(BevyMarkdownError::Transform {
                    info: format!("unexpected node: {:?}", node),
                }),
            }
            if errors.len() > 0 {
                return Err(errors);
            } else {
                let mut sections = Vec::new();
                let mut links = Vec::new();
                for (section, link) in text_sections {
                    sections.push(section);
                    links.push(link);
                }
                let text_bundle_id = Uuid::new_v4();
                let mut text_bundle_style = Style::default();
                // TODO: main branch of bevy doesn't need setting max_size for wrapping to work
                if let Some((x, y)) = bevy_markdown.max_size {
                    text_bundle_style.max_size = Size::new(x, y);
                }
                let top = commands.spawn(NodeBundle::default()).id();
                let text_bundle = commands
                    .spawn((
                        TextBundle {
                            text: Text {
                                sections,
                                ..default()
                            },
                            style: text_bundle_style,
                            ..default()
                        },
                        BevyMarkdownNode {
                            id: text_bundle_id,
                            link_sections: links,
                        },
                    ))
                    .id();
                commands.entity(top).add_child(text_bundle);
                Ok(top)
            }
        }
        Err(e) => Err(vec![BevyMarkdownError::Parsing { info: e }]),
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::*;

    fn test_render_text_style_system(mut commands: Commands, asset_server: Res<AssetServer>) {
        let text = "**bold1**
__bold2__
*italic1*
_italic2_
Hello world
[link](https://example.com)
    "
        .to_string();
        let font = asset_server.load("fonts/SourceCodePro-Regular.ttf");
        let bevy_markdown = BevyMarkdown {
            regular_font: Some(font.clone()),
            bold_font: Some(font.clone()),
            italic_font: Some(font.clone()),
            semi_bold_italic_font: Some(font.clone()),
            max_size: None,
            text,
        };
        render_bevy_markdown(&mut commands, bevy_markdown).unwrap();
    }
    #[test]
    pub fn test_render_text_style() {
        let mut app = App::new();
        app.add_plugin(TaskPoolPlugin::default());
        app.add_plugin(AssetPlugin::default());
        app.add_system(test_render_text_style_system);

        app.update();

        let mut text_nodes_query = app.world.query::<&Text>();
        for node in text_nodes_query.iter(&app.world) {
            insta::assert_debug_snapshot!(node.clone());
        }
        let mut bevy_markdown_query = app.world.query::<&BevyMarkdownNode>();
        for node in bevy_markdown_query.iter(&app.world) {
            insta::assert_debug_snapshot!(node.link_sections.clone());
        }
    }
}
