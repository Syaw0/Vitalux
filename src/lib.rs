//! # Vitalux: Rich Api For Colorize Terminal
//!
//!

mod ansi_code;
mod styles;

// =======================================================================
use crate::{
    ansi_code::ANSIEscapeCode,
    styles::{
        basic_color,
        formatter,
        paint_type::PaintType,
        palette::PaletteColor,
        rgb::Rgb,
        Styles,
    },
};

// =======================================================================

/// Creates a styled text string with the given text and styles.
///
/// This function returns a `StyledText` object that can be customized with various styles,
/// colors, and effects. The resulting styled text can be printed or used in other contexts.
///
/// # Examples
///
/// ```
/// use vitalux::styled;
///
/// let txt = styled("Happy Day!")
///     .rgb(204, 182, 122)
///     .italic()
///     .rapid_blink()
///     .bg()
///     .rgb(74, 56, 7)
///     .fg()
///     .paint();
/// println!("{txt}");
/// ```
///
/// # Styles
///
/// The following styles can be applied to the text:
///
/// * `rgb(r, g, b)`: Sets the text color to the given RGB values.
/// * `italic()`: Sets the text to italic.
/// * `rapid_blink()`: Sets the text to rapidly blink.
/// * `bg()`: Sets the background color.
/// * `fg()`: Sets the foreground color.
///
/// # Colors
///
/// The following colors can be used with the `rgb` method:
///
/// * `rgb(r, g, b)`: Sets the color to the given RGB values.
///
/// # Effects
///
/// The following effects can be applied to the text:
///
/// * `rapid_blink()`: Sets the text to rapidly blink.
///
/// # Returns
///
/// A `StyledText` object that can be printed or used in other contexts.
pub fn styled(text: &str) -> StyledText {
    StyledText::new(text.to_string())
}

pub struct StyledText {
    text: String,
    start_styles: Vec<Styles>,
}

impl StyledText {
    fn new(text: String) -> Self {
        // we should use Formatter::reset()
        // let reset_ansi = ANSIEscapeCode::new("0");
        StyledText {
            text,
            start_styles: vec![],
        }
    }

    pub fn paint(&mut self) -> String {
        let mut default_paint_type = PaintType::FG;

        let start_codes_list: Vec<String> = self.start_styles
            .iter()
            .rev()
            .filter_map(|s| {
                if let Styles::StylePaintType(p) = s {
                    default_paint_type = p.clone();
                    return None;
                }
                let t = s.make_styles(Some(&default_paint_type));
                Some(ANSIEscapeCode::new(t.as_str()).code())
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        let start_codes = start_codes_list.join("");
        let end_codes = ANSIEscapeCode::new(
            &formatter::RESET.make_styles(Some(&default_paint_type))
        ).code();

        format!("{}{}{}", start_codes, self.text, end_codes)
    }

    pub fn fg(&mut self) -> &mut Self {
        self.start_styles.push(Styles::StylePaintType(PaintType::FG));
        self
    }

    pub fn bg(&mut self) -> &mut Self {
        self.start_styles.push(Styles::StylePaintType(PaintType::BG));
        self
    }

    // Colors

    pub fn rgb(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.start_styles.push(Styles::StyleRgb(Rgb { r, g, b }));
        self
    }

    pub fn palette(&mut self, index: u8) -> &mut Self {
        self.start_styles.push(Styles::StylePaletteColor(PaletteColor { index }));
        self
    }

    pub fn black(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::BLACK);
        self
    }

    pub fn red(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::RED);
        self
    }

    pub fn green(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::GREEN);
        self
    }

    pub fn yellow(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::YELLOW);
        self
    }

    pub fn blue(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::BLUE);
        self
    }

    pub fn magenta(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::MAGENTA);
        self
    }

    pub fn cyan(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::CYAN);
        self
    }

    pub fn white(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::WHITE);
        self
    }

    pub fn gray(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::GRAY);
        self
    }

    pub fn bright_red(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::BRIGHT_RED);
        self
    }

    pub fn bright_green(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::BRIGHT_GREEN);
        self
    }

    pub fn bright_yellow(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::BRIGHT_YELLOW);
        self
    }

    pub fn bright_blue(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::BRIGHT_BLUE);
        self
    }

    pub fn bright_magenta(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::BRIGHT_MAGENTA);
        self
    }

    pub fn bright_cyan(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::BRIGHT_CYAN);
        self
    }

    pub fn bright_white(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::BRIGHT_WHITE);
        self
    }

    // Formatters

    pub fn reset(&mut self) -> &mut Self {
        self.start_styles.push(formatter::RESET);
        self
    }

    pub fn bold(&mut self) -> &mut Self {
        self.start_styles.push(formatter::BOLD);
        self
    }

    pub fn faint(&mut self) -> &mut Self {
        self.start_styles.push(formatter::FAINT);
        self
    }

    pub fn italic(&mut self) -> &mut Self {
        self.start_styles.push(formatter::ITALIC);
        self
    }

    pub fn underline(&mut self) -> &mut Self {
        self.start_styles.push(formatter::UNDERLINE);
        self
    }

    pub fn slow_blink(&mut self) -> &mut Self {
        self.start_styles.push(formatter::SLOW_BLINK);
        self
    }

    pub fn rapid_blink(&mut self) -> &mut Self {
        self.start_styles.push(formatter::RAPID_BLINK);
        self
    }

    pub fn overline(&mut self) -> &mut Self {
        self.start_styles.push(formatter::OVERLINE);
        self
    }
}
