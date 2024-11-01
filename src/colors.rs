pub struct BasicColor {
    fg: u8,
    bg: u8,
}

pub const BLACK: BasicColor = BasicColor { fg: 30, bg: 40 };
pub const RED: BasicColor = BasicColor { fg: 31, bg: 41 };
pub const GREEN: BasicColor = BasicColor { fg: 32, bg: 42 };
pub const YELLOW: BasicColor = BasicColor { fg: 33, bg: 43 };
pub const BLUE: BasicColor = BasicColor { fg: 34, bg: 44 };
pub const MAGENTA: BasicColor = BasicColor { fg: 35, bg: 45 };
pub const CYAN: BasicColor = BasicColor { fg: 36, bg: 46 };
pub const WHITE: BasicColor = BasicColor { fg: 37, bg: 47 };
pub const GRAY: BasicColor = BasicColor { fg: 90, bg: 10 };
pub const BRIGHT_RED: BasicColor = BasicColor { fg: 91, bg: 101 };
pub const BRIGHT_GREEN: BasicColor = BasicColor { fg: 92, bg: 102 };
pub const BRIGHT_YELLOW: BasicColor = BasicColor { fg: 93, bg: 103 };
pub const BRIGHT_BLUE: BasicColor = BasicColor { fg: 94, bg: 104 };
pub const BRIGHT_MAGENTA: BasicColor = BasicColor { fg: 95, bg: 105 };
pub const BRIGHT_CYAN: BasicColor = BasicColor { fg: 96, bg: 106 };
pub const BRIGHT_WHITE: BasicColor = BasicColor { fg: 97, bg: 107 };

//

pub struct PaletteColor {
    index: u8,
}

pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

// =======================================================================

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn black_colors() {
        assert_eq!(30, BLACK.fg);
        assert_eq!(40, BLACK.bg)
    }

    #[test]
    fn cyan_colors() {
        assert_eq!(36, CYAN.fg);
        assert_eq!(46, CYAN.bg)
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
    fn red_rgb_color() {
        let red = RGB { r: 255, g: 0, b: 0 };
        assert_eq!(255, red.r);
        assert_eq!(0, red.g);
        assert_eq!(0, red.b)
    }

    #[test]
    fn salmon_rgb_color() {
        let salmon = RGB { r: 250, g: 128, b: 114 };
        assert_eq!(250, salmon.r);
        assert_eq!(128, salmon.g);
        assert_eq!(114, salmon.b)
    }
}
