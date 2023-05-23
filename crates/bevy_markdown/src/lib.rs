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
    pub code_font: Option<Handle<Font>>,
    pub size: Option<(Val, Val)>,
}

#[repr(u8)]
#[derive(Clone)]
enum InlineStyleType {
    Strong = 0x01,
    Emphasis = 0x02,
    StrongEmphasis = 0x03,
    // StrikeThrough = 0x04,
    // StrikeBold = 0x05,
    // StrikeItalic = 0x06,
    // StrikeBoldItalic = 0x07,
    None = 0x00,
}

pub fn get_header_font_size(val: u8) -> f32 {
    match val {
        1 => 30.0,
        2 => 27.0,
        3 => 24.0,
        4 => 21.0,
        5 => 18.0,
        6 => 15.0,
        _ => 15.0,
    }
}

impl InlineStyleType {
    #[inline]
    pub fn from_u8(style_code: u8) -> Self {
        match style_code {
            0x01 => InlineStyleType::Strong,
            0x02 => InlineStyleType::Emphasis,
            0x03 => InlineStyleType::StrongEmphasis,
            // 0x04 => InlineStyleType::StrikeThrough,
            // 0x05 => InlineStyleType::StrikeBold,
            // 0x06 => InlineStyleType::StrikeItalic,
            // 0x07 => InlineStyleType::StrikeBoldItalic,
            _ => InlineStyleType::None,
        }
    }
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

pub fn get_resultant_style(
    bevy_markdown: &BevyMarkdown,
    style_mask: u8,
) -> bevy::prelude::Handle<bevy::prelude::Font> {
    match InlineStyleType::from_u8(style_mask) {
        InlineStyleType::Strong => bevy_markdown.bold_font.clone().unwrap(),
        InlineStyleType::Emphasis => bevy_markdown.italic_font.clone().unwrap(),
        InlineStyleType::StrongEmphasis => bevy_markdown.semi_bold_italic_font.clone().unwrap(),
        _ => bevy_markdown.regular_font.clone().unwrap(),
    }
}

pub fn handle_block_styling(
    node: &markdown::mdast::Node,
    bevy_markdown: &BevyMarkdown,
    text_sections: &mut Vec<(TextSection, Option<String>)>,
    errors: &mut Vec<BevyMarkdownError>,
) -> Result<(), Vec<BevyMarkdownError>> {
    match node {
        markdown::mdast::Node::Heading(header) => {
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
            header.children.iter().for_each(|child| {
                let _ = handle_inline_styling(
                    child,
                    bevy_markdown,
                    text_sections,
                    errors,
                    InlineStyleType::Strong as u8,
                    None,
                    Some(get_header_font_size(header.depth)),
                    &None,
                );
            });
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
                markdown::mdast::Node::Break(_) => {
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
                markdown::mdast::Node::Strong(_)
                | markdown::mdast::Node::Emphasis(_)
                | markdown::mdast::Node::InlineCode(_)
                | markdown::mdast::Node::Delete(_)
                | markdown::mdast::Node::Link(_) => {
                    let _ = handle_inline_styling(
                        child,
                        bevy_markdown,
                        text_sections,
                        errors,
                        InlineStyleType::None as u8,
                        None,
                        None,
                        &None,
                    );
                }
                node => errors.push(BevyMarkdownError::Transform {
                    info: format!("{:?} node is not implemented for paragraph", node),
                }),
            });
        }
        _ => errors.push(BevyMarkdownError::Transform {
            info: "nesting is not implemented".to_string(),
        }),
    }
    Ok(())
}

