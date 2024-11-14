//! # Term_Tools: Rich API for Colorizing Terminal
//!
//! term_tools is a Rust library that provides a rich API for colorizing terminal output.
//! It allows you to create styled text strings with various colors, effects, and formatters.

mod ansi_code;
mod into_styles;
mod styles;

// =======================================================================

use into_styles::IntoStyled;

use crate::{
    ansi_code::ANSIEscapeCode,
    styles::{
        basic_color, formatter, paint_type::PaintType, palette::PaletteColor, rgb::Rgb, Styles,
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
/// use term_tools::styled;
///
/// let txt = styled("Happy Day!")
///     .rgb(204, 182, 122)
///     .italic()
///     .rapid_blink()
///     .bg()
///     .black()
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
/// * basic colors like : `red`,`black` and etc.
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
pub fn styled<S: IntoStyled>(text: S) -> StyledText {
    text.styled()
}

/// A struct representing a styled text string.
pub struct StyledText {
    text: String,
    start_styles: Vec<Styles>,
}

impl StyledText {
    /// Creates a new `StyledText` object with the given text.
    fn new(text: String) -> Self {
        StyledText {
            text,
            start_styles: vec![],
        }
    }

    /// Paints the styled text string with the given styles.
    ///
    /// This method returns a string representing the styled text.
    pub fn paint(&mut self) -> String {
        let mut default_paint_type = PaintType::FG;

        let start_codes_list: Vec<String> = self
            .start_styles
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
        let end_codes =
            ANSIEscapeCode::new(&formatter::RESET.make_styles(Some(&default_paint_type))).code();

        format!("{}{}{}", start_codes, self.text, end_codes)
    }

    /// Sets the foreground color of the colors you have called.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("How you doing?").black().fg().red().bg().paint();
    /// ```
    /// the colors before this method will paint as foreground!
    /// so the black color will paint as foreground and red color paint
    /// as background color
    /// **if the one `fg` call, all the colors will paint as foreground no matter there is before or after `fg`**
    pub fn fg(&mut self) -> &mut Self {
        self.start_styles
            .push(Styles::StylePaintType(PaintType::FG));
        self
    }

    /// Sets the background color of the colors you have called.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("I'm feel Happy").bright_cyan().bg().blue().fg().paint();
    /// ```
    /// the colors before this method will paint as background!
    /// so the bright_cyan color will paint as background and blue color paint
    /// as foreground color
    /// **if the one `bg` call, all the colors will paint as background no matter there is before or after `bg`**
    pub fn bg(&mut self) -> &mut Self {
        self.start_styles
            .push(Styles::StylePaintType(PaintType::BG));
        self
    }

    // Colors

    /// Sets the `rgb` color to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("Our life is what our thoughts make it.").rgb(48,118,230).paint();
    /// ```
    pub fn rgb(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.start_styles.push(Styles::StyleRgb(Rgb { r, g, b }));
        self
    }

    /// Sets the `palette` color to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("If it is not right, do not do it. If it is not true, do not say it.").palette(132).paint();
    /// ```
    ///
    /// the index should be 8 bit color between 0 to 255.
    pub fn palette(&mut self, index: u8) -> &mut Self {
        self.start_styles
            .push(Styles::StylePaletteColor(PaletteColor { index }));
        self
    }

    /// Sets the `black` color to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The best revenge is to not be like your enemies.").black().paint();
    /// ```
    pub fn black(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::BLACK);
        self
    }

    /// Sets the `red` color to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("To love only what happens, what was destined. No greater harmony.").red().paint();
    /// ```
    pub fn red(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::RED);
        self
    }

    /// Sets the `green` color to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("Everything we hear is opinion, not a fact. Everything we see is a perspective, not the truth.").green().paint();
    /// ```
    pub fn green(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::GREEN);
        self
    }

    /// Sets the `yellow` color to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The present is all we have to live in . . . or to lose.").yellow().paint();
    /// ```
    pub fn yellow(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::YELLOW);
        self
    }

    /// Sets the `blue` color to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The present is all we have to live in . . . or to lose.").blue().paint();
    /// ```
    pub fn blue(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::BLUE);
        self
    }

    /// Sets the `magenta` color to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The present is all we have to live in . . . or to lose.").magenta().paint();
    /// ```
    pub fn magenta(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::MAGENTA);
        self
    }

    /// Sets the `cyan` color to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The present is all we have to live in . . . or to lose.").cyan().paint();
    /// ```
    pub fn cyan(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::CYAN);
        self
    }

    /// Sets the `white` color to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The present is all we have to live in . . . or to lose.").white().paint();
    /// ```
    pub fn white(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::WHITE);
        self
    }

    /// Sets the `gray` color to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The present is all we have to live in . . . or to lose.").gray().paint();
    /// ```
    pub fn gray(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::GRAY);
        self
    }

    /// Sets the `bright_red` color to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The present is all we have to live in . . . or to lose.").bright_red().paint();
    /// ```
    pub fn bright_red(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::BRIGHT_RED);
        self
    }

    /// Sets the `bright_green` color to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The present is all we have to live in . . . or to lose.").bright_green().paint();
    /// ```
    pub fn bright_green(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::BRIGHT_GREEN);
        self
    }

    /// Sets the `bright_yellow` color to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The present is all we have to live in . . . or to lose.").bright_yellow().paint();
    /// ```
    pub fn bright_yellow(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::BRIGHT_YELLOW);
        self
    }

    /// Sets the `bright_blue` color to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The present is all we have to live in . . . or to lose.").bright_blue().paint();
    /// ```
    pub fn bright_blue(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::BRIGHT_BLUE);
        self
    }

    /// Sets the `bright_magenta` color to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The present is all we have to live in . . . or to lose.").bright_magenta().paint();
    /// ```
    /// use term_tools::styled;
    pub fn bright_magenta(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::BRIGHT_MAGENTA);
        self
    }

    /// Sets the `bright_cyan` color to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The present is all we have to live in . . . or to lose.").bright_cyan().paint();
    /// ```
    pub fn bright_cyan(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::BRIGHT_CYAN);
        self
    }

    /// Sets the `bright_white` color to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The present is all we have to live in . . . or to lose.").bright_white().paint();
    /// ```
    pub fn bright_white(&mut self) -> &mut Self {
        self.start_styles.push(basic_color::BRIGHT_WHITE);
        self
    }

    // Formatters

    /// Sets the `reset` effect to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The present is all we have to live in . . . or to lose.").red().reset().paint();
    /// ```
    /// ** this will reset all the effects, colors and formatters that are called before this**
    /// so in the top example the red color will never applied to the input text
    pub fn reset(&mut self) -> &mut Self {
        self.start_styles.push(formatter::RESET);
        self
    }

    /// Sets the `bold` format to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The present is all we have to live in . . . or to lose.").bold().paint();
    /// ```
    pub fn bold(&mut self) -> &mut Self {
        self.start_styles.push(formatter::BOLD);
        self
    }

    /// Sets the `faint` format to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The present is all we have to live in . . . or to lose.").faint().paint();
    /// ```
    pub fn faint(&mut self) -> &mut Self {
        self.start_styles.push(formatter::FAINT);
        self
    }

    /// Sets the `italic` format to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The present is all we have to live in . . . or to lose.").italic().paint();
    /// ```
    pub fn italic(&mut self) -> &mut Self {
        self.start_styles.push(formatter::ITALIC);
        self
    }

    /// Sets the `underline` format to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The present is all we have to live in . . . or to lose.").underline().paint();
    /// ```
    pub fn underline(&mut self) -> &mut Self {
        self.start_styles.push(formatter::UNDERLINE);
        self
    }

    /// Sets the `slow_blink` effect to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The present is all we have to live in . . . or to lose.").slow_blink().paint();
    /// ```
    ///
    /// **base on the terminal you are using this could not be applied**
    pub fn slow_blink(&mut self) -> &mut Self {
        self.start_styles.push(formatter::SLOW_BLINK);
        self
    }

    /// Sets the `rapid_blink` effect to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The present is all we have to live in . . . or to lose.").rapid_blink().paint();
    /// ```
    ///
    /// **base on the terminal you are using this could not be applied**
    pub fn rapid_blink(&mut self) -> &mut Self {
        self.start_styles.push(formatter::RAPID_BLINK);
        self
    }

    /// Sets the `overline` effect to the input text.
    ///
    /// # Example:
    /// ```
    /// use term_tools::styled;
    /// let styled_text = styled("The present is all we have to live in . . . or to lose.").overline().paint();
    /// ```
    pub fn overline(&mut self) -> &mut Self {
        self.start_styles.push(formatter::OVERLINE);
        self
    }
}
