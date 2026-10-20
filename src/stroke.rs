use crate::geometry::Point;
use crate::{BoundingBox, Color};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Stroke {
    pub color: Color,
    pub points: Vec<Point>,
}

// A stroke is a vector of points, meaning a 'stroke' of line
impl Stroke {
    // Contructor
    // pub fn new(points: Vec<(f64, f64)>) -> Self {
    //     Self { points }
    // }

    // Finds the bounding box of the stroke of line, that is, the smallest box
    // that encloses the stroke.
    pub fn find_bounding_box(&self) -> Option<BoundingBox> {
        self.points
            .iter()
            .map(|p| BoundingBox::from_point(p.x, p.y))
            .reduce(|acc, e| acc.join(e))
    }

    // Add a point to this stroke
    pub fn add_point(&mut self, point: Point) {
        self.points.append(&mut vec![point]);
    }

    // Calculate the length of the stroke
    pub fn total_length(&self) -> f32 {
        self.points
            .windows(2)
            .map(|w| {
                // Calculates the euclidian distance between 2 points
                let p1 = &w[0];
                let p2 = &w[1];
                let diff_x = p1.x - p2.x;
                let diff_y = p1.y - p2.y;
                (diff_x.powi(2) + diff_y * diff_y).sqrt()
            })
            // Summing the distances to get total distance
            .sum()
    }
}
