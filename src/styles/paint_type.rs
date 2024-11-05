use super::Stylify;

// =======================================================================

#[derive(Debug, Clone)]
pub enum PaintType {
    FG,
    BG,
}

impl Stylify for PaintType {
    fn make_styles(&self, _paint_type: Option<&PaintType>) -> String {
        String::new()
    }
}
