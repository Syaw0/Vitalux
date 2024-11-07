/// A module for working with paint types.
///
/// This module provides an enum `PaintType` that represents different types of paint.
/// It also implements the `Stylify` trait for `PaintType`, which allows for generating styles based on the paint type.

// =======================================================================

use super::Stylify;

// =======================================================================

/// An enum representing different types of paint.
///
/// This enum has two variants: `FG` and `BG`, which represent foreground and background paint, respectively.
#[derive(Debug, Clone)]
pub enum PaintType {
    /// Foreground paint.
    FG,
    /// Background paint.
    BG,
}

impl Stylify for PaintType {
    /// Generates styles based on the paint type.
    ///
    /// This method takes an optional `paint_type` parameter, which is ignored in this implementation.
    /// It returns an empty string, indicating that no styles are generated.
    fn make_styles(&self, _paint_type: Option<&PaintType>) -> String {
        String::new()
    }
}
