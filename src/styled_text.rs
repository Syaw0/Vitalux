use crate::colors::{
    PaintType,
    PaletteColor,
    Styles,
    Stylify,
    BLACK,
    BLUE,
    BRIGHT_BLUE,
    BRIGHT_CYAN,
    BRIGHT_GREEN,
    BRIGHT_MAGENTA,
    BRIGHT_RED,
    BRIGHT_WHITE,
    BRIGHT_YELLOW,
    CYAN,
    GRAY,
    GREEN,
    MAGENTA,
    RED,
    Rgb,
    WHITE,
    YELLOW,
};

// =======================================================================

// this is public api for users
/// # TXT
pub fn txt(text: &str) -> StyledText {
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
        let indexes: Vec<usize> = self.start_styles
            .iter()
            .enumerate()
            .filter_map(|(index, style)| {
                match style {
                    Styles::StylePaintType(_) => Some(index),
                    _ => None,
                }
            })
            .collect();

        println!("{:?}", indexes);
        // Add formatter reset in the end
        // self.end_styles.push();
        // format!()
        String::new()
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_paint_type() {
        let txt = txt("Siavash").bright_blue().fg().red().yellow().bg().paint();
        assert_eq!(true, false);
    }
}
