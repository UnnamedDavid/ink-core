use crate::{Color, Point, Stroke, parser::PaintOptions};
use std::io::{self, Write};

const DEFAULT_RADIUS: f32 = 5.0;
const PLACEHOLDER_TIMESTAMP: u64 = 0;
const DEFAULT_PRESSURE: f32 = 1.0;

pub enum PaintStatus {
    Continue,
    Quit,
}

pub struct PaintMode {
    pub brush: Brush,
}
pub enum Brush {
    Eraser { radius: f32 },
    Pen { radius: f32, opacity: f32 },
    Highlighter { radius: f32 },
}

impl PaintMode {
    pub fn print_help() {
        println!("Paint mode lets you make changes on the canvas.");
        println!("Available Commands:");
        println!("  tool <pen | highligher | eraser>   --Change the brush you are using");
        println!("  draw                               --Start drawing a stroke");
        println!("  save                               --Save your changes");
        println!("  quit                               --exit paint mode");
        println!("");
        println!("Available Options:");
        println!("  -r VALUE --radius VALUE             --Set pen radius of current brush");
        println!(
            "  -o VALUE --opacity VALUE             --Set opacity of current brush, available only to highligher"
        )
    }

    // paint mode interactive CLI
    pub fn execute(&self, positionals: &Vec<&str>, options: &Vec<PaintOptions>) -> PaintStatus {
        match positionals.first().copied() {
            Some("tool") => match positionals.get(1).copied() {
                Some("pen") => {
                    todo!();
                }
                _ => {
                    println!("missing argument <pen | highligher | eraser> ");
                    return PaintStatus::Continue;
                }
            },
            Some("draw") => {
                Self::draw_stroke();
                return PaintStatus::Continue;
            }
            _ => {
                println!("unrecognized input, type help to see commands");
                return PaintStatus::Continue;
            }
        }
    }

    // Create a stroke from inputted points
    // Self contained input methods
    pub fn draw_stroke() -> Stroke {
        let mut usr_stroke: Stroke = Stroke::new(Vec::new(), Color::new(1.0, 0.0, 0.0));
        println!("Start editing stroke, to add a point use <x> <y>, ENTER to finish");
        loop {
            print!("point {}> ", usr_stroke.points.len() + 1);
            let _ = io::stdout().flush();

            // Read user input
            let mut response = String::new();
            match io::stdin().read_line(&mut response) {
                Ok(0) => break,
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Failed to process your input: {e}");
                    continue;
                }
            };

            // Handle when user wants to finish editing stroke
            if response.trim().is_empty() {
                break;
            }

            // Parse command
            // Check if there are only two arguments
            let mut parts = response.split_whitespace();
            let (Some(x), Some(y), None) = (parts.next(), parts.next(), parts.next()) else {
                eprintln!("Expected format: <x> <y>");
                continue;
            };

            // Check argument type
            let (Ok(x), Ok(y)) = (x.parse::<f32>(), y.parse::<f32>()) else {
                eprintln!("Expected float numbers");
                continue;
            };

            usr_stroke.add_point(Point::new(x, y, PLACEHOLDER_TIMESTAMP, DEFAULT_PRESSURE));
        }
        usr_stroke
    }
}
