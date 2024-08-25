use crate::Metadata;
use std::io::Write;
use termion::{color, terminal_size};

pub fn render_slide(
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
        writeln!(stdout, "{}{}", termion::cursor::Goto(1, i as u16 + 4), line).unwrap();
    }
    stdout.flush().unwrap();
}

pub fn render_title(metadata: &Metadata, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {
    let (width, _) = terminal_size().unwrap();
    let title = metadata.title.as_ref().unwrap();
    let padding = (width as usize - title.len()) / 2;
    let spaces = " ".repeat(padding);
    write!(
        stdout,
        "{}{}{}{}",
        color::Fg(color::Rgb(243, 139, 168)),
        spaces,
        title,
        color::Fg(color::Reset)
    )
    .unwrap();
}
