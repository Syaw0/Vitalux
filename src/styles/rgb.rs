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
