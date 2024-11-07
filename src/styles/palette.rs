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
