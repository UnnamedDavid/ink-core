use serde::{Deserialize, Serialize};

// Color struct that stores red, gree, blue each from value 0.0-1.0
#[derive(Debug, Deserialize, Serialize)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}
