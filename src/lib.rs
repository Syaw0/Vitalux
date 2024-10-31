struct ANSIEscapeCode {
    parameter: String,
}
impl ANSIEscapeCode {
    fn new(parameter: String) -> Self {
        ANSIEscapeCode { parameter }
    }

    fn code(&self) -> String {
        format!("\\x1b[{}m", self.parameter)
    }
}

#[cfg(test)]
mod test {
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
}
