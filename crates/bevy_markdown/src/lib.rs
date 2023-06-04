use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use syntect::easy::HighlightLines;
use syntect::highlighting::{FontStyle, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use uuid::Uuid;

pub struct BevyMarkdownFonts {
    pub regular_font: Handle<Font>,
    pub bold_font: Handle<Font>,
    pub italic_font: Handle<Font>,
    pub semi_bold_italic_font: Handle<Font>,
    pub extra_bold_font: Handle<Font>,
    pub code_font: Handle<Font>,
}

pub struct BevyMarkdownTheme {
    pub code_theme: String,
    pub code_default_lang: String,
    pub font: Color,
    pub link: Color,
    pub inline_code: Color,
}

#[derive(Default)]
pub struct BevyMarkdown {
    pub text: String,
    pub fonts: Option<BevyMarkdownFonts>,
    pub theme: Option<BevyMarkdownTheme>,
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
        InlineStyleType::Strong => bevy_markdown.fonts.as_ref().unwrap().bold_font.clone(),
        InlineStyleType::Emphasis => bevy_markdown.fonts.as_ref().unwrap().italic_font.clone(),
        InlineStyleType::StrongEmphasis => bevy_markdown
            .fonts
            .as_ref()
            .unwrap()
            .semi_bold_italic_font
            .clone(),
        _ => bevy_markdown.fonts.as_ref().unwrap().regular_font.clone(),
    }
}

