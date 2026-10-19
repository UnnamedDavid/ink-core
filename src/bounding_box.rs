use serde::Deserialize;
use serde::Serialize;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Canvas {
    pub strokes: Vec<Stroke>,
}

// A canvas holds many strokes
impl Canvas {
    // From Json
    pub fn from_json_file(path: &str) -> Self {
        let content = fs::read_to_string(path).expect("Could not read json file");
        serde_json::from_str(&content).expect("Json file is not well-formatted")
    }

    // Find the bounding box that encloses all the strokes on the canvas
    pub fn overall_bounding_box(&self) -> Option<BoundingBox> {
        // If all the strokes are empty, None will be returned
        self.strokes
            .iter()
            // Evaluates the bounding box only when the strokes have valid points, ie, not None
            .filter_map(|stroke| stroke.find_bounding_box())
            .reduce(|acc, e| acc.join(e))
    }

    // Find bounding box for each of the strokes on the canvas at once
    pub fn each_bounding_box(&self) -> Vec<Option<BoundingBox>> {
        self.strokes
            .iter()
            .map(|stroke| stroke.find_bounding_box())
            .collect()
    }
}

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
    fn find_bounding_box(&self) -> Option<BoundingBox> {
        self.points
            .iter()
            .map(|p| BoundingBox::from_point(p.x, p.y))
            .reduce(|acc, e| acc.join(e))
    }

    // Add a point to this stroke
    fn add_point(&mut self, point: Point) {
        self.points.append(&mut vec![point]);
    }

    // Calculate the length of the stroke
    fn total_length(&self) -> f32 {
        self.points
            .windows(2)
            .map(|w| {
                // Calculates the euclidian distance between 2 points
                let p1 = &w[0];
                let p2 = &w[1];
                let diff_x = p1.x - p2.x;
                let diff_y = p2.y - p2.y;
                (diff_x.powi(2) + diff_y * diff_y).sqrt()
            })
            // Summing the distances to get total distance
            .sum()
    }
}

#[derive(Debug, Deserialize, Serialize)]
// Information every point in a stroke holds
pub struct Point {
    x: f32,
    y: f32,
    timestamp: u64,
    pressure: f32,
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
    fn join(&self, other: BoundingBox) -> BoundingBox {
        BoundingBox {
            min_x: self.min_x.min(other.min_x),
            min_y: self.min_y.min(other.min_y),
            max_x: self.max_x.max(other.max_x),
            max_y: self.max_y.max(other.max_y),
        }
    }
}

// Color struct that stores red, gree, blue each from value 0.0-1.0
#[derive(Debug, Deserialize, Serialize)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}
