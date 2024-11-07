/// A module for working with styles and colors.
///
/// This module provides a set of types and traits for representing
///  different styles and colors,
/// as well as a way to generate styles based on a given paint type.

// =======================================================================

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

/// An enum representing different styles.
///
/// This enum has five variants, each representing a different type of style:
/// `StyleRgb`, `StyleBasicColor`, `StylePaletteColor`, `StylePaintType`, and `StyleFormatter`.
///
/// # Examples
///
/// ```
/// let style = Styles::StyleRgb(Rgb::new(255, 0, 0));
/// let paint_type = PaintType::FG;
/// let styles = style.make_styles(Some(&paint_type));
/// ```
#[derive(Debug, Clone)]
pub enum Styles {
    /// A style represented by an RGB color.
    StyleRgb(Rgb),
    /// A style represented by a basic color.
    StyleBasicColor(BasicColor),
    /// A style represented by a palette color.
    StylePaletteColor(PaletteColor),
    /// A style represented by a paint type.
    StylePaintType(PaintType),
    /// A style represented by a formatter.
    StyleFormatter(Formatter),
}

impl Styles {
    /// Generates String styles based on the given paint type.
    ///
    /// This method takes an optional `paint_type` parameter, which is used to determine the styles to generate.
    /// It returns a string representing the generated styles.
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

/// A trait for types that can generate styles based on a given paint type.
pub trait Stylify {
    /// Generates String styles based on the given paint type.
    ///
    /// This method takes an optional `paint_type` parameter, which is used to determine the styles to generate.
    /// It returns a string representing the generated styles.
    fn make_styles(&self, paint_type: Option<&PaintType>) -> String;
}
