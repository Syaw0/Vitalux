use crate::colors::{
    PaintType,
    PaletteColor,
    Stylify,
    RGB,
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
    start_styles: Vec<Box<dyn Stylify>>,
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
        // Add formatter reset in the end
        // self.end_styles.push();
        // format!()
        String::new()
    }

    pub fn fg(&mut self) {
        self.start_styles.push(Box::new(PaintType::FG));
    }

    pub fn bg(&mut self) {
        self.start_styles.push(Box::new(PaintType::BG));
    }

    // Colors

    pub fn rgb(&mut self, r: u8, g: u8, b: u8) {
        self.start_styles.push(Box::new(RGB { r, g, b }));
    }

    pub fn palette(&mut self, index: u8) {
        self.start_styles.push(Box::new(PaletteColor { index }));
    }

    pub fn black(&mut self) {
        self.start_styles.push(Box::new(BLACK));
    }

    pub fn red(&mut self) {
        self.start_styles.push(Box::new(RED));
    }

    pub fn green(&mut self) {
        self.start_styles.push(Box::new(GREEN));
    }

    pub fn yellow(&mut self) {
        self.start_styles.push(Box::new(YELLOW));
    }

    pub fn blue(&mut self) {
        self.start_styles.push(Box::new(BLUE));
    }

    pub fn magenta(&mut self) {
        self.start_styles.push(Box::new(MAGENTA));
    }

    pub fn cyan(&mut self) {
        self.start_styles.push(Box::new(CYAN));
    }

    pub fn white(&mut self) {
        self.start_styles.push(Box::new(WHITE));
    }

    pub fn gray(&mut self) {
        self.start_styles.push(Box::new(GRAY));
    }

    pub fn bright_red(&mut self) {
        self.start_styles.push(Box::new(BRIGHT_RED));
    }

    pub fn bright_green(&mut self) {
        self.start_styles.push(Box::new(BRIGHT_GREEN));
    }

    pub fn bright_yellow(&mut self) {
        self.start_styles.push(Box::new(BRIGHT_YELLOW));
    }

    pub fn bright_blue(&mut self) {
        self.start_styles.push(Box::new(BRIGHT_BLUE));
    }

    pub fn bright_magenta(&mut self) {
        self.start_styles.push(Box::new(BRIGHT_MAGENTA));
    }

    pub fn bright_cyan(&mut self) {
        self.start_styles.push(Box::new(BRIGHT_CYAN));
    }

    pub fn bright_white(&mut self) {
        self.start_styles.push(Box::new(BRIGHT_WHITE));
    }
}
