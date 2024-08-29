# Term Deck

Term Deck is a simple terminal-based presentation tool written in Rust. It's
written in rust...by the way. So it's blazingly fast. It's memory safe. It has
borrower nightmares...bullshit borrow checker it's called, not nightmares. If it
compiles, it works. It has zero cost abstractions. It's based on web
assembly...no it's not. It ...

Ok, kidding aside...it allows you to create and navigate through slides directly
in your terminal. This project was developed primarily as a learning exercise in
Rust, and while it is functional, there are more feature-rich alternatives
available.

I want to see what all the fuss is about with Rust and how it compares to good
old C.

## Features

- **Markdown Support**: Term Deck uses a Markdown subset for slide creation
  making it easy to write and format your slides. In fact only headers are
  supported.
- **Navigation**: Navigate through your slides using simple keyboard commands.
- **Metadata**: Each presentation can include metadata such as author, title,
  and subtitle.
- **Terminal-Based**: No need for a GUI - Term Deck runs directly in your terminal.

## Usage

To start a presentation, simply pass the path to your Markdown file as an
argument when running Term Deck:

```bash
cargo run /path/to/your/presentation.md
```

Once the presentation is running, you can navigate through your slides using the
'h' and 'l' keys. To quit the presentation, press 'q'.

### Metadata

To add metadata to your presentation, include the following block at the top of
your presentation:

```bash
---
title: My first presentation
author: Thomas Becker
subtitle: A simple presentation
---
```

### Demo slides

```bash
[examples/demo.md](examples/demo.md)
```

## Learning Rust

This project was developed primarily as a learning exercise in Rust. While it is
a functional tool, there are more feature-rich alternatives available for
terminal-based presentations, such as:

- [slides](https://github.com/maaslalani/slides)
- [presenterm](https://github.com/mfontanini/presenterm/tree/master)
- [Patat](https://github.com/jaspervdj/patat)
- [mdp](https://github.com/visit1985/mdp)

If you're looking for a robust, feature-rich presentation tool, you may want to
consider these alternatives.

However, if you're interested in seeing a simple project built in Rust, or if
you're learning Rust yourself, feel free to explore the codebase!

## Contributing

As this project is primarily for learning, contributions are not actively
sought. However, if you spot a bug or see a way to improve the code, feel free
to open an issue or submit a pull request.
