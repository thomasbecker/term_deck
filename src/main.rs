use std::{
    fs,
    io::{stdin, stdout},
    path::Path,
    process,
};

use colors::Theme;
use regex::Regex;
use termion::{input::TermRead, raw::IntoRawMode};

pub mod colors;
pub mod rendering;

#[derive(Debug)]
pub struct Metadata {
    author: Option<String>,
    title: Option<String>,
    subtitle: Option<String>,
}

pub struct Presentation<'a> {
    current_slide: usize,
    presentation_file: &'a str,
    slides: Vec<&'a str>,
    metadata: Metadata,
    current_theme_index: usize,
    themes: Vec<&'a Theme>,
}

impl Presentation<'_> {
    pub fn new<'a>(
        metadata: Metadata,
        slides: Vec<&'a str>,
        presentation_file: &'a str,
    ) -> Presentation<'a> {
        Presentation {
            current_slide: 0,
            presentation_file,
            slides,
            metadata,
            current_theme_index: 0,
            themes: vec![
                &Theme::CatppuccinLatte,
                &Theme::CatppuccinMocha,
                &Theme::OneDark,
            ],
        }
    }

    pub fn total_slides(&self) -> usize {
        self.slides.len()
    }

    pub fn current_slide(&self) -> &str {
        self.slides[self.current_slide]
    }
    pub fn current_theme(&self) -> &Theme {
        self.themes[self.current_theme_index]
    }

    pub fn cycle_theme(&mut self) {
        self.current_theme_index = (self.current_theme_index + 1) % self.themes.len();
    }

    pub fn move_to_previous_slide(&mut self) {
        self.current_slide = self.current_slide.saturating_sub(1);
    }

    pub fn move_to_next_slide(&mut self) {
        if self.current_slide < self.slides.len() - 1 {
            self.current_slide = self.current_slide.saturating_add(1);
        }
    }
}

#[tokio::main]
async fn main() {
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
                let mut presentation = Presentation::new(metadata, slides, presentation_file);
                let stdin = stdin();
                let mut stdout = stdout().into_raw_mode().unwrap();
                rendering::render_slide(&presentation, &mut stdout);
                for c in stdin.keys() {
                    match c.unwrap() {
                        termion::event::Key::Char('h') => {
                            presentation.move_to_previous_slide();
                        }
                        termion::event::Key::Char('l') => {
                            presentation.move_to_next_slide();
                        }
                        termion::event::Key::Char('t') => {
                            presentation.cycle_theme();
                            rendering::render_slide(&presentation, &mut stdout);
                            rendering::render_notification(
                                presentation.current_theme().get_name(),
                                &mut stdout,
                                presentation.current_theme().get_theme_colors().text,
                            )
                            .await;
                        }
                        termion::event::Key::Char('q') => {
                            break;
                        }
                        _ => {}
                    }
                    rendering::render_slide(&presentation, &mut stdout);
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
