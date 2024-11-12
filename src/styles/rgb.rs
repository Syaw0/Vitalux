/// A module for creating rgb color.
///
/// This module provides an struct `Rgb`
/// that represents rgb color, with r,g,b field.
/// It also implements the `Stylify` trait for `Rgb`,
/// which allows for generating styles based on the paint type.

// =======================================================================

use super::{ paint_type::PaintType, Stylify };

// =======================================================================

/// An struct representing rgb color.
///
/// This struct has 3 field: `r`, `g` and `b` , which each of them represent 8 bit color code between 0 to 255.
#[derive(Debug, Clone)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Stylify for Rgb {
    /// Returns a string representation of the rgb color code.
    ///
    /// If `paint_type` is `None`, the foreground color is assumed.
    fn make_styles(&self, paint_type: Option<&PaintType>) -> String {
        let paint_type = paint_type.unwrap_or(&PaintType::FG);
        format!(
            "{};2;{};{};{}",
            match paint_type {
                PaintType::FG => "38",
                PaintType::BG => "48",
            },
            self.r,
            self.g,
            self.b
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_style_fg() {
        let color = Rgb { r: 102, g: 23, b: 240 };
        let styles = color.make_styles(Some(&PaintType::FG));
        assert_eq!(styles, "38;2;102;23;240")
    }

    #[test]
    fn test_make_style_default_fg() {
        let color = Rgb { r: 2, g: 55, b: 100 };
        let styles = color.make_styles(None);
        assert_eq!(styles, "38;2;2;55;100")
    }

    #[test]
    fn test_make_style_bg() {
        let color = Rgb { r: 255, g: 255, b: 43 };
        let styles = color.make_styles(Some(&PaintType::BG));
        assert_eq!(styles, "48;2;255;255;43")
    }

    #[test]
    fn test_fg_and_bg_values() {
        let color = Rgb { r: 78, g: 32, b: 210 };
        let styles_fg = color.make_styles(Some(&PaintType::FG));
        let styles_bg = color.make_styles(Some(&PaintType::BG));
        assert_eq!(styles_fg, "38;2;78;32;210");
        assert_eq!(styles_bg, "48;2;78;32;210")
    }

    #[test]
    fn test_fg_and_bg_values_with_none_paint_type() {
        let color = Rgb { r: 1, g: 91, b: 58 };
        let styles_fg = color.make_styles(None);
        let styles_bg = color.make_styles(Some(&PaintType::BG));
        assert_eq!(styles_fg, "38;2;1;91;58");
        assert_eq!(styles_bg, "48;2;1;91;58")
    }
}
