use super::{ paint_type::PaintType, Styles, Stylify };

// =======================================================================

#[derive(Debug, Clone)]
pub struct BasicColor {
    fg: u8,
    bg: u8,
}

impl Stylify for BasicColor {
    /// If the `is_foreground` was None it's assume as foreground
    fn make_styles(&self, paint_type: Option<&PaintType>) -> String {
        let paint_type = paint_type.unwrap_or(&PaintType::FG);
        format!("{}", match paint_type {
            PaintType::FG => self.fg,
            PaintType::BG => self.bg,
        })
    }
}

macro_rules! color_code {
    ($name:ident, { fg: $fg:expr, bg: $bg:expr }) => {
        pub const $name:Styles = Styles::StyleBasicColor(BasicColor { fg: $fg, bg: $bg });
    };
}

color_code!(BLACK,{fg: 30, bg: 40 });
color_code!(RED,{fg: 31, bg: 41});
color_code!(GREEN,{ fg: 32, bg: 42 });
color_code!(YELLOW,{ fg: 33, bg: 43 });
color_code!(BLUE,{fg: 34, bg: 44});
color_code!(MAGENTA,{ fg: 35, bg: 45});
color_code!(CYAN,{fg:36,bg:46});
color_code!(WHITE,{fg:37,bg:47});
color_code!(GRAY,{fg:90,bg:100});
color_code!(BRIGHT_RED,{fg:91,bg:101});
color_code!(BRIGHT_GREEN,{fg:92,bg:102});
color_code!(BRIGHT_YELLOW,{fg:93,bg:103});
color_code!(BRIGHT_BLUE,{fg:94,bg:104});
color_code!(BRIGHT_MAGENTA,{fg:95,bg:105});
color_code!(BRIGHT_CYAN,{fg:96,bg:106});
color_code!(BRIGHT_WHITE,{fg:97,bg:107});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_style_fg() {
        let color = BasicColor { fg: 30, bg: 40 };
        let styles = color.make_styles(Some(&PaintType::FG));
        assert_eq!(styles, "30")
    }

    #[test]
    fn test_make_style_default_fg() {
        let color = BasicColor { fg: 34, bg: 44 };
        let styles = color.make_styles(None);
        assert_eq!(styles, "34")
    }

    #[test]
    fn test_make_style_bg() {
        let color = BasicColor { fg: 30, bg: 40 };
        let styles = color.make_styles(Some(&PaintType::BG));
        assert_eq!(styles, "40")
    }

    #[test]
    fn test_fg_and_bg_values() {
        let color = BasicColor { fg: 30, bg: 40 };
        let styles_fg = color.make_styles(Some(&PaintType::FG));
        let styles_bg = color.make_styles(Some(&PaintType::BG));
        assert_eq!(styles_fg, "30");
        assert_eq!(styles_bg, "40");
    }

    #[test]
    fn test_fg_and_bg_values_with_none_paint_type() {
        let color = BasicColor { fg: 30, bg: 40 };
        let styles_fg = color.make_styles(None);
        let styles_bg = color.make_styles(Some(&PaintType::BG));
        assert_eq!(styles_fg, "30");
        assert_eq!(styles_bg, "40");
    }

    #[test]
    fn test_color_code_macro() {
        color_code!(TEST_COLOR,{fg:100,bg:200});
        match TEST_COLOR {
            Styles::StyleBasicColor(BasicColor { fg, bg }) => {
                assert_eq!(fg, 100);
                assert_eq!(bg, 200);
            }
            _ => panic!("TEST_COLOR is not a BasicColor"),
        }
    }

    macro_rules! color_test {
        ($test_name:ident, $color_name:ident, $color:expr, $fg:expr, $bg:expr) => {
            #[test]
            fn $test_name(){
                match $color_name{
                    Styles::StyleBasicColor(BasicColor{fg,bg})=>{
                        assert_eq!(fg,$fg);
                        assert_eq!(bg,$bg);
                    },
                    _=>{
                        panic!("This color is not a basic color!"); 
                    }
                }
            }
        };
    }

    color_test!(black_color, BLACK, "Black", 30, 40);
    color_test!(red_color, RED, "Red", 31, 41);
    color_test!(green_color, GREEN, "Green", 32, 42);
    color_test!(yellow_color, YELLOW, "Yellow", 33, 43);
    color_test!(blue_color, BLUE, "Blue", 34, 44);
    color_test!(magenta_color, MAGENTA, "Magenta", 35, 45);
    color_test!(cyan_color, CYAN, "Cyan", 36, 46);
    color_test!(white_color, WHITE, "White", 37, 47);
    color_test!(gray_color, GRAY, "Gray", 90, 100);
    color_test!(bright_red_color, BRIGHT_RED, "Bright Red", 91, 101);
    color_test!(bright_green_color, BRIGHT_GREEN, "Bright Green", 92, 102);
    color_test!(bright_yellow_color, BRIGHT_YELLOW, "Bright Yellow", 93, 103);
    color_test!(bright_blue_color, BRIGHT_BLUE, "Bright Blue", 94, 104);
    color_test!(bright_magenta_color, BRIGHT_MAGENTA, "Bright Magenta", 95, 105);
    color_test!(bright_cyan_color, BRIGHT_CYAN, "Bright Cyan", 96, 106);
    color_test!(bright_white_color, BRIGHT_WHITE, "Bright White", 97, 107);
}
