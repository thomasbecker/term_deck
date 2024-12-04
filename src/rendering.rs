use crate::{Presentation, Theme};
use std::{
    fmt::Display,
    io::{self, stdout, Write},
    ops::Add,
    path::Path,
    process, thread,
    time::Duration,
};
use streaming_iterator::StreamingIterator;
use termion::{
    color::{self, Rgb},
    cursor::{self, DetectCursorPos},
    raw::IntoRawMode,
    style, terminal_size,
};
use tree_sitter::{Language, Parser, Query};
use viuer::{print_from_file, Config};

enum Header {
    Header1,
    Header2,
    Header3,
    Header4,
}

#[derive(Clone)]
struct CodeBlock {
    language: String,
    content: String,
}

#[derive(Debug)]
enum SyntaxKind {
    Keyword,
    Function,
    Type,
    String,
    Number,
    Comment,
    Variable,
    Parameter,
    Operator,
    Default,
}

impl SyntaxKind {
    fn color(&self, theme: &Theme) -> Rgb {
        match self {
            SyntaxKind::Keyword => theme.get_theme_colors().primary,
            SyntaxKind::Function => theme.get_theme_colors().secondary,
            SyntaxKind::Type => theme.get_theme_colors().tertiary,
            SyntaxKind::String => Rgb(158, 206, 106),
            SyntaxKind::Number => Rgb(247, 118, 142),
            SyntaxKind::Comment => Rgb(150, 150, 150),
            SyntaxKind::Variable => theme.get_theme_colors().accent,
            SyntaxKind::Parameter => Rgb(224, 175, 104),
            SyntaxKind::Operator => Rgb(187, 154, 247),
            SyntaxKind::Default => Rgb(255, 255, 255),
        }
    }
}

struct SyntaxToken {
    kind: SyntaxKind,
    start: usize,
    end: usize,
}

fn get_language_config(lang: &str) -> Option<(Language, &'static str)> {
    match lang {
        "rust" => Some((
            tree_sitter_rust::LANGUAGE.into(),
            include_str!("../queries/rust.scm"),
        )),
        "java" => Some((
            tree_sitter_java::LANGUAGE.into(),
            include_str!("../queries/java.scm"),
        )),
        "python" => Some((
            tree_sitter_python::LANGUAGE.into(),
            include_str!("../queries/python.scm"),
        )),
        _ => None,
    }
}

fn parse_syntax(
    content: &str,
    language: &str,
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
) -> Vec<SyntaxToken> {
    let mut tokens = Vec::new();

    if let Some((lang, query_source)) = get_language_config(language) {
        let mut parser = Parser::new();
        parser.set_language(&lang).unwrap();

        let tree = match parser.parse(content, None) {
            Some(tree) => tree,
            None => return Vec::new(),
        };

        let query = match Query::new(&lang, query_source) {
            Ok(query) => query,
            Err(e) => {
                write!(
                    stdout,
                    "Error parsing query for language {}: {:?}",
                    language, e
                )
                .unwrap();
                stdout.flush().unwrap();
                process::exit(1);
            }
        };

        let mut query_cursor = tree_sitter::QueryCursor::new();
        let mut matches = query_cursor.matches(&query, tree.root_node(), content.as_bytes());
        while let Some(match_) = matches.next() {
            for capture in match_.captures {
                let node = capture.node;
                let capture_name = &query.capture_names()[capture.index as usize];

                let kind = match capture_name.to_string().as_str() {
                    "keyword" => SyntaxKind::Keyword,
                    "function" => SyntaxKind::Function,
                    "type" => SyntaxKind::Type,
                    "string" => SyntaxKind::String,
                    "number" => SyntaxKind::Number,
                    "comment" => SyntaxKind::Comment,
                    "variable" => SyntaxKind::Variable,
                    "parameter" => SyntaxKind::Parameter,
                    "operator" => SyntaxKind::Operator,
                    _ => SyntaxKind::Default,
                };

                tokens.push(SyntaxToken {
                    kind,
                    start: node.start_byte(),
                    end: node.end_byte(),
                });
            }
        }
    }

    tokens.sort_by_key(|t| t.start);
    tokens
}

impl CodeBlock {
    fn parse(text: &str) -> Option<Self> {
        let mut lines = text.lines();
        let first_line = lines.next()?;

        if !first_line.starts_with("```") {
            return None;
        }

        let language = first_line.trim_start_matches('`').trim().to_string();
        let content = text
            .lines()
            .skip(1)
            .take_while(|line| !line.starts_with("```"))
            .collect::<Vec<_>>()
            .join("\n");

        Some(CodeBlock { language, content })
    }
}

