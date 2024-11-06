use super::{ paint_type::PaintType, Styles, Stylify };

// =======================================================================

/// A struct representing a Formatter with code value.
#[derive(Debug, Clone)]
pub struct Formatter {
    code: u8,
}

impl Stylify for Formatter {
    /// Returns a string representation of the formatter style.
    ///
    /// The `PaintType` is not any involved in this method!
    fn make_styles(&self, _paint_type: Option<&PaintType>) -> String {
        format!("{}", self.code)
    }
}

/// A macro for generating formatter constants.
macro_rules! formatter_code {
    ($name:ident, $code:expr) => {
        pub const $name: Styles = Styles::StyleFormatter(Formatter { code: $code });
    };
}

formatter_code!(RESET, 0);
formatter_code!(BOLD, 1);
formatter_code!(FAINT, 2);
formatter_code!(ITALIC, 3);
formatter_code!(UNDERLINE, 4);
formatter_code!(SLOW_BLINK, 5);
formatter_code!(RAPID_BLINK, 6);
formatter_code!(OVERLINE, 53);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_styles_fg() {
        let formatter = Formatter { code: 22 };
        let styles = formatter.make_styles(Some(&PaintType::FG));
        assert_eq!(styles, "22")
    }

    #[test]
    fn make_styles_bg() {
        let formatter = Formatter { code: 111 };
        let styles = formatter.make_styles(Some(&PaintType::BG));
        assert_eq!(styles, "111")
    }

    #[test]
    fn make_styles_default_paint_type() {
        let formatter = Formatter { code: 31 };
        let styles = formatter.make_styles(None);
        assert_eq!(styles, "31")
    }

    #[test]
    fn test_code_values() {
        let formatter = Formatter { code: 90 };
        let styles_fg = formatter.make_styles(Some(&PaintType::FG));
        let styles_bg = formatter.make_styles(Some(&PaintType::BG));
        assert_eq!(styles_fg, "90");
        assert_eq!(styles_bg, "90");
    }

    #[test]
    fn test_code_values_with_none_paint_type() {
        let formatter = Formatter { code: 47 };
        let styles_fg = formatter.make_styles(None);
        let styles_bg = formatter.make_styles(Some(&PaintType::BG));
        assert_eq!(styles_fg, "47");
        assert_eq!(styles_bg, "47");
    }

    #[test]
    fn test_formatter_code_macro() {
        formatter_code!(TEST_FORMATTER, 103);
        match TEST_FORMATTER {
            Styles::StyleFormatter(Formatter { code }) => {
                assert_eq!(code, 103);
            }
            _ => panic!("TEST_FORMATTER is not a Formatter"),
        }
    }

    /// A macro for generating formatter tests.
    macro_rules! formatter_test {
        ($test_name:ident, $formatter_name:ident, $code:expr) => {
            #[test]
            fn $test_name(){
                match $formatter_name{
                    Styles::StyleFormatter(Formatter{code})=>{
                        assert_eq!(code,$code);
                    },
                    _=>{
                        panic!("This formatter is not a Formatter!"); 
                    }
                }
            }
        };
    }

    formatter_test!(test_reset_formatter, RESET, 0);
    formatter_test!(test_bold_formatter, BOLD, 1);
    formatter_test!(test_faint_formatter, FAINT, 2);
    formatter_test!(test_italic_formatter, ITALIC, 3);
    formatter_test!(test_underline_formatter, UNDERLINE, 4);
    formatter_test!(test_slow_blink_formatter, SLOW_BLINK, 5);
    formatter_test!(test_rapid_blink_formatter, RAPID_BLINK, 6);
    formatter_test!(test_overline_formatter, OVERLINE, 53);
}
