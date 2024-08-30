use termion::color::Rgb;

pub struct ColorTheme {
    pub text: Rgb,
    pub teal: Rgb,
    pub peach: Rgb,
    pub red: Rgb,
    pub green: Rgb,
}

pub enum Theme {
    CatppuccinLatte,
    CatppuccinMocha,
}

impl Theme {
    pub fn get_colors(&self) -> ColorTheme {
        match self {
            Theme::CatppuccinLatte => ColorTheme {
                text: hex_to_rgb("#4c4f69"),
                teal: hex_to_rgb("#179299"),
                peach: hex_to_rgb("#fe640b"),
                red: hex_to_rgb("#d20f39"),
                green: hex_to_rgb("#40a02b"),
            },
            Theme::CatppuccinMocha => ColorTheme {
                text: hex_to_rgb("#cdd6f4"),
                teal: hex_to_rgb("#94e2d5"),
                peach: hex_to_rgb("#fab387"),
                red: hex_to_rgb("#f38ba8"),
                green: hex_to_rgb("#a6e3a1"),
            },
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            Theme::CatppuccinLatte => "Catppuccino Latte",
            Theme::CatppuccinMocha => "Catppuccino Mocha",
        }
    }
}

fn hex_to_rgb(hex: &str) -> Rgb {
    let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
    let b = u8::from_str_radix(&hex[5..7], 16).unwrap();

    Rgb(r, g, b)
}
