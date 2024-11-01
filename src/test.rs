#[cfg(test)]
mod ansi_escape_code_test {
    use crate::ansi_escape_code::ANSIEscapeCode;
    use super::*;

    #[test]
    fn create_simple_ansi_code() {
        let p = ANSIEscapeCode::new(String::from("33"));
        assert_eq!(p.code(), "\\x1b[33m")
    }

    #[test]
    fn create_reset_ansi_code() {
        let reset_ansi = ANSIEscapeCode::new(String::from("0"));
        assert_eq!("\\x1b[0m", reset_ansi.code());
    }

    #[test]
    fn create_bright_cyan_ansi_code() {
        let reset_ansi = ANSIEscapeCode::new(String::from("96"));
        assert_eq!("\\x1b[96m", reset_ansi.code());
    }
}
