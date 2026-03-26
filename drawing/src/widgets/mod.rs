use std::collections::HashMap;

use crate::{model::pixel::Pixel, widgets::area::Area};

pub mod area;
pub mod color_picker;

pub trait Widget {
    fn render(&self, area: &Area, buffer: &mut HashMap<(u16, u16), Pixel>);
}
