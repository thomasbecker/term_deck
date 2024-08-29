use crate::Presentation;
use std::{fmt::Display, io::Write, ops::Add};
use termion::{
    color,
    cursor::{self, DetectCursorPos},
    style, terminal_size,
};

enum Header {
    Header1,
    Header2,
    Header3,
    Header4,
}

impl Header {
    fn color(&self) -> color::Rgb {
        match self {
            Header::Header1 => color::Rgb(243, 139, 168),
            Header::Header2 => color::Rgb(166, 227, 161),
            Header::Header3 => color::Rgb(148, 226, 213),
            Header::Header4 => color::Rgb(245, 224, 220),
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
    render_text_centered(presentation.metadata.title.as_ref().unwrap(), false, stdout);
    for (i, line) in presentation
        .current_slide()
        .lines()
        .skip_while(|line| line.trim().is_empty())
        .enumerate()
    {
        let (line, color): (&str, Box<dyn Display>) = if line.starts_with("#") {
            let (hash, line) = extract_prefix(line);
            let header = Header::header_by_prefix(&hash).unwrap();
            (line, Box::new(color::Fg(header.color())))
        } else {
            (line, Box::new(color::Fg(color::Reset)))
        };
        writeln!(
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
    );
    render_progress_bar(
        presentation.current_slide,
        presentation.total_slides(),
        stdout,
    );
    stdout.flush().unwrap();
}

fn extract_prefix(s: &str) -> (String, &str) {
    let prefix = s.chars().take_while(|c| *c == '#').collect::<String>();
    let rest = s.trim_start_matches('#').trim_start();
    (prefix, rest)
}

fn render_text_centered(
    text: &str,
    goto_bottom: bool,
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
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
        color::Fg(color::Rgb(243, 139, 168)),
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
) {
    let (width, height) = terminal_size().unwrap();
    let progress_ratio = current_slide.add(1) as f32 / total_slides as f32;
    let progress_length = (progress_ratio * width as f32) as usize;
    write!(
        stdout,
        "{}{}{}{}",
        cursor::Goto(1, height),
        color::Fg(color::Rgb(243, 139, 168)),
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
