use termion::color::Rgb;

pub struct Color {
    pub text: Rgb,
    pub teal: Rgb,
    pub sky: Rgb,
    pub peach: Rgb,
    pub red: Rgb,
    pub green: Rgb,
}

pub struct ThemeColors {
    pub text: Rgb,
    pub primary: Rgb,
    pub secondary: Rgb,
    pub tertiary: Rgb,
    pub accent: Rgb,
}

pub enum Theme {
    CatppuccinLatte,
    CatppuccinMocha,
    OneDark,
}

impl Theme {
    fn get_colors(&self) -> Color {
        match self {
            Theme::CatppuccinLatte => Color {
                text: hex_to_rgb("#4c4f69"),
                teal: hex_to_rgb("#179299"),
                sky: hex_to_rgb("#04a5e5"),
                peach: hex_to_rgb("#fe640b"),
                red: hex_to_rgb("#d20f39"),
                green: hex_to_rgb("#40a02b"),
            },
            Theme::CatppuccinMocha => Color {
                text: hex_to_rgb("#cdd6f4"),
                teal: hex_to_rgb("#94e2d5"),
                sky: hex_to_rgb("#94e2d5"),
                peach: hex_to_rgb("#fab387"),
                red: hex_to_rgb("#f38ba8"),
                green: hex_to_rgb("#a6e3a1"),
            },
            Theme::OneDark => Color {
                text: hex_to_rgb("#abb2bf"),
                teal: hex_to_rgb("#56b6c2"),
                sky: hex_to_rgb("#61afef"),
                peach: hex_to_rgb("#e5c07b"),
                red: hex_to_rgb("#e06c75"),
                green: hex_to_rgb("#98c379"),
            },
        }
    }

    pub fn get_theme_colors(&self) -> ThemeColors {
        let colors = self.get_colors();
        ThemeColors {
            text: colors.text,
            primary: colors.teal,
            secondary: colors.sky,
            tertiary: colors.green,
            accent: colors.green,
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            Theme::CatppuccinLatte => "Catppuccin Latte",
            Theme::CatppuccinMocha => "Catppuccin Mocha",
            Theme::OneDark => "One Dark",
        }
    }
}

fn hex_to_rgb(hex: &str) -> Rgb {
    let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
    let b = u8::from_str_radix(&hex[5..7], 16).unwrap();

    Rgb(r, g, b)
}
