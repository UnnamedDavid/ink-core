use serde::{Deserialize, Serialize};
// Information every point in a stroke holds
#[derive(Debug, Deserialize, Serialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub timestamp: u64,
    pub pressure: f32,
}

impl Point {
    // Constructor
    pub fn new(x: f32, y: f32, timestamp: u64, pressure: f32) -> Self {
        Self {
            x,
            y,
            timestamp,
            pressure,
        }
    }
}
// A bounding box
#[derive(Debug)]
pub struct BoundingBox {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl BoundingBox {
    // Creates a bounding box using a single point, useful for the find bounding box function
    pub fn from_point(x: f32, y: f32) -> Self {
        BoundingBox {
            min_x: x,
            min_y: y,
            max_x: x,
            max_y: y,
        }
    }
    // Find the bounding box of two bounding boxes
    pub fn join(&self, other: BoundingBox) -> BoundingBox {
        BoundingBox {
            min_x: self.min_x.min(other.min_x),
            min_y: self.min_y.min(other.min_y),
            max_x: self.max_x.max(other.max_x),
            max_y: self.max_y.max(other.max_y),
        }
    }
}
