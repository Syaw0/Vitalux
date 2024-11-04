use crate::{
    ansi_escape_code::ANSIEscapeCode,
    colors::{
        PaintType,
        PaletteColor,
        Rgb,
        Styles,
        Stylify,
        BLACK,
        BLUE,
        BOLD,
        BRIGHT_BLUE,
        BRIGHT_CYAN,
        BRIGHT_GREEN,
        BRIGHT_MAGENTA,
        BRIGHT_RED,
        BRIGHT_WHITE,
        BRIGHT_YELLOW,
        CYAN,
        FAINT,
        GRAY,
        GREEN,
        ITALIC,
        MAGENTA,
        OVERLINE,
        RAPID_BLINK,
        RED,
        RESET,
        SLOW_BLINK,
        UNDERLINE,
        WHITE,
        YELLOW,
    },
};

// =======================================================================

// this is public api for users
/// # TXT
pub fn styled(text: &str) -> StyledText {
    StyledText::new(text.to_string())
}

pub struct StyledText {
    text: String,
    start_styles: Vec<Styles>,
    end_styles: Vec<Box<dyn Stylify>>,
}

impl StyledText {
    fn new(text: String) -> Self {
        // we should use Formatter::reset()
        // let reset_ansi = ANSIEscapeCode::new("0");
        StyledText {
            text,
            start_styles: vec![],
            end_styles: vec![],
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
        let end_codes = ANSIEscapeCode::new(&RESET.make_styles(Some(&default_paint_type))).code();

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
        self.start_styles.push(BLACK);
        self
    }

    pub fn red(&mut self) -> &mut Self {
        self.start_styles.push(RED);
        self
    }

    pub fn green(&mut self) -> &mut Self {
        self.start_styles.push(GREEN);
        self
    }

    pub fn yellow(&mut self) -> &mut Self {
        self.start_styles.push(YELLOW);
        self
    }

    pub fn blue(&mut self) -> &mut Self {
        self.start_styles.push(BLUE);
        self
    }

    pub fn magenta(&mut self) -> &mut Self {
        self.start_styles.push(MAGENTA);
        self
    }

    pub fn cyan(&mut self) -> &mut Self {
        self.start_styles.push(CYAN);
        self
    }

    pub fn white(&mut self) -> &mut Self {
        self.start_styles.push(WHITE);
        self
    }

    pub fn gray(&mut self) -> &mut Self {
        self.start_styles.push(GRAY);
        self
    }

    pub fn bright_red(&mut self) -> &mut Self {
        self.start_styles.push(BRIGHT_RED);
        self
    }

    pub fn bright_green(&mut self) -> &mut Self {
        self.start_styles.push(BRIGHT_GREEN);
        self
    }

    pub fn bright_yellow(&mut self) -> &mut Self {
        self.start_styles.push(BRIGHT_YELLOW);
        self
    }

    pub fn bright_blue(&mut self) -> &mut Self {
        self.start_styles.push(BRIGHT_BLUE);
        self
    }

    pub fn bright_magenta(&mut self) -> &mut Self {
        self.start_styles.push(BRIGHT_MAGENTA);
        self
    }

    pub fn bright_cyan(&mut self) -> &mut Self {
        self.start_styles.push(BRIGHT_CYAN);
        self
    }

    pub fn bright_white(&mut self) -> &mut Self {
        self.start_styles.push(BRIGHT_WHITE);
        self
    }

    // formatters

    pub fn reset(&mut self) -> &mut Self {
        self.start_styles.push(RESET);
        self
    }

    pub fn bold(&mut self) -> &mut Self {
        self.start_styles.push(BOLD);
        self
    }

    pub fn faint(&mut self) -> &mut Self {
        self.start_styles.push(FAINT);
        self
    }

    pub fn italic(&mut self) -> &mut Self {
        self.start_styles.push(ITALIC);
        self
    }

    pub fn underline(&mut self) -> &mut Self {
        self.start_styles.push(UNDERLINE);
        self
    }

    pub fn slow_blink(&mut self) -> &mut Self {
        self.start_styles.push(SLOW_BLINK);
        self
    }

    pub fn rapid_blink(&mut self) -> &mut Self {
        self.start_styles.push(RAPID_BLINK);
        self
    }

    pub fn overline(&mut self) -> &mut Self {
        self.start_styles.push(OVERLINE);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn white_fg_black_bg() {
        let raw_text = "I'm So Happy";
        let styled_text = styled(raw_text).white().bg().black().fg().paint();
        assert_eq!(styled_text, "\x1b[47m\x1b[30mI'm So Happy\x1b[0m");
    }

    fn blue_fg_bright_cyan_bg() {
        let raw_text = "Silence is power";
        let styled_text = styled(raw_text).blue().fg().bright_cyan().bg().paint();
        assert_eq!(styled_text, "\x1b[34m\x1b[106mSilence is power\x1b[0m");
    }

    #[test]
    fn test_with_one_paint_type() {
        let no: Vec<Styles> = styled("Hello there")
            .bright_blue()
            .red()
            .bg()
            .start_styles.iter()
            .filter(|s| {
                match s {
                    Styles::StylePaintType(_) => true,
                    _ => false,
                }
            })
            .cloned()
            .collect();

        assert_eq!(no.len(), 1)
    }

    #[test]
    fn test_with_zero_paint_type() {
        let no: Vec<Styles> = styled("Hello there")
            .bright_blue()
            .red()
            .start_styles.iter()
            .filter(|s| {
                match s {
                    Styles::StylePaintType(_) => true,
                    _ => false,
                }
            })
            .cloned()
            .collect();

        assert_eq!(no.len(), 0)
    }
}
