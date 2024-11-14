pub use crate::helpers::{IntoStyle, IntoStyled};
pub use crate::styles::basic_color::BasicColor;
pub use crate::styles::formatter::Formatter;

pub mod colors {
    use crate::styles::basic_color;

    pub use basic_color::BLACK;
    pub use basic_color::BLUE;
    pub use basic_color::BRIGHT_BLUE;
    pub use basic_color::BRIGHT_CYAN;
    pub use basic_color::BRIGHT_GREEN;
    pub use basic_color::BRIGHT_MAGENTA;
    pub use basic_color::BRIGHT_RED;
    pub use basic_color::BRIGHT_WHITE;
    pub use basic_color::BRIGHT_YELLOW;
    pub use basic_color::CYAN;
    pub use basic_color::GRAY;
    pub use basic_color::GREEN;
    pub use basic_color::MAGENTA;
    pub use basic_color::RED;
    pub use basic_color::WHITE;
    pub use basic_color::YELLOW;
}

pub mod formats {
    use crate::styles::formatter;

    pub use formatter::BOLD;
    pub use formatter::FAINT;
    pub use formatter::ITALIC;
    pub use formatter::OVERLINE;
    pub use formatter::RAPID_BLINK;
    pub use formatter::RESET;
    pub use formatter::SLOW_BLINK;
    pub use formatter::UNDERLINE;
}
