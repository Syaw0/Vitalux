use crate::colors::PaintType;

pub trait Stylify {
    fn make_styles(&self, paint_type: Option<&PaintType>) -> String;
}
