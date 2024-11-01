pub struct ANSIEscapeCode {
    parameter: String,
}
impl ANSIEscapeCode {
    pub fn new(parameter: String) -> Self {
        ANSIEscapeCode { parameter }
    }

    pub fn code(&self) -> String {
        format!("\\x1b[{}m", self.parameter)
    }
}
