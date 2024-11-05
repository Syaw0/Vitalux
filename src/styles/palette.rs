use super::{ paint_type::PaintType, Stylify };

// =======================================================================

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
