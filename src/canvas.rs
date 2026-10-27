use crate::{BoundingBox, InkError, Point, Stroke};
use serde::{Deserialize, Serialize};
use std::fs;
use svg::{Document, node::element::Rectangle};

#[derive(Debug, Deserialize, Serialize)]
pub struct Canvas {
    pub strokes: Vec<Stroke>,
}

// A canvas holds many strokes
impl Canvas {
    // From Json
    pub fn from_json_file(path: &str) -> Result<Self, InkError> {
        let content = fs::read_to_string(path)?;
        let result = serde_json::from_str(&content)?;
        Result::Ok(result)
    }

    // Writing Canvas data back to a Json file
    pub fn to_json_file(&self, path: &str) -> Result<(), InkError> {
        let result = serde_json::to_string(self)?;
        fs::write(path, result)?;
        Ok(())
    }

    // Remove a stroke from the canvas from index
    pub fn remove_stroke(&mut self, index: usize) -> Result<Stroke, InkError> {
        if index >= self.strokes.len() {
            return Err(InkError::StrokeNotFound(index));
        }
        Ok(self.strokes.remove(index))
    }

    // Remove everything on the canvas
    pub fn clear(&mut self) {
        self.strokes.clear();
    }

    // Find the bounding box that encloses all the strokes on the canvas
    pub fn overall_bounding_box(&self) -> Option<BoundingBox> {
        // If all the strokes are empty, None will be returned
        self.strokes
            .iter()
            // Evaluates the bounding box only when the strokes have valid points, ie, not None
            .filter_map(|stroke| stroke.find_bounding_box())
            .reduce(|acc, e| acc.join(&e))
    }

    // Find bounding box for each of the strokes on the canvas at once
    pub fn each_bounding_box(&self) -> Vec<Option<BoundingBox>> {
        self.strokes
            .iter()
            .map(|stroke| stroke.find_bounding_box())
            .collect()
    }

    // Shift every point on the canvas
    pub fn translate(&mut self, dx: f32, dy: f32) {
        for stroke in &mut self.strokes {
            for point in &mut stroke.points {
                point.x += dx;
                point.y += dy
            }
        }
    }

    // Check if a points hits a stroke or not
    pub fn hit_test(&self, point: Point, tolerance: f32) -> bool {
        self.strokes
            .iter()
            .any(|stroke| stroke.hit_test(&point, tolerance))
    }

    // Render a SVG of the canvas
    pub fn save_to_svg(&self, path: &str) -> std::io::Result<()> {
        let bbox = self.overall_bounding_box().unwrap_or(BoundingBox {
            // Default canvas size if there is nothing on the canvas
            min_x: 0.0,
            min_y: 0.0,
            max_x: 500.0,
            max_y: 500.0,
        });

        let view_x = bbox.min_x;
        let view_y = bbox.min_y;
        let view_w = bbox.max_x - bbox.min_x;
        let view_h = bbox.max_y - bbox.min_y;

        // Create white background rectangle
        let background = Rectangle::new()
            .set("x", view_x)
            .set("y", view_y)
            .set("width", view_w)
            .set("height", view_h)
            .set("fill", "#ffffff");

        // Initialize SVG document
        let mut document = Document::new()
            .set("viewBox", (view_x, view_y, view_w, view_h))
            .add(background);

        for stroke in &self.strokes {
            if !stroke.points.is_empty() {
                document = document.add(stroke.to_svg_polyline(2.5));
            }
        }

        svg::save(path, &document)
    }
}