pub fn handle_inline_styling(
    node: &markdown::mdast::Node,
    bevy_markdown: &BevyMarkdown,
    text_sections: &mut Vec<(TextSection, Option<String>)>,
    errors: &mut Vec<BevyMarkdownError>,
    applied_style: u8,
    force_color: Option<Color>,
    force_size: Option<f32>,
    force_data: &Option<String>,
) -> Result<(), Vec<BevyMarkdownError>> {
    match node {
        markdown::mdast::Node::InlineCode(code) => {
            let text_section = TextSection {
                value: code.value.clone(),
                style: TextStyle {
                    font: bevy_markdown.code_font.clone().unwrap(),
                    font_size: if let Some(size) = force_size {
                        size
                    } else {
                        18.0
                    },
                    color: if let Some(color) = force_color {
                        color
                    } else {
                        Color::GRAY
                    },
                },
            };
            text_sections.push((text_section, force_data.clone()));
        }
        markdown::mdast::Node::Emphasis(emphasis) => emphasis.children.iter().for_each(|child| {
            let _ = handle_inline_styling(
                child,
                bevy_markdown,
                text_sections,
                errors,
                applied_style | InlineStyleType::Emphasis as u8,
                force_color,
                force_size,
                force_data,
            );
        }),
        markdown::mdast::Node::Strong(strong) => strong.children.iter().for_each(|child| {
            let _ = handle_inline_styling(
                child,
                bevy_markdown,
                text_sections,
                errors,
                applied_style | InlineStyleType::Strong as u8,
                force_color,
                force_size,
                force_data,
            );
        }),
        markdown::mdast::Node::Text(text) => {
            let text_section = TextSection {
                value: text.value.clone(),
                style: TextStyle {
                    font: get_resultant_style(bevy_markdown, applied_style),
                    font_size: if let Some(size) = force_size {
                        size
                    } else {
                        18.0
                    },
                    color: if let Some(color) = force_color {
                        color
                    } else {
                        Color::BLACK
                    },
                },
            };
            text_sections.push((text_section, force_data.clone()));
        }
        markdown::mdast::Node::Link(link) => link.children.iter().for_each(|child| {
            let _ = handle_inline_styling(
                child,
                bevy_markdown,
                text_sections,
                errors,
                applied_style,
                Some(Color::BLUE),
                force_size,
                &Some(link.url.clone()),
            );
        }),
        _ => {
            errors.push(BevyMarkdownError::Transform {
                info: "nesting is not implemented".to_string(),
            });
        }
    }
    Ok(())
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
                        markdown::mdast::Node::Heading(_) | markdown::mdast::Node::Paragraph(_) => {
                            let _ = handle_block_styling(
                                child,
                                &bevy_markdown,
                                &mut text_sections,
                                &mut errors,
                            );
                        }
                        markdown::mdast::Node::List(list) => {
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

                            let mut list_index = list.start;
                            list.children
                                .clone()
                                .into_iter()
                                .for_each(|node| match node {
                                    markdown::mdast::Node::ListItem(item) => {
                                        text_sections.push((
                                            TextSection {
                                                value: "\n".to_string(),
                                                style: TextStyle {
                                                    font: bevy_markdown
                                                        .regular_font
                                                        .clone()
                                                        .unwrap(),
                                                    font_size: 12.0,
                                                    color: Color::BLACK,
                                                },
                                            },
                                            None,
                                        ));

                                        let indent_char = if list.ordered {
                                            let index = list_index.unwrap();
                                            list_index = Some(index + 1);
                                            format!("\t {}. ", index)
                                        } else {
                                            "\t • ".to_string()
                                        };

                                        text_sections.push((
                                            TextSection {
                                                value: indent_char,
                                                style: TextStyle {
                                                    font: bevy_markdown
                                                        .regular_font
                                                        .clone()
                                                        .unwrap(),
                                                    font_size: 18.0,
                                                    color: Color::BLACK,
                                                },
                                            },
                                            None,
                                        ));

                                        item.children.into_iter().for_each(|child| match child {
                                            markdown::mdast::Node::Paragraph(paragraph) => {
                                                paragraph.children.iter().for_each(|child| {
                                                    let _ = handle_inline_styling(
                                                        child,
                                                        &bevy_markdown,
                                                        &mut text_sections,
                                                        &mut errors,
                                                        InlineStyleType::None as u8,
                                                        None,
                                                        None,
                                                        &None,
                                                    );
                                                })
                                            }
                                            node => errors.push(BevyMarkdownError::Transform {
                                                info: format!(
                                                    "{:?} node is not implemented for list item",
                                                    node
                                                ),
                                            }),
                                        });

                                        text_sections.push((
                                            TextSection {
                                                value: "\n".to_string(),
                                                style: TextStyle {
                                                    font: bevy_markdown
                                                        .regular_font
                                                        .clone()
                                                        .unwrap(),
                                                    font_size: 18.0,
                                                    color: Color::BLACK,
                                                },
                                            },
                                            None,
                                        ));
                                    }
                                    _ => {
                                        errors.push(BevyMarkdownError::Transform {
                                            info: "invalid list children".to_string(),
                                        });
                                    }
                                })
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
                    code_font: Some(font.clone()),
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
    pub fn test_render_text_complicated() {
        let input = "**bold1** normal text
**Italic* and then italic again*
[Inner links **can be styled*too***](https://google.com)
    ";
        test_bevymarkdown(
            input.to_string(),
            "test_render_text_style_complicated".to_string(),
        );
    }

    #[test]
    pub fn test_render_text_with_header() {
        let input = "# Header 1
## Header 2
### Header 3
#### Some header 4 *as well* italicised
##### another header [*redirecting to google*](https://google.com)
";
        test_bevymarkdown(
            input.to_string(),
            "test_render_text_style_header".to_string(),
        );
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
        let input = "(link)[https://example.com]   

hello world
    "
        .to_string();
        test_bevymarkdown(
            input.to_string(),
            "test_render_break_after_link".to_string(),
        );
    }

    #[test]
    pub fn test_render_unordered_list() {
        let input = "
- Import a HTML file and watch it magically convert to Markdown
- Drag and drop images (requires your Dropbox account be linked)
- Import and save files from GitHub, Dropbox, Google Drive and One Drive
- Drag and drop markdown and HTML files into Dillinger
- Export documents as Markdown, HTML and PDF
"
        .to_string();
        test_bevymarkdown(input, "test_render_unordered_list".to_string())
    }

    #[test]
    pub fn test_render_ordered_list() {
        let input = "
1. Import a HTML file and watch it magically convert to Markdown
2. Drag and drop images (requires your Dropbox account be linked)
3. Import and save files from GitHub, Dropbox, Google Drive and One Drive
"
        .to_string();
        test_bevymarkdown(input, "test_render_ordered_list".to_string())
    }
}
