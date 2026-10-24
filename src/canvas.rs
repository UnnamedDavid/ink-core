use crate::{BoundingBox, InkError, Stroke};
use serde::{Deserialize, Serialize};
use std::fs;

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
