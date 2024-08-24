use std::{
    fs,
    io::{stdin, stdout, Write},
    path::Path,
    process,
};

use regex::Regex;
use termion::{input::TermRead, raw::IntoRawMode, terminal_size};

#[derive(Debug)]
struct Metadata {
    author: Option<String>,
    title: Option<String>,
    subtitle: Option<String>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let presentation_file = &args[1];
        if !Path::new(presentation_file).exists() {
            eprintln!("The file {} does not exist!", presentation_file);
            process::exit(1);
        }
        match fs::read_to_string(presentation_file) {
            Ok(content) => {
                let (metadata, content_without_metadata) = parse_metadata(&content);
                let slides: Vec<&str> = content_without_metadata
                    .split("<!-- end_slide -->")
                    .collect();
                let mut current_slide: usize = 0;
                let stdin = stdin();
                let mut stdout = stdout().into_raw_mode().unwrap();
                render_slide(slides[current_slide], &metadata, &mut stdout);
                for c in stdin.keys() {
                    render_slide(slides[current_slide], &metadata, &mut stdout);
                    match c.unwrap() {
                        termion::event::Key::Char('h') => {
                            current_slide = current_slide.saturating_sub(1)
                        }
                        termion::event::Key::Char('l') => {
                            if current_slide < slides.len() - 1 {
                                current_slide = current_slide.saturating_add(1)
                            }
                        }
                        termion::event::Key::Char('q') => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading file: {}", err);
                process::exit(1);
            }
        }
    } else {
        eprintln!("Please provide a presentation markdown file as an argument!");
        process::exit(1);
    }
}

fn render_slide(
    slide: &str,
    metadata: &Metadata,
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
) {
    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    render_title(metadata, stdout);
    for (i, line) in slide
        .lines()
        .skip_while(|line| line.trim().is_empty())
        .enumerate()
    {
        writeln!(stdout, "{}{}", termion::cursor::Goto(1, i as u16 + 1), line).unwrap();
    }
    stdout.flush().unwrap();
}

fn render_title(metadata: &Metadata, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {
    let (width, _) = terminal_size().unwrap();
    let title = metadata.title.as_ref().unwrap();
    let padding = (width as usize - title.len()) / 2;
    let spaces = " ".repeat(padding);
    write!(stdout, "{}{}\n\n\n", spaces, title).unwrap();
}

fn parse_metadata(content: &str) -> (Metadata, String) {
    let re = Regex::new(r"(author|title|subtitle): (.*?)\n").unwrap();
    let mut metadata = Metadata {
        author: None,
        title: None,
        subtitle: None,
    };

    for cap in re.captures_iter(content) {
        let key = &cap[1];
        let value = cap[2].trim().to_string();
        match key {
            "author" => metadata.author = Some(value),
            "title" => metadata.title = Some(value),
            "subtitle" => metadata.subtitle = Some(value),
            _ => {}
        }
    }
    let content_without_metadata = Regex::new(r"(?s)^---\n.*\n---\n")
        .unwrap()
        .replace(content, "");
    (metadata, content_without_metadata.to_string())
}
