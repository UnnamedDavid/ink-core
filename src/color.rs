use serde::{Deserialize, Serialize};

// Color struct that stores red, gree, blue each from value 0.0-1.0
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}
impl Color {
    // Contructor
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r: r, g: g, b: b }
    }

    // Outputs syntax that is ready to use in svg
    pub fn to_svg_rgb(&self) -> String {
        let r = (self.r.clamp(0.0, 1.0) * 255.0).round() as u8;
        let g = (self.g.clamp(0.0, 1.0) * 255.0).round() as u8;
        let b = (self.b.clamp(0.0, 1.0) * 255.0).round() as u8;
        format!("rgb({r}, {g}, {b})")
    }
}
