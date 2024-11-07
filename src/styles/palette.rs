/// A module for creating palette color.
///
/// This module provides an struct `PaletteColor`
/// that represents palette color, index should be between 0 to 255 mean u8.
/// It also implements the `Stylify` trait for `PaletteColor`,
/// which allows for generating styles based on the paint type.

// =======================================================================

use super::{ paint_type::PaintType, Stylify };

// =======================================================================

/// An struct representing index of palette color.
///
/// This struct has 1 field: `index`, which represent 8 bit color code between 0 to 255.
#[derive(Debug, Clone)]
pub struct PaletteColor {
    pub index: u8,
}

impl Stylify for PaletteColor {
    /// Returns a string representation of the palette index code.
    ///
    /// If `paint_type` is `None`, the foreground color is assumed.
    fn make_styles(&self, paint_type: Option<&PaintType>) -> String {
        let paint_type = paint_type.unwrap_or(&PaintType::FG);
        format!(
            "{};5;{}",
            match paint_type {
                PaintType::FG => "38",
                PaintType::BG => "48",
            },
            self.index
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_style_fg() {
        let color = PaletteColor { index: 42 };
        let styles = color.make_styles(Some(&PaintType::FG));
        assert_eq!(styles, "38;5;42")
    }

    #[test]
    fn test_make_style_default_fg() {
        let color = PaletteColor { index: 105 };
        let styles = color.make_styles(None);
        assert_eq!(styles, "38;5;105")
    }

    #[test]
    fn test_make_style_bg() {
        let color = PaletteColor { index: 1 };
        let styles = color.make_styles(Some(&PaintType::BG));
        assert_eq!(styles, "48;5;1")
    }

    #[test]
    fn test_fg_and_bg_values() {
        let color = PaletteColor { index: 200 };
        let styles_fg = color.make_styles(Some(&PaintType::FG));
        let styles_bg = color.make_styles(Some(&PaintType::BG));
        assert_eq!(styles_fg, "38;5;200");
        assert_eq!(styles_bg, "48;5;200")
    }

    #[test]
    fn test_fg_and_bg_values_with_none_paint_type() {
        let color = PaletteColor { index: 108 };
        let styles_fg = color.make_styles(None);
        let styles_bg = color.make_styles(Some(&PaintType::BG));
        assert_eq!(styles_fg, "38;5;108");
        assert_eq!(styles_bg, "48;5;108")
    }
}
