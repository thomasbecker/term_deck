use crate::{colors::Theme, Presentation};
use std::{
    fmt::Display,
    io::{stdout, Write},
    ops::Add,
    thread,
    time::Duration,
};
use termion::{
    color::{self, Rgb},
    cursor::{self, DetectCursorPos},
    raw::IntoRawMode,
    style, terminal_size,
};

enum Header {
    Header1,
    Header2,
    Header3,
    Header4,
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
    for (i, line) in presentation
        .current_slide()
        .lines()
        .skip_while(|line| line.trim().is_empty())
        .enumerate()
    {
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
    }
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
    stdout.flush().unwrap();
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
        "{}{}{}{}{}{}{}",
        cursor::Goto(1, y_position),
        style::Bold,
        color::Fg(color),
        spaces,
        text,
        color::Fg(color::Reset),
        style::Reset
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
