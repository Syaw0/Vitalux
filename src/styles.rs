use basic_color::BasicColor;
use formatter::Formatter;
use paint_type::PaintType;
use palette::PaletteColor;
use rgb::Rgb;

// =======================================================================

pub mod paint_type;
pub mod formatter;
pub mod basic_color;
pub mod rgb;
pub mod palette;

// =======================================================================

#[derive(Debug, Clone)]
pub enum Styles {
    StyleRgb(Rgb),
    StyleBasicColor(BasicColor),
    StylePaletteColor(PaletteColor),
    StylePaintType(PaintType),
    StyleFormatter(Formatter),
}

impl Styles {
    pub fn make_styles(&self, paint_type: Option<&PaintType>) -> String {
        match self {
            Styles::StyleBasicColor(c) => c.make_styles(paint_type),
            Styles::StylePaintType(c) => c.make_styles(paint_type),
            Styles::StylePaletteColor(c) => c.make_styles(paint_type),
            Styles::StyleRgb(c) => c.make_styles(paint_type),
            Styles::StyleFormatter(c) => c.make_styles(paint_type),
        }
    }
}

pub trait Stylify {
    fn make_styles(&self, paint_type: Option<&PaintType>) -> String;
}
