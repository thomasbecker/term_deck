use std::{
    fs,
    io::{stdin, stdout, Write},
    path::Path,
    process,
};

use termion::{input::TermRead, raw::IntoRawMode};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let presentation_file = &args[1];
        if !Path::new(presentation_file).exists() {
            eprintln!("The file {} does not exist!", presentation_file);
            process::exit(1);
        }
        match fs::read_to_string(presentation_file) {
            Ok(contents) => {
                let slides: Vec<&str> = contents.split("<!-- end_slide -->").collect();
                let mut current_slide = 0;
                let stdout = stdout();
                let mut stdout = stdout.lock().into_raw_mode().unwrap();
                let stdin = stdin();
                for c in stdin.keys() {
                    write!(
                        stdout,
                        "{}{}",
                        termion::clear::All,
                        termion::cursor::Goto(1, 1)
                    )
                    .unwrap();
                    for (i, line) in slides[current_slide].lines().enumerate() {
                        writeln!(stdout, "{}{}", termion::cursor::Goto(1, i as u16 + 1), line)
                            .unwrap();
                    }
                    stdout.flush().unwrap();
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
