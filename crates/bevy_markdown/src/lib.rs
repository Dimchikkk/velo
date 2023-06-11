use std::vec;

use cosmic_text::{AttrsOwned, Color, Weight};
use syntect::easy::HighlightLines;
use syntect::highlighting::{FontStyle, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

pub struct BevyMarkdownTheme {
    pub code_theme: String,
    pub code_default_lang: String,
    pub link: cosmic_text::Color,
    pub inline_code: cosmic_text::Color,
}

#[derive(Clone, Debug, Default)]
pub struct TextSpanMetadata {
    pub link: Option<String>,
}

#[inline]
pub fn default<T: Default>() -> T {
    std::default::Default::default()
}

#[derive(Default)]
pub struct TextSpan {
    pub text: String,
    pub font_size: Option<f32>,
    pub weigth: Option<Weight>,
    pub style: Option<cosmic_text::Style>,
    pub color: Option<cosmic_text::Color>,
    pub metadata: Option<TextSpanMetadata>,
}

pub struct BevyMarkdown {
    pub markdown_theme: BevyMarkdownTheme,
    pub text: String,
    pub attrs: AttrsOwned,
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
    text_spans: &mut Vec<TextSpan>,
    errors: &mut Vec<BevyMarkdownError>,
) -> Result<(), Vec<BevyMarkdownError>> {
    match node {
        markdown::mdast::Node::Heading(header) => {
            text_spans.push(TextSpan {
                text: "\n".to_string(),
                ..default()
            });
            header.children.iter().for_each(|child| {
                let _ = handle_inline_styling(
                    child,
                    bevy_markdown,
                    text_spans,
                    errors,
                    InlineStyleType::Strong as u8,
                    None,
                    Some(get_header_font_size(header.depth)),
                    &None,
                );
            });
            text_spans.push(TextSpan {
                text: "\n".to_string(),
                ..default()
            });
        }
        markdown::mdast::Node::Paragraph(paragraph) => {
            paragraph.children.iter().for_each(|child| match child {
                markdown::mdast::Node::Break(_) => {
                    text_spans.push(TextSpan {
                        text: "\n".to_string(),
                        ..default()
                    });
                }
                markdown::mdast::Node::Text(text) => {
                    text_spans.push(TextSpan {
                        text: text.value.to_string(),
                        ..default()
                    });
                }
                markdown::mdast::Node::Strong(_)
                | markdown::mdast::Node::Emphasis(_)
                | markdown::mdast::Node::InlineCode(_)
                | markdown::mdast::Node::Delete(_)
                | markdown::mdast::Node::Link(_) => {
                    let _ = handle_inline_styling(
                        child,
                        bevy_markdown,
                        text_spans,
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
    text_spans: &mut Vec<TextSpan>,
    errors: &mut Vec<BevyMarkdownError>,
    applied_style: u8,
    force_color: Option<Color>,
    force_size: Option<f32>,
    force_data: &Option<String>,
) -> Result<(), Vec<BevyMarkdownError>> {
    match node {
        markdown::mdast::Node::InlineCode(code) => {
            let mut text_span = TextSpan {
                text: code.value.clone(),
                color: Some(bevy_markdown.markdown_theme.inline_code),
                font_size: force_size,
                ..default()
            };
            if let Some(link) = force_data {
                text_span.metadata = Some(TextSpanMetadata {
                    link: Some(link.clone()),
                })
            }
            text_spans.push(text_span);
        }
        markdown::mdast::Node::Emphasis(emphasis) => emphasis.children.iter().for_each(|child| {
            let _ = handle_inline_styling(
                child,
                bevy_markdown,
                text_spans,
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
                text_spans,
                errors,
                applied_style | InlineStyleType::Strong as u8,
                force_color,
                force_size,
                force_data,
            );
        }),
        markdown::mdast::Node::Text(text) => {
            let mut text_span = TextSpan {
                text: text.value.clone(),
                font_size: force_size,
                ..default()
            };
            if let Some(color) = force_color {
                text_span.color = Some(color)
            }
            if let Some(link) = force_data {
                text_span.metadata = Some(TextSpanMetadata {
                    link: Some(link.clone()),
                })
            }
            match InlineStyleType::from_u8(applied_style) {
                InlineStyleType::Strong => {
                    text_span.weigth = Some(Weight::BOLD);
                }
                InlineStyleType::Emphasis => {
                    text_span.style = Some(cosmic_text::Style::Italic);
                }
                InlineStyleType::StrongEmphasis => {
                    text_span.weigth = Some(Weight::BOLD);
                    text_span.style = Some(cosmic_text::Style::Italic);
                }
                _ => {}
            }
            text_spans.push(text_span);
        }
        markdown::mdast::Node::Link(link) => link.children.iter().for_each(|child| {
            let _ = handle_inline_styling(
                child,
                bevy_markdown,
                text_spans,
                errors,
                applied_style,
                Some(bevy_markdown.markdown_theme.link),
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
    text_spans: &mut Vec<TextSpan>,
    errors: &mut Vec<BevyMarkdownError>,
    indentation_level: u8,
) -> Result<(), Vec<BevyMarkdownError>> {
    text_spans.push(TextSpan {
        text: "\n".to_string(),
        ..default()
    });

    let mut list_index = list.start;
    list.children
        .clone()
        .into_iter()
        .for_each(|node| match node {
            markdown::mdast::Node::ListItem(item) => {
                for _ in 0..indentation_level {
                    text_spans.push(TextSpan {
                        text: "    ".to_string(),
                        ..default()
                    });
                }

                let indent_char = if list.ordered {
                    let index = list_index.unwrap();
                    list_index = Some(index + 1);
                    format!(" {}. ", index)
                } else {
                    get_bullet_for_indentation_level(indentation_level).to_string()
                };

                text_spans.push(TextSpan {
                    text: indent_char,
                    ..default()
                });

                item.children.into_iter().for_each(|child| match child {
                    markdown::mdast::Node::Paragraph(paragraph) => {
                        paragraph.children.iter().for_each(|child| {
                            let _ = handle_inline_styling(
                                child,
                                bevy_markdown,
                                text_spans,
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
                            text_spans,
                            errors,
                            indentation_level + 1,
                        );
                    }
                    node => errors.push(BevyMarkdownError::Transform {
                        info: format!("{:?} node is not implemented for list item", node),
                    }),
                });

                text_spans.push(TextSpan {
                    text: "\n".to_string(),
                    ..default()
                });
            }
            _ => {
                errors.push(BevyMarkdownError::Transform {
                    info: "invalid list children".to_string(),
                });
            }
        });
    Ok(())
}

#[derive(Debug)]
pub struct BevyMarkdownLines {
    pub lines: Vec<Vec<(String, cosmic_text::AttrsOwned)>>,
    pub span_metadata: Vec<TextSpanMetadata>,
}

pub fn generate_markdown_lines(
    bevy_markdown: BevyMarkdown,
) -> Result<BevyMarkdownLines, Vec<BevyMarkdownError>> {
    let node = markdown::to_mdast(bevy_markdown.text.as_str(), &markdown::ParseOptions::gfm());
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    match node {
        Ok(node) => {
            let mut text_spans = Vec::new();
            let mut errors = Vec::new();
            match node {
                markdown::mdast::Node::Root(root) => {
                    root.children.iter().for_each(|child| match child {
                        markdown::mdast::Node::Code(code) => {
                            let default_lang =
                                bevy_markdown.markdown_theme.code_default_lang.clone();
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
                                &ts.themes[&bevy_markdown.markdown_theme.code_theme.clone()],
                            );
                            text_spans.push(TextSpan {
                                text: "\n\n".to_string(),
                                ..default()
                            });
                            for line in LinesWithEndings::from(code.value.as_str()) {
                                let ranges: Vec<(syntect::highlighting::Style, &str)> =
                                    h.highlight_line(line, &ps).unwrap();

                                for &(style, text) in ranges.iter() {
                                    let mut text_span = TextSpan {
                                        text: text.to_string(),
                                        ..default()
                                    };
                                    match style.font_style {
                                        FontStyle::BOLD => text_span.weigth = Some(Weight::BOLD),
                                        FontStyle::ITALIC => {
                                            text_span.style = Some(cosmic_text::Style::Italic)
                                        }
                                        FontStyle::UNDERLINE => {
                                            text_span.weigth = Some(Weight::BOLD);
                                            text_span.style = Some(cosmic_text::Style::Italic);
                                        }
                                        _ => text_span.weigth = Some(Weight::SEMIBOLD),
                                    };
                                    let color = style.foreground;
                                    text_span.color =
                                        Some(cosmic_text::Color::rgb(color.r, color.g, color.b));
                                    text_spans.push(text_span);
                                }
                            }
                            text_spans.push(TextSpan {
                                text: "\n".to_string(),
                                ..default()
                            });
                        }
                        markdown::mdast::Node::Heading(_) | markdown::mdast::Node::Paragraph(_) => {
                            let _ = handle_block_styling(
                                child,
                                &bevy_markdown,
                                &mut text_spans,
                                &mut errors,
                            );
                        }
                        markdown::mdast::Node::List(list) => {
                            let _ = handle_list_recursive(
                                list,
                                &bevy_markdown,
                                &mut text_spans,
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
                let mut spans_meta = vec![];
                let mut lines: Vec<Vec<(String, AttrsOwned)>> = vec![vec![]];

                for (i, span) in text_spans.iter().enumerate() {
                    let mut attrs = bevy_markdown.attrs.as_attrs();
                    // if cosmic-text implements attrs.size add it here
                    if let Some(color) = span.color {
                        attrs = attrs.color(color)
                    }
                    if let Some(style) = span.style {
                        attrs = attrs.style(style)
                    }
                    if let Some(weight) = span.weigth {
                        attrs = attrs.weight(weight)
                    }
                    attrs = attrs.metadata(i);
                    if let Some(metadata) = span.metadata.clone() {
                        spans_meta.push(metadata);
                    } else {
                        spans_meta.push(TextSpanMetadata { link: None });
                    };

                    let mut temp = String::new();

                    for ch in span.text.chars() {
                        if ch == '\n' {
                            if !temp.is_empty() {
                                lines
                                    .last_mut()
                                    .unwrap()
                                    .push((temp.clone(), AttrsOwned::new(attrs)));
                                temp.clear();
                            }
                            lines.push(Vec::new());
                        } else {
                            temp.push(ch);
                        }
                    }

                    if !temp.is_empty() {
                        lines
                            .last_mut()
                            .unwrap()
                            .push((temp, AttrsOwned::new(attrs)));
                    }
                }
                Ok(BevyMarkdownLines {
                    lines,
                    span_metadata: spans_meta,
                })
            }
        }
        Err(e) => Err(vec![BevyMarkdownError::Parsing { info: e }]),
    }
}

#[cfg(test)]
mod tests {
    use cosmic_text::Attrs;

    use crate::*;

    fn test_bevymarkdown(input: String, test_name: String) {
        let markdown_theme = BevyMarkdownTheme {
            code_theme: "Solarized (light)".to_string(),
            code_default_lang: "rs".to_string(),
            link: Color::rgb(10, 10, 10),
            inline_code: Color::rgb(100, 100, 100),
        };

        insta::assert_debug_snapshot!(
            test_name.clone(),
            generate_markdown_lines(BevyMarkdown {
                markdown_theme,
                text: input.clone(),
                attrs: AttrsOwned::new(Attrs::new()),
            })
        );
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
