pub trait Stylify {
    fn make_styles(&self, paint_type: Option<&PaintType>) -> String;
}
#[derive(Debug, Clone)]
pub enum PaintType {
    FG,
    BG,
}

#[derive(Debug, Clone)]
pub enum Styles {
    StyleRgb(Rgb),
    StyleBasicColor(BasicColor),
    StylePaletteColor(PaletteColor),
    StylePaintType(PaintType),
}

impl Styles {
    pub fn make_styles(&self, paint_type: Option<&PaintType>) -> String {
        match self {
            Styles::StyleBasicColor(c) => c.make_styles(paint_type),
            Styles::StylePaintType(c) => c.make_styles(paint_type),
            Styles::StylePaletteColor(c) => c.make_styles(paint_type),
            Styles::StyleRgb(c) => c.make_styles(paint_type),
        }
    }
}

impl Stylify for PaintType {
    fn make_styles(&self, _paint_type: Option<&PaintType>) -> String {
        String::new()
    }
}
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

pub const BLACK: Styles = Styles::StyleBasicColor(BasicColor { fg: 30, bg: 40 });
pub const RED: Styles = Styles::StyleBasicColor(BasicColor { fg: 31, bg: 41 });
pub const GREEN: Styles = Styles::StyleBasicColor(BasicColor { fg: 32, bg: 42 });
pub const YELLOW: Styles = Styles::StyleBasicColor(BasicColor { fg: 33, bg: 43 });
pub const BLUE: Styles = Styles::StyleBasicColor(BasicColor { fg: 34, bg: 44 });
pub const MAGENTA: Styles = Styles::StyleBasicColor(BasicColor { fg: 35, bg: 45 });
pub const CYAN: Styles = Styles::StyleBasicColor(BasicColor { fg: 36, bg: 46 });
pub const WHITE: Styles = Styles::StyleBasicColor(BasicColor { fg: 37, bg: 47 });
pub const GRAY: Styles = Styles::StyleBasicColor(BasicColor { fg: 90, bg: 10 });
pub const BRIGHT_RED: Styles = Styles::StyleBasicColor(BasicColor { fg: 91, bg: 101 });
pub const BRIGHT_GREEN: Styles = Styles::StyleBasicColor(BasicColor { fg: 92, bg: 102 });
pub const BRIGHT_YELLOW: Styles = Styles::StyleBasicColor(BasicColor { fg: 93, bg: 103 });
pub const BRIGHT_BLUE: Styles = Styles::StyleBasicColor(BasicColor { fg: 94, bg: 104 });
pub const BRIGHT_MAGENTA: Styles = Styles::StyleBasicColor(BasicColor { fg: 95, bg: 105 });
pub const BRIGHT_CYAN: Styles = Styles::StyleBasicColor(BasicColor { fg: 96, bg: 106 });
pub const BRIGHT_WHITE: Styles = Styles::StyleBasicColor(BasicColor { fg: 97, bg: 107 });

//
#[derive(Debug, Clone)]
pub struct PaletteColor {
    pub index: u8,
}

impl Stylify for PaletteColor {
    fn make_styles(&self, paint_type: Option<&PaintType>) -> String {
        let paint_type = paint_type.unwrap_or(&PaintType::FG);
        format!(
            "{};5;{}",
            match paint_type {
                PaintType::FG => "38",
                PaintType::BG => "48",
            },
            self.index
        )
    }
}
#[derive(Debug, Clone)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Stylify for Rgb {
    fn make_styles(&self, paint_type: Option<&PaintType>) -> String {
        let paint_type = paint_type.unwrap_or(&PaintType::FG);
        format!(
            "{};2;{};{};{}",
            match paint_type {
                PaintType::FG => "38",
                PaintType::BG => "48",
            },
            self.r,
            self.g,
            self.b
        )
    }
}

// =======================================================================

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn black_colors() {
        let black = BasicColor { fg: 30, bg: 40 };
        assert_eq!(30, black.fg);
        assert_eq!(40, black.bg)
    }

    #[test]
    fn cyan_colors() {
        let cyan = BasicColor { fg: 36, bg: 46 };
        assert_eq!(36, cyan.fg);
        assert_eq!(46, cyan.bg)
    }

    #[test]
    fn paint_yellow_foreground() {
        let painted_fg = YELLOW.make_styles(None);

        assert_eq!("33", painted_fg)
    }

    #[test]
    fn paint_magenta_background() {
        let painted_bg = MAGENTA.make_styles(Some(&PaintType::BG));

        assert_eq!("45", painted_bg)
    }

    #[test]
    fn black_palette_color() {
        let standard_black = PaletteColor { index: 0 };
        assert_eq!(0, standard_black.index)
    }

    #[test]
    fn gray_scale_palette_color() {
        let gray_scale = PaletteColor { index: 243 };
        assert_eq!(243, gray_scale.index)
    }

    #[test]
    fn palette_paint_green_fg() {
        let green = PaletteColor { index: 118 };
        let painted_fg = green.make_styles(None);
        assert_eq!("38;5;118", painted_fg)
    }

    #[test]
    fn palette_paint_blue_bg() {
        let blue = PaletteColor { index: 33 };
        let painted_bg = blue.make_styles(Some(&PaintType::BG));
        assert_eq!("48;5;33", painted_bg)
    }

    #[test]
    fn red_rgb_color() {
        let red = Rgb { r: 255, g: 0, b: 0 };
        assert_eq!(255, red.r);
        assert_eq!(0, red.g);
        assert_eq!(0, red.b)
    }

    #[test]
    fn salmon_rgb_color() {
        let salmon = Rgb { r: 250, g: 128, b: 114 };
        assert_eq!(250, salmon.r);
        assert_eq!(128, salmon.g);
        assert_eq!(114, salmon.b)
    }

    #[test]
    fn rgb_paint_purple_fg() {
        let purple = Rgb { r: 126, g: 50, b: 219 };
        let painted_fg = purple.make_styles(None);
        assert_eq!("38;2;126;50;219", painted_fg)
    }

    #[test]
    fn rgb_paint_orange_bg() {
        let orange = Rgb { r: 219, g: 132, b: 50 };
        let painted_bg = orange.make_styles(Some(&PaintType::BG));
        assert_eq!("48;2;219;132;50", painted_bg)
    }
}