impl Header {
    fn color(&self, theme: &Theme) -> color::Rgb {
        match self {
            Header::Header1 => theme.get_theme_colors().primary,
            Header::Header2 => theme.get_theme_colors().secondary,
            Header::Header3 => theme.get_theme_colors().tertiary,
            Header::Header4 => theme.get_theme_colors().accent,
        }
    }

    fn header_by_prefix(prefix: &str) -> Option<Header> {
        match prefix {
            "#" => Some(Header::Header1),
            "##" => Some(Header::Header2),
            "###" => Some(Header::Header3),
            "####" => Some(Header::Header4),
            _ => None,
        }
    }
}

pub fn render_slide(
    presentation: &Presentation,
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
) {
    write!(stdout, "{}{}", termion::clear::All, cursor::Goto(1, 1)).unwrap();
    render_text_centered(
        presentation
            .metadata
            .title
            .as_ref()
            .unwrap_or(&String::from("No title found")),
        false,
        stdout,
        presentation.current_theme().get_theme_colors().primary,
    );
    render_text_centered(
        presentation
            .metadata
            .subtitle
            .as_ref()
            .unwrap_or(&String::from("No subtitle found")),
        false,
        stdout,
        presentation.current_theme().get_theme_colors().primary,
    );
    let lines: Vec<&str> = presentation.current_slide().lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if let Some(image_path) = extract_image_path(line) {
            let full_image_path = Path::new(presentation.presentation_file)
                .parent()
                .unwrap()
                .join(image_path);
            render_image(&full_image_path);
        } else if line.starts_with("```") {
            let remaining_lines = lines[i..].join("\n");

            if let Some(code_block) = CodeBlock::parse(&remaining_lines) {
                render_code_block(
                    &code_block,
                    stdout,
                    i as u16 + 4,
                    presentation.current_theme(),
                );
                // Skip the remaining lines of the code block
                i += code_block.content.lines().count() + 2; // +2 for start/end markers
            }
        } else {
            let (line, color): (&str, Box<dyn Display>) = match line.starts_with("#") {
                true => {
                    let (hash, line) = extract_prefix(line);
                    let header = Header::header_by_prefix(&hash).unwrap();
                    (
                        line,
                        Box::new(color::Fg(header.color(presentation.current_theme()))),
                    )
                }
                _ => (line, Box::new(color::Fg(color::Reset))),
            };
            write!(
                stdout,
                "{}{}{}{}{}{}",
                style::Bold,
                cursor::Goto(1, i as u16 + 4),
                color,
                line,
                color::Fg(color::Reset),
                style::Reset
            )
            .unwrap();
            i += 1;
        }
    }
    render_footer(presentation, stdout);
    stdout.flush().unwrap();
}

fn render_footer(
    presentation: &Presentation,
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
) {
    render_text_centered(
        format!(
            "{}/{} slides",
            presentation.current_slide + 1,
            presentation.total_slides()
        )
        .as_str(),
        true,
        stdout,
        presentation.current_theme().get_theme_colors().accent,
    );
    render_progress_bar(
        presentation.current_slide,
        presentation.total_slides(),
        stdout,
        presentation.current_theme().get_theme_colors().accent,
    );
}

fn extract_image_path(line: &str) -> Option<&str> {
    if line.starts_with("![") && line.contains("](") && line.ends_with(")") {
        let start = line.find("](").unwrap() + 2;
        let end = line.len() - 1;
        Some(&line[start..end])
    } else {
        None
    }
}

fn render_code_block(
    block: &CodeBlock,
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
    start_line: u16,
    theme: &Theme,
) {
    let indent = 4;

    // Render language identifier
    write!(
        stdout,
        "{}{}{}{}{}{}",
        cursor::Goto(indent, start_line - 1),
        style::Bold,
        color::Fg(theme.get_theme_colors().primary),
        block.language,
        color::Fg(color::Reset),
        style::Reset
    )
    .unwrap();

    let tokens = parse_syntax(&block.content, &block.language, stdout);

    // Track current position in the content
    let mut current_pos = 0;
    let mut current_line = 0;

    for line in block.content.lines() {
        let line_start = current_pos;
        let line_end = line_start + line.len();

        // Get tokens for this line
        let line_tokens: Vec<_> = tokens
            .iter()
            .filter(|t| t.start >= line_start && t.start < line_end)
            .collect();

        let mut line_pos = 0;

        // Write the line with syntax highlighting
        write!(
            stdout,
            "{}",
            cursor::Goto(indent, start_line + current_line as u16),
        )
        .unwrap();

        if line_tokens.is_empty() {
            // No syntax highlighting for this line
            write!(stdout, "{}", line).unwrap();
        } else {
            // Apply syntax highlighting
            for token in line_tokens {
                // Write any text before the token
                let token_start_in_line = token.start - line_start;
                if token_start_in_line > line_pos {
                    write!(stdout, "{}", &line[line_pos..token_start_in_line]).unwrap();
                }

                // Write the token with its color
                let token_end_in_line = std::cmp::min(token.end - line_start, line.len());
                write!(
                    stdout,
                    "{}{}{}",
                    color::Fg(token.kind.color(theme)),
                    &line[token_start_in_line..token_end_in_line],
                    color::Fg(color::Reset)
                )
                .unwrap();

                line_pos = token_end_in_line;
            }

            // Write any remaining text
            if line_pos < line.len() {
                write!(stdout, "{}", &line[line_pos..]).unwrap();
            }
        }

        current_pos += line.len() + 1; // +1 for newline
        current_line += 1;
    }
}