pub fn get_bullet_for_indentation_level(level: u8) -> &'static str {
    let level = level % 3;
    if level == 0 {
        " • "
    } else if level == 1 {
        " ◦ "
    } else {
        " ▪ "
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
                        font: bevy_markdown.fonts.as_ref().unwrap().regular_font.clone(),
                        font_size: 18.0,
                        color: bevy_markdown.theme.as_ref().unwrap().font.clone(),
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
                        font: bevy_markdown.fonts.as_ref().unwrap().regular_font.clone(),
                        font_size: 18.0,
                        color: bevy_markdown.theme.as_ref().unwrap().font.clone(),
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
                                font: bevy_markdown.fonts.as_ref().unwrap().regular_font.clone(),
                                font_size: 18.0,
                                color: bevy_markdown.theme.as_ref().unwrap().font.clone(),
                            },
                        },
                        None,
                    ));
                }
                markdown::mdast::Node::Text(text) => {
                    let text_section = TextSection {
                        value: text.value.clone(),
                        style: TextStyle {
                            font: bevy_markdown.fonts.as_ref().unwrap().regular_font.clone(),
                            font_size: 18.0,
                            color: bevy_markdown.theme.as_ref().unwrap().font.clone(),
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
                    font: bevy_markdown.fonts.as_ref().unwrap().code_font.clone(),
                    font_size: if let Some(size) = force_size {
                        size
                    } else {
                        18.0
                    },
                    color: if let Some(color) = force_color {
                        color
                    } else {
                        bevy_markdown.theme.as_ref().unwrap().inline_code.clone()
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
                        bevy_markdown.theme.as_ref().unwrap().font.clone()
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
                Some(bevy_markdown.theme.as_ref().unwrap().link.clone()),
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

fn handle_list_recursive(
    list: &markdown::mdast::List,
    bevy_markdown: &BevyMarkdown,
    text_sections: &mut Vec<(TextSection, Option<String>)>,
    errors: &mut Vec<BevyMarkdownError>,
    indentation_level: u8,
) -> Result<(), Vec<BevyMarkdownError>> {
    text_sections.push((
        TextSection {
            value: "\n".to_string(),
            style: TextStyle {
                font: bevy_markdown.fonts.as_ref().unwrap().regular_font.clone(),
                font_size: 18.0,
                color: Color::NONE,
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
                for _ in 0..indentation_level {
                    text_sections.push((
                        TextSection {
                            value: "    ".to_string(),
                            ..Default::default()
                        },
                        None,
                    ));
                }

                let indent_char = if list.ordered {
                    let index = list_index.unwrap();
                    list_index = Some(index + 1);
                    format!(" {}. ", index)
                } else {
                    get_bullet_for_indentation_level(indentation_level).to_string()
                };

                text_sections.push((
                    TextSection {
                        value: indent_char,
                        style: TextStyle {
                            font: bevy_markdown.fonts.as_ref().unwrap().regular_font.clone(),
                            font_size: 18.0,
                            color: bevy_markdown.theme.as_ref().unwrap().font.clone(),
                        },
                    },
                    None,
                ));

                item.children.into_iter().for_each(|child| match child {
                    markdown::mdast::Node::Paragraph(paragraph) => {
                        paragraph.children.iter().for_each(|child| {
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
                        })
                    }
                    markdown::mdast::Node::List(inner_list) => {
                        let _ = handle_list_recursive(
                            &inner_list,
                            bevy_markdown,
                            text_sections,
                            errors,
                            indentation_level + 1,
                        );
                    }
                    node => errors.push(BevyMarkdownError::Transform {
                        info: format!("{:?} node is not implemented for list item", node),
                    }),
                });

                text_sections.push((
                    TextSection {
                        value: "\n".to_string(),
                        style: TextStyle {
                            font: bevy_markdown.fonts.as_ref().unwrap().regular_font.clone(),
                            font_size: 18.0,
                            color: Color::NONE,
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
        });
    Ok(())
}

pub fn spawn_bevy_markdown(
    commands: &mut Commands,
    bevy_markdown: BevyMarkdown,
) -> Result<Entity, Vec<BevyMarkdownError>> {
    let node = markdown::to_mdast(bevy_markdown.text.as_str(), &markdown::ParseOptions::gfm());
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
                            let default_lang = bevy_markdown
                                .theme
                                .as_ref()
                                .unwrap()
                                .code_default_lang
                                .clone();
                            let lang = code.lang.as_ref().unwrap_or(&default_lang);
                            let syntax = vec![
                                ps.find_syntax_by_name(lang.as_str()),
                                ps.find_syntax_by_extension(lang.as_str()),
                            ]
                            .iter()
                            .find(|&o| o.is_some())
                            .unwrap()
                            .unwrap();
                            let mut h = HighlightLines::new(
                                syntax,
                                &ts.themes
                                    [&bevy_markdown.theme.as_ref().unwrap().code_theme.clone()],
                            );
                            text_sections.push((
                                TextSection {
                                    value: "\n\n".to_string(),
                                    style: TextStyle {
                                        font: bevy_markdown
                                            .fonts
                                            .as_ref()
                                            .unwrap()
                                            .regular_font
                                            .clone(),
                                        font_size: 18.0,
                                        color: Color::NONE,
                                    },
                                },
                                None,
                            ));
                            for line in LinesWithEndings::from(code.value.as_str()) {
                                let ranges: Vec<(syntect::highlighting::Style, &str)> =
                                    h.highlight_line(line, &ps).unwrap();

                                for &(style, text) in ranges.iter() {
                                    let font = match style.font_style {
                                        FontStyle::BOLD => bevy_markdown
                                            .fonts
                                            .as_ref()
                                            .unwrap()
                                            .extra_bold_font
                                            .clone(),
                                        FontStyle::ITALIC => bevy_markdown
                                            .fonts
                                            .as_ref()
                                            .unwrap()
                                            .italic_font
                                            .clone(),
                                        FontStyle::UNDERLINE => bevy_markdown
                                            .fonts
                                            .as_ref()
                                            .unwrap()
                                            .regular_font
                                            .clone(),
                                        _ => {
                                            bevy_markdown.fonts.as_ref().unwrap().bold_font.clone()
                                        }
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
                                        font: bevy_markdown
                                            .fonts
                                            .as_ref()
                                            .unwrap()
                                            .regular_font
                                            .clone(),
                                        font_size: 18.0,
                                        color: Color::NONE,
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
                            let _ = handle_list_recursive(
                                list,
                                &bevy_markdown,
                                &mut text_sections,
                                &mut errors,
                                0,
                            );
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
                let top_style = Style::default();
                let mut text_bundle_style = Style::default();
                // Main branch of bevy doesn't need setting max_size for wrapping to work
                // bevy_markdown will spawn multiple text bundles with more markdown features supported (e.g. inline images)
                // TODO: adjust it after moving to 0.11 bevy
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
                let fonts = BevyMarkdownFonts {
                    regular_font: font.clone(),
                    bold_font: font.clone(),
                    italic_font: font.clone(),
                    semi_bold_italic_font: font.clone(),
                    extra_bold_font: font.clone(),
                    code_font: font.clone(),
                };
                let theme = BevyMarkdownTheme {
                    code_theme: "Solarized (light)".to_string(),
                    code_default_lang: "rs".to_string(),
                    font: Color::BLACK,
                    link: Color::BLUE,
                    inline_code: Color::GRAY,
                };
                let bevy_markdown = BevyMarkdown {
                    text: input.clone(),
                    fonts: Some(fonts),
                    theme: Some(theme),
                    size: None,
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

    #[test]
    pub fn test_render_nested_unordered_list() {
        let input = "
- Import a HTML file and watch it magically convert to Markdown
    - Drag and drop images (requires your Dropbox account be linked)
- Import and save files from GitHub, Dropbox, Google Drive and One Drive
    - Drag and drop markdown and HTML files into Dillinger
- Export documents as Markdown, HTML and PDF
"
        .to_string();
        test_bevymarkdown(input, "test_render_nested_unordered_list".to_string())
    }

    #[test]
    pub fn test_render_nested_ordered_list() {
        let input = "
1. Import a HTML file and watch it magically convert to Markdown
2. Drag and drop images (requires your Dropbox account be linked)
    1. Import and save files from GitHub, Dropbox, Google Drive and One Drive
    2. Drag and drop images (requires your Dropbox account be linked)
3. Drag and drop images (requires your Dropbox account be linked)
"
        .to_string();
        test_bevymarkdown(input, "test_render_nested_ordered_list".to_string())
    }
}
