//! A module for working with styles and colors.
//! This module provides a set of types and traits for representing different styles and colors,
//! as well as a way to generate styles based on a given paint type.

// =======================================================================

use basic_color::BasicColor;
use formatter::Formatter;
use paint_type::PaintType;
use palette::PaletteColor;
use rgb::Rgb;

// =======================================================================

pub mod basic_color;
pub mod formatter;
pub mod paint_type;
pub mod palette;
pub mod rgb;

// =======================================================================

/// An enum representing different styles.
///
/// This enum has five variants, each representing a different type of style:
/// `StyleRgb`, `StyleBasicColor`, `StylePaletteColor`, `StylePaintType`, and `StyleFormatter`.
#[derive(Debug, Clone)]
pub enum Styles {
    /// A style represented by an RGB color.
    Rgb(Rgb),
    /// A style represented by a basic color.
    BasicColor(BasicColor),
    /// A style represented by a palette color.
    PaletteColor(PaletteColor),
    /// A style represented by a paint type.
    PaintType(PaintType),
    /// A style represented by a formatter.
    Formatter(Formatter),
}

impl Styles {
    /// Generates String styles based on the given paint type.
    ///
    /// This method takes an optional `paint_type` parameter, which is used to determine the styles to generate.
    /// It returns a string representing the generated styles.
    pub fn make_styles(&self, paint_type: Option<&PaintType>) -> String {
        match self {
            Styles::BasicColor(c) => c.make_styles(paint_type),
            Styles::PaintType(c) => c.make_styles(paint_type),
            Styles::PaletteColor(c) => c.make_styles(paint_type),
            Styles::Rgb(c) => c.make_styles(paint_type),
            Styles::Formatter(c) => c.make_styles(paint_type),
        }
    }
}

impl Into<Styles> for Rgb {
    fn into(self) -> Styles {
        Styles::Rgb(self)
    }
}

impl Into<Styles> for BasicColor {
    fn into(self) -> Styles {
        Styles::BasicColor(self)
    }
}

impl Into<Styles> for PaletteColor {
    fn into(self) -> Styles {
        Styles::PaletteColor(self)
    }
}

impl Into<Styles> for PaintType {
    fn into(self) -> Styles {
        Styles::PaintType(self)
    }
}

impl Into<Styles> for Formatter {
    fn into(self) -> Styles {
        Styles::Formatter(self)
    }
}

/// A trait for types that can generate styles based on a given paint type.
pub trait Stylify {
    /// Generates String styles based on the given paint type.
    ///
    /// This method takes an optional `paint_type` parameter, which is used to determine the styles to generate.
    /// It returns a string representing the generated styles.
    fn make_styles(&self, paint_type: Option<&PaintType>) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_styles() {
        let style = Styles::Rgb(Rgb { r: 255, g: 0, b: 0 });
        let styles_default = style.make_styles(None);
        let styles_fg = style.make_styles(Some(&PaintType::FG));
        let styles_bg = style.make_styles(Some(&PaintType::BG));
        assert_eq!(styles_default, "38;2;255;0;0");
        assert_eq!(styles_fg, "38;2;255;0;0");
        assert_eq!(styles_bg, "48;2;255;0;0");

        //
        let style = Styles::BasicColor(BasicColor { fg: 34, bg: 44 });
        let styles_default = style.make_styles(None);
        let styles_fg = style.make_styles(Some(&PaintType::FG));
        let styles_bg = style.make_styles(Some(&PaintType::BG));
        assert_eq!(styles_default, "34");
        assert_eq!(styles_fg, "34");
        assert_eq!(styles_bg, "44");

        //
        let style = Styles::PaletteColor(PaletteColor { index: 44 });
        let styles_default = style.make_styles(None);
        let styles_fg = style.make_styles(Some(&PaintType::FG));
        let styles_bg = style.make_styles(Some(&PaintType::BG));
        assert_eq!(styles_default, "38;5;44");
        assert_eq!(styles_fg, "38;5;44");
        assert_eq!(styles_bg, "48;5;44");

        //
        let style = Styles::PaintType(PaintType::BG);
        let styles_default = style.make_styles(None);
        let styles_fg = style.make_styles(Some(&PaintType::FG));
        let styles_bg = style.make_styles(Some(&PaintType::BG));
        assert_eq!(styles_default, "");
        assert_eq!(styles_fg, "");
        assert_eq!(styles_bg, "");

        //
        let style = Styles::Formatter(Formatter { code: 3 });
        let styles_default = style.make_styles(None);
        let styles_fg = style.make_styles(Some(&PaintType::FG));
        let styles_bg = style.make_styles(Some(&PaintType::BG));
        assert_eq!(styles_default, "3");
        assert_eq!(styles_fg, "3");
        assert_eq!(styles_bg, "3");
    }
}
