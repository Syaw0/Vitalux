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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_styles() {
        let paint_type = PaintType::FG;
        let styles = paint_type.make_styles(None);
        assert_eq!(styles, String::new());

        let paint_type = PaintType::BG;
        let styles = paint_type.make_styles(Some(&PaintType::FG));
        assert_eq!(styles, String::new());
    }

    #[test]
    fn test_make_styles_null_input() {
        let paint_type = PaintType::FG;
        let styles = paint_type.make_styles(None);
        assert_eq!(styles, String::new());
    }

    #[test]
    fn test_make_styles_multiple_calls() {
        let paint_type = PaintType::FG;
        let styles1 = paint_type.make_styles(None);
        let styles2 = paint_type.make_styles(None);
        assert_eq!(styles1, styles2);
    }
}
