use super::{ paint_type::PaintType, Styles, Stylify };

// =======================================================================

#[derive(Debug, Clone)]
pub struct Formatter {
    n: u8,
}

impl Stylify for Formatter {
    fn make_styles(&self, _paint_type: Option<&PaintType>) -> String {
        format!("{}", self.n)
    }
}

pub const RESET: Styles = Styles::StyleFormatter(Formatter { n: 0 });
pub const BOLD: Styles = Styles::StyleFormatter(Formatter { n: 1 });
pub const FAINT: Styles = Styles::StyleFormatter(Formatter { n: 2 });
pub const ITALIC: Styles = Styles::StyleFormatter(Formatter { n: 3 });
pub const UNDERLINE: Styles = Styles::StyleFormatter(Formatter { n: 4 });
pub const SLOW_BLINK: Styles = Styles::StyleFormatter(Formatter { n: 5 });
pub const RAPID_BLINK: Styles = Styles::StyleFormatter(Formatter { n: 6 });
pub const OVERLINE: Styles = Styles::StyleFormatter(Formatter { n: 53 });
