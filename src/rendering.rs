use crate::{colors::Theme, Presentation};
use std::{
    fmt::Display,
    io::{stdout, Write},
    ops::Add,
    path::Path,
    thread,
    time::Duration,
};
use termion::{
    color::{self, Rgb},
    cursor::{self, DetectCursorPos},
    raw::IntoRawMode,
    style, terminal_size,
};
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
    let color = match block.language.as_str() {
        "rust" => theme.get_theme_colors().primary,
        "java" | "kotlin" => theme.get_theme_colors().secondary,
        "python" => theme.get_theme_colors().tertiary,
        _ => theme.get_theme_colors().accent,
    };

    // Render language identifier
    write!(
        stdout,
        "{}{}{}{}{}{}",
        cursor::Goto(indent, start_line - 1),
        style::Bold,
        color::Fg(color),
        block.language,
        color::Fg(color::Reset),
        style::Reset
    )
    .unwrap();

    // Render code content
    for (idx, line) in block.content.lines().enumerate() {
        write!(
            stdout,
            "{}{}{}{}",
            cursor::Goto(indent, start_line + idx as u16),
            color::Fg(color),
            line,
            color::Fg(color::Reset),
        )
        .unwrap();
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
