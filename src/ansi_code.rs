/// A module for creating ansi escape code.
///
/// This module provides an struct `ANSIEscapeCode`
/// that represents ansi escape code, with `parameter` field.
/// It also implements the `ANSIEscapeCode`, with `new` and `code` method
/// which allows for generating anis escape code and getting a code.

// =======================================================================

/// An struct representing ansi escape code.
///
/// This struct has 1 field: `parameter` , which represent String of the ansi code.
pub struct ANSIEscapeCode {
    parameter: String,
}
impl ANSIEscapeCode {
    /// Returns a ANSIEscapeCode instance with parameter .
    pub fn new(parameter: &str) -> Self {
        ANSIEscapeCode { parameter: parameter.to_string() }
    }

    /// Returns a String that represent the ansi code.
    pub fn code(&self) -> String {
        format!("\x1b[{}m", self.parameter)
    }
}

// =======================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_simple_ansi_code() {
        let p = ANSIEscapeCode::new("33");
        assert_eq!(p.code(), "\x1b[33m")
    }

    #[test]
    fn create_reset_ansi_code() {
        let reset_ansi = ANSIEscapeCode::new("0");
        assert_eq!("\x1b[0m", reset_ansi.code());
    }

    #[test]
    fn create_bright_cyan_ansi_code() {
        let reset_ansi = ANSIEscapeCode::new("96");
        assert_eq!("\x1b[96m", reset_ansi.code());
    }
}
