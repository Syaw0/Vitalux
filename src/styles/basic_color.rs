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
    fn black_color() {
        match BLACK {
            Styles::StyleBasicColor(BasicColor { fg: 30, bg: 40 }) => assert!(true),
            _ => panic!("It's not a `Black` color! black color should have fg:30 and bg:40"),
        }
    }

    #[test]
    fn red_color() {
        match RED {
            Styles::StyleBasicColor(BasicColor { fg: 31, bg: 41 }) => assert!(true),
            _ => panic!("It's not a `Red` color! red color should have fg:31 and bg:41"),
        }
    }
}