fn render_image(image_path: &Path) {
    if !image_path.exists() {
        eprintln!("Error: File does not exist - {:?}", image_path);
        std::io::stderr().flush().unwrap(); // Ensure the error message is flushed
        std::process::exit(1);
    }

    let config = Config {
        ..Default::default()
    };
    print_from_file(image_path, &config).unwrap();
}

fn extract_prefix(s: &str) -> (String, &str) {
    let prefix = s.chars().take_while(|c| *c == '#').collect::<String>();
    let rest = s.trim_start_matches('#').trim_start();
    (prefix, rest)
}

pub async fn render_notification(
    text: &str,
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
    color: Rgb,
) {
    let (width, _) = terminal_size().unwrap();
    let start = width - text.len() as u16;
    write!(
        stdout,
        "{}{}{}{}{}",
        cursor::Goto(start, 1),
        color::Fg(color),
        text,
        color::Fg(color::Reset),
        cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();
    tokio::spawn(async move {
        clear_notification(start, 3).await;
    });
}

pub async fn clear_notification(start: u16, delay_seconds: i8) {
    thread::sleep(Duration::from_secs(delay_seconds as u64));
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}{}",
        cursor::Goto(start, 1),
        termion::clear::UntilNewline,
        cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();
}

fn render_text_centered(
    text: &str,
    goto_bottom: bool,
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
    color: Rgb,
) {
    let (width, height) = terminal_size().unwrap();
    let padding = (width as usize - text.len()) / 2;
    let spaces = " ".repeat(padding);
    let (_, y) = stdout.cursor_pos().unwrap();
    let y_position = if goto_bottom { height - 1 } else { y };
    write!(
        stdout,
        "{}{}{}{}{}{}{}{}",
        cursor::Goto(1, y_position),
        style::Bold,
        color::Fg(color),
        spaces,
        text,
        color::Fg(color::Reset),
        style::Reset,
        cursor::Goto(1, y_position + 1)
    )
    .unwrap();
}

fn render_progress_bar(
    current_slide: usize,
    total_slides: usize,
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
    color: Rgb,
) {
    let (width, height) = terminal_size().unwrap();
    let progress_ratio = current_slide.add(1) as f32 / total_slides as f32;
    let progress_length = (progress_ratio * width as f32) as usize;
    write!(
        stdout,
        "{}{}{}{}",
        cursor::Goto(1, height),
        color::Fg(color),
        "".repeat(progress_length),
        color::Fg(color::Reset)
    )
    .unwrap();

    write!(
        stdout,
        "{}{}",
        " ".repeat(width as usize - progress_length),
        cursor::Goto(1, height + 1)
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_hash_no_hash() {
        let (prefix, rest) = extract_prefix("Hello, world!");
        assert_eq!(prefix, "");
        assert_eq!(rest, "Hello, world!");
    }

    #[test]
    fn test_extract_hash_one_hash() {
        let (prefix, rest) = extract_prefix("#Hello, world!");
        assert_eq!(prefix, "#");
        assert_eq!(rest, "Hello, world!");
    }

    #[test]
    fn test_extract_hash_multiple_hashes() {
        let (prefix, rest) = extract_prefix("###Hello, world!");
        assert_eq!(prefix, "###");
        assert_eq!(rest, "Hello, world!");
    }

    #[test]
    fn test_remove_leading_whitespaces_from_rest() {
        let (prefix, rest) = extract_prefix("###  Hello, world!");
        assert_eq!(prefix, "###");
        assert_eq!(rest, "Hello, world!");
    }
}
