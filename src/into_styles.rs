//!
//!

// =======================================================================

use crate::StyledText;

// =======================================================================

/// Converts everything that can convert into [`String`], to an [`StyledText`].
pub trait IntoStyled {
    /// Creates a styled text string with the given text and styles.
    ///
    /// This function returns a `StyledText` object that can be customized with various styles,
    /// colors, and effects. The resulting styled text can be printed or used in other contexts.
    ///
    /// # Examples
    ///
    /// ```
    /// use term_tools::styled;
    ///
    /// let txt = "Happy Day!"
    ///     .styled()
    ///     .rgb(204, 182, 122)
    ///     .italic()
    ///     .rapid_blink()
    ///     .bg()
    ///     .black()
    ///     .fg()
    ///     .paint();
    ///
    /// println!("{txt}");
    /// ```
    ///
    /// # Styles
    ///
    /// The following styles can be applied to the text:
    ///
    /// * `rgb(r, g, b)`: Sets the text color to the given RGB values.
    /// * `italic()`: Sets the text to italic.
    /// * `rapid_blink()`: Sets the text to rapidly blink.
    /// * `bg()`: Sets the background color.
    /// * `fg()`: Sets the foreground color.
    /// * basic colors like : `red`,`black` and etc.
    ///
    /// # Colors
    ///
    /// The following colors can be used with the `rgb` method:
    ///
    /// * `rgb(r, g, b)`: Sets the color to the given RGB values.
    ///
    /// # Effects
    ///
    /// The following effects can be applied to the text:
    ///
    /// * `rapid_blink()`: Sets the text to rapidly blink.
    ///
    /// # Returns
    ///
    /// A `StyledText` object that can be printed or used in other contexts.
    fn styled(self) -> StyledText;
}

impl<T: Into<String>> IntoStyled for T {
    fn styled(self) -> StyledText {
        StyledText::new(self.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_styles() {
        let txt = "Hi Father"
            .styled()
            .rgb(204, 182, 122)
            .italic()
            .rapid_blink()
            .bg()
            .black()
            .fg()
            .paint();

        println!("{txt}");
    }
}
