use super::{ paint_type::PaintType, Styles, Stylify };

// =======================================================================

#[derive(Debug, Clone)]
pub struct Formatter {
    code: u8,
}

impl Stylify for Formatter {
    fn make_styles(&self, _paint_type: Option<&PaintType>) -> String {
        format!("{}", self.code)
    }
}

macro_rules! formatter_code {
    ($name:ident, $code:expr) => {
        pub const $name: Styles = Styles::StyleFormatter(Formatter { code: $code });
    };
}

formatter_code!(RESET, 0);
formatter_code!(BOLD, 1);
formatter_code!(FAINT, 2);
formatter_code!(ITALIC, 3);
formatter_code!(UNDERLINE, 4);
formatter_code!(SLOW_BLINK, 5);
formatter_code!(RAPID_BLINK, 6);
formatter_code!(OVERLINE, 53);
