use super::{ paint_type::PaintType, Stylify };

// =======================================================================

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
