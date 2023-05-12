use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use syntect::easy::HighlightLines;
use syntect::highlighting::{FontStyle, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use uuid::Uuid;

#[derive(Default)]
pub struct BevyMarkdown {
    pub text: String,
    pub regular_font: Option<Handle<Font>>,
    pub bold_font: Option<Handle<Font>>,
    pub italic_font: Option<Handle<Font>>,
    pub semi_bold_italic_font: Option<Handle<Font>>,
    pub extra_bold_font: Option<Handle<Font>>,
    pub size: Option<(Val, Val)>,
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

pub fn spawn_bevy_markdown(
    commands: &mut Commands,
    bevy_markdown: BevyMarkdown,
) -> Result<Entity, Vec<BevyMarkdownError>> {
    let node = markdown::to_mdast(bevy_markdown.text.as_str(), &markdown::ParseOptions::gfm());
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    match node {
        Ok(node) => {
            let mut text_sections = Vec::new();
            let mut errors = Vec::new();
            match node {
                markdown::mdast::Node::Root(root) => {
                    root.children.iter().for_each(|child| match child {
                        markdown::mdast::Node::Code(code) => {
                            let default_lang = "rs".to_string();
                            let lang = code.lang.as_ref().unwrap_or(&default_lang);
                            let syntax = vec![
                                ps.find_syntax_by_name(lang.as_str()),
                                ps.find_syntax_by_extension(lang.as_str()),
                            ]
                            .iter()
                            .find(|&o| o.is_some())
                            .unwrap()
                            .unwrap();
                            let mut h =
                                HighlightLines::new(syntax, &ts.themes["Solarized (light)"]);
                            text_sections.push((
                                TextSection {
                                    value: "\n\n".to_string(),
                                    style: TextStyle {
                                        font: bevy_markdown.regular_font.clone().unwrap(),
                                        font_size: 18.0,
                                        color: Color::Rgba {
                                            red: 0.,
                                            green: 0.,
                                            blue: 0.,
                                            alpha: 0.,
                                        },
                                    },
                                },
                                None,
                            ));
                            for line in LinesWithEndings::from(code.value.as_str()) {
                                let ranges: Vec<(syntect::highlighting::Style, &str)> =
                                    h.highlight_line(line, &ps).unwrap();

                                for &(style, text) in ranges.iter() {
                                    let font = match style.font_style {
                                        FontStyle::BOLD => {
                                            bevy_markdown.extra_bold_font.clone().unwrap()
                                        }
                                        FontStyle::ITALIC => {
                                            bevy_markdown.italic_font.clone().unwrap()
                                        }
                                        FontStyle::UNDERLINE => {
                                            bevy_markdown.regular_font.clone().unwrap()
                                        }
                                        _ => bevy_markdown.bold_font.clone().unwrap(),
                                    };
                                    let color = style.foreground;
                                    let text_section = TextSection {
                                        value: text.to_string(),
                                        style: TextStyle {
                                            font,
                                            font_size: 18.0,
                                            color: Color::Rgba {
                                                red: color.r as f32 / 255.,
                                                green: color.g as f32 / 255.,
                                                blue: color.b as f32 / 255.,
                                                alpha: color.a as f32 / 255.,
                                            },
                                        },
                                    };
                                    text_sections.push((text_section, None));
                                }
                            }
                            text_sections.push((
                                TextSection {
                                    value: "\n".to_string(),
                                    style: TextStyle {
                                        font: bevy_markdown.regular_font.clone().unwrap(),
                                        font_size: 18.0,
                                        color: Color::Rgba {
                                            red: 0.,
                                            green: 0.,
                                            blue: 0.,
                                            alpha: 0.,
                                        },
                                    },
                                },
                                None,
                            ));
                        }
                        markdown::mdast::Node::Paragraph(paragraph) => {
                            text_sections.push((
                                TextSection {
                                    value: "\n".to_string(),
                                    style: TextStyle {
                                        font: bevy_markdown.regular_font.clone().unwrap(),
                                        font_size: 18.0,
                                        color: Color::BLACK,
                                    },
                                },
                                None,
                            ));
                            paragraph.children.iter().for_each(|child| match child {
                                markdown::mdast::Node::Break(_break) => {
                                    text_sections.push((
                                        TextSection {
                                            value: "\n".to_string(),
                                            style: TextStyle {
                                                font: bevy_markdown.regular_font.clone().unwrap(),
                                                font_size: 18.0,
                                                color: Color::BLACK,
                                            },
                                        },
                                        None,
                                    ));
                                }
                                markdown::mdast::Node::Text(text) => {
                                    let text_section = TextSection {
                                        value: text.value.clone(),
                                        style: TextStyle {
                                            font: bevy_markdown.regular_font.clone().unwrap(),
                                            font_size: 18.0,
                                            color: Color::BLACK,
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
            if !errors.is_empty() {
                Err(errors)
            } else {
                let mut sections = Vec::new();
                let mut links = Vec::new();
                for (section, link) in text_sections {
                    sections.push(section);
                    links.push(link);
                }
                let text_bundle_id = Uuid::new_v4();
                let top_style = Style {
                    padding: UiRect::all(Val::Px(10.)),
                    ..default()
                };
                let mut text_bundle_style = Style::default();
                // Main branch of bevy doesn't need setting max_size for wrapping to work
                // bevy_markdown will spawn multiple text bundles with more markdown features supported
                // this is temp solution make wrapping to work
                if let Some((x, y)) = bevy_markdown.size {
                    text_bundle_style.max_size = Size::new(x, y);
                }
                let top = commands
                    .spawn(NodeBundle {
                        style: top_style,
                        ..default()
                    })
                    .id();
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

    fn test_bevymarkdown(input: String, test_name: String) {
        let test_render_text_style_system =
            move |mut commands: Commands, asset_server: Res<AssetServer>| {
                let font = asset_server.load("fonts/SourceCodePro-Regular.ttf");
                let bevy_markdown = BevyMarkdown {
                    regular_font: Some(font.clone()),
                    bold_font: Some(font.clone()),
                    italic_font: Some(font.clone()),
                    semi_bold_italic_font: Some(font.clone()),
                    extra_bold_font: Some(font.clone()),
                    size: None,
                    text: input.clone(),
                };
                spawn_bevy_markdown(&mut commands, bevy_markdown).unwrap();
            };

        let mut app = App::new();
        app.add_plugin(TaskPoolPlugin::default());
        app.add_plugin(AssetPlugin::default());
        app.add_system(test_render_text_style_system);

        app.update();

        let mut text_nodes_query = app.world.query::<&Text>();
        for node in text_nodes_query.iter(&app.world) {
            insta::assert_debug_snapshot!(
                format!("{}_{}", test_name.clone(), "node"),
                node.clone()
            );
        }
        let mut bevy_markdown_query = app.world.query::<&BevyMarkdownNode>();
        for node in bevy_markdown_query.iter(&app.world) {
            insta::assert_debug_snapshot!(
                format!("{}_{}", test_name.clone(), "links"),
                node.link_sections.clone()
            );
        }
    }

    #[test]
    pub fn test_render_text_style() {
        let input = "**bold1**  
__bold2__
*italic1*
_italic2_
Hello world
[link](https://example.com)
    ";
        test_bevymarkdown(input.to_string(), "test_render_text_style".to_string());
    }

    #[test]
    pub fn test_render_code() {
        let input = "My rust code:

```rs
fn main() {
    println!(\"Hello world\");
}
```
    "
        .to_string();
        test_bevymarkdown(input.to_string(), "test_render_code".to_string());
    }

    #[test]
    pub fn test_render_break() {
        let input = "Hello world  
hello world
    "
        .to_string();
        test_bevymarkdown(input.to_string(), "test_render_break".to_string());
    }

    #[test]
    pub fn test_render_break_after_link() {
        let input = "(link)[https://example.com]]   

hello world
    "
        .to_string();
        test_bevymarkdown(
            input.to_string(),
            "test_render_break_after_link".to_string(),
        );
    }
}
