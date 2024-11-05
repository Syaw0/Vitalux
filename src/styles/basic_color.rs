use super::{ paint_type::PaintType, Styles, Stylify };

// =======================================================================

#[derive(Debug, Clone)]
pub struct BasicColor {
    fg: u8,
    bg: u8,
}

impl Stylify for BasicColor {
    /// If the `is_foreground` was None it's assume as foreground
    fn make_styles(&self, paint_type: Option<&PaintType>) -> String {
        let paint_type = paint_type.unwrap_or(&PaintType::FG);
        format!("{}", match paint_type {
            PaintType::FG => self.fg,
            PaintType::BG => self.bg,
        })
    }
}

pub const BLACK: Styles = Styles::StyleBasicColor(BasicColor { fg: 30, bg: 40 });
pub const RED: Styles = Styles::StyleBasicColor(BasicColor { fg: 31, bg: 41 });
pub const GREEN: Styles = Styles::StyleBasicColor(BasicColor { fg: 32, bg: 42 });
pub const YELLOW: Styles = Styles::StyleBasicColor(BasicColor { fg: 33, bg: 43 });
pub const BLUE: Styles = Styles::StyleBasicColor(BasicColor { fg: 34, bg: 44 });
pub const MAGENTA: Styles = Styles::StyleBasicColor(BasicColor { fg: 35, bg: 45 });
pub const CYAN: Styles = Styles::StyleBasicColor(BasicColor { fg: 36, bg: 46 });
pub const WHITE: Styles = Styles::StyleBasicColor(BasicColor { fg: 37, bg: 47 });
pub const GRAY: Styles = Styles::StyleBasicColor(BasicColor { fg: 90, bg: 10 });
pub const BRIGHT_RED: Styles = Styles::StyleBasicColor(BasicColor { fg: 91, bg: 101 });
pub const BRIGHT_GREEN: Styles = Styles::StyleBasicColor(BasicColor { fg: 92, bg: 102 });
pub const BRIGHT_YELLOW: Styles = Styles::StyleBasicColor(BasicColor { fg: 93, bg: 103 });
pub const BRIGHT_BLUE: Styles = Styles::StyleBasicColor(BasicColor { fg: 94, bg: 104 });
pub const BRIGHT_MAGENTA: Styles = Styles::StyleBasicColor(BasicColor { fg: 95, bg: 105 });
pub const BRIGHT_CYAN: Styles = Styles::StyleBasicColor(BasicColor { fg: 96, bg: 106 });
pub const BRIGHT_WHITE: Styles = Styles::StyleBasicColor(BasicColor { fg: 97, bg: 107 });
