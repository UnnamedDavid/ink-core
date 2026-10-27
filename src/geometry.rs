use serde::{Deserialize, Serialize};
// Information every point in a stroke holds
#[derive(Debug, Deserialize, Serialize, PartialEq)]
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

    // the squared distance between two points (x2 - x1)² + (y2 - y1)²
    pub fn distance_squared(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    // The nearest squared distance between a point and a segment
    pub fn distance_squared_to_segment(&self, a: &Point, b: &Point) -> f32 {
        // Vector AB
        let ab_x = b.x - a.x;
        let ab_y = b.y - a.y;
        let dist_ab_sq = ab_x * ab_x + ab_y * ab_y;

        // Prevent division by zero: when point A and B are the same points
        if dist_ab_sq == 0.0 {
            return self.distance_squared(a);
        }

        let ap_x = a.x - self.x;
        let ap_y = a.y - self.y;

        // Concepts: Parametric lines & dot product
        // ap_x * ab_x + ap_y * ab_y = cos θ * |ap| * |ab|
        // cos θ * |ap| = leg of the right triangle with hypotenuse |ap| = our desired distance
        // Droping a perpendicular line from self to line AB intersecting at point P
        // and find the fraction t of the way P is from point A to point B
        let t = ((ap_x * ab_x + ap_y * ab_y) / dist_ab_sq).clamp(0.0, 1.0);

        let closest = Point {
            x: a.x + t * ab_x,
            y: a.y + t * ab_y,
            timestamp: 0,
            pressure: 0.0,
        };

        self.distance_squared(&closest)
    }
}
// A bounding box
#[derive(Debug, PartialEq)]
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
    pub fn join(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox {
            min_x: self.min_x.min(other.min_x),
            min_y: self.min_y.min(other.min_y),
            max_x: self.max_x.max(other.max_x),
            max_y: self.max_y.max(other.max_y),
        }
    }

    // Check if current bounding box is in contact with another
    // De Morgan's laws
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        other.min_x <= self.max_x
            && other.max_x >= self.min_x
            && other.min_y <= self.max_y
            && other.max_y >= self.min_y
    }
}
