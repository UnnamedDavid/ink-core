use crate::Point;
use crate::{BoundingBox, Color};
use serde::{Deserialize, Serialize};
use svg::node::element::Polyline;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Stroke {
    pub color: Color,
    pub points: Vec<Point>,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            // Black
            color: Color::new(0.0, 0.0, 0.0),
            points: Vec::new(),
        }
    }
}
// A stroke is a vector of points, meaning a 'stroke' of line
impl Stroke {
    // Contructor
    pub fn new(points: Vec<Point>, color: Color) -> Self {
        Self { color, points }
    }
    // Finds the bounding box of the stroke of line, that is, the smallest box
    // that encloses the stroke.
    pub fn find_bounding_box(&self) -> Option<BoundingBox> {
        self.points
            .iter()
            .map(|p| BoundingBox::from_point(p.x, p.y))
            .reduce(|acc, e| acc.join(&e))
    }

    // Add a point to this stroke
    pub fn add_point(&mut self, point: Point) {
        self.points.append(&mut vec![point]);
    }

    // Calculate the length of the stroke
    pub fn length(&self) -> f32 {
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

    // Determine if a point hits the stroke
    pub fn hit_test(&self, point: &Point, tolerance: f32) -> bool {
        let Some(stroke_bbox) = self.find_bounding_box() else {
            return false;
        };
        let query_bbox = BoundingBox::from_point(point.x, point.y).pad(tolerance);

        if !query_bbox.intersects(&stroke_bbox) {
            return false;
        }

        let tolerance_sq = tolerance * tolerance;

        match self.points.as_slice() {
            [] => false,
            [single] => point.distance_squared(single) <= tolerance_sq,
            points => points
                .windows(2)
                .any(|seg| point.distance_squared_to_segment(&seg[0], &seg[1]) <= tolerance_sq),
        }
    }

    pub fn stroke_hit_test(&self, other: &Self, tolerance: f32) -> bool {
        const CHUNK_SIZE: usize = 16;
        // Check if each stroke is empty
        let Some(self_bbox) = self.find_bounding_box() else {
            return false;
        };
        let Some(other_bbox) = other.find_bounding_box() else {
            return false;
        };

        // Check if each other's bounding box intersects with tolerance
        if !other_bbox.pad(tolerance).intersects(&self_bbox) {
            return false;
        };

        // Split each stroke into chunk of stroke each with maximum of 50 points
        let mut self_chunks: Vec<Stroke> = Vec::new();
        let mut other_chunks: Vec<Stroke> = Vec::new();

        for start_point in (0..self.points.len()).step_by(CHUNK_SIZE) {
            let end_point = (start_point + CHUNK_SIZE).min(self.points.len() - 1);
            self_chunks.push(Stroke {
                points: self.points[start_point..=end_point].to_vec(),
                ..Default::default()
            });
        }

        for start_point in (0..other.points.len()).step_by(CHUNK_SIZE) {
            let end_point = (start_point + CHUNK_SIZE).min(other.points.len() - 1);
            other_chunks.push(Stroke {
                points: other.points[start_point..=end_point].to_vec(),
                ..Default::default()
            });
        }

        for self_chunk in &self_chunks {
            let Some(self_chunk_bbox) = self_chunk.find_bounding_box() else {
                continue;
            };

            // Check if each chunk of the stroke's bbox has intersect with other's
            // bbox with tolerance
            if !self_chunk_bbox.intersects(&other_bbox.pad(tolerance)) {
                continue;
            }

            for other_chunk in &other_chunks {
                let Some(other_chunk_bbox) = other_chunk.find_bounding_box() else {
                    continue;
                };

                if !self_chunk_bbox.pad(tolerance).intersects(&other_chunk_bbox) {
                    continue;
                }

                if self_chunk
                    .points
                    .iter()
                    .any(|p| other_chunk.hit_test(p, tolerance))
                    || other_chunk
                        .points
                        .iter()
                        .any(|p| self_chunk.hit_test(p, tolerance))
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn to_svg_polyline(&self, width: f32) -> Polyline {
        let points_str = self
            .points
            .iter()
            .map(|p| format!("{:.2}, {:.2}", p.x, p.y))
            .collect::<Vec<_>>()
            .join(" ");

        Polyline::new()
            .set("fill", "none")
            .set("stroke", self.color.to_svg_rgb())
            .set("stroke-width", width)
            .set("stroke-linecap", "round")
            .set("stroke-linejoin", "round")
            .set("points", points_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Using negative and positive numbers that is not the origin to ensure bbox isn't anchored at origin.
    const TEST_X: f32 = -52.0;
    const TEST_Y: f32 = 71.0;

    fn dummy_color() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
    #[test]
    fn test_empty_stroke() {
        let stroke = Stroke::new(Vec::new(), dummy_color());
        assert_eq!(stroke.find_bounding_box(), None);
        assert_eq!(stroke.length(), 0.0);
    }

    #[test]
    fn test_single_point_stroke() {
        let mut stroke = Stroke::new(Vec::new(), dummy_color());
        stroke.add_point(Point::from_xy(TEST_X, TEST_Y));
        assert_eq!(stroke.length(), 0.0);

        let bbox = stroke.find_bounding_box().unwrap();
        assert_eq!(bbox.min_x, -52.0);
        assert_eq!(bbox.max_x, -52.0);
        assert_eq!(bbox.min_y, 71.0);
        assert_eq!(bbox.max_y, 71.0);
    }
}
