use ink_core::{Canvas, Point};
use std::env;
use std::io::{self, Write};

// Default hit-test tolerance (in canvas units) when the user doesn't provide one
const DEFAULT_TOLERANCE: f32 = 5.0;
// Default pressure for points added from the CLI
const DEFAULT_PRESSURE: f32 = 0.5;

fn print_help() {
    println!("Available commands:");
    println!("  canvas info                       - number of strokes and overall bounding box");
    println!("  canvas translate <dx> <dy>        - shift every point on the canvas");
    println!(
        "  stroke list                       - list every stroke with index, length and color"
    );
    println!("  stroke bbox <index>               - bounding box of a stroke");
    println!("  stroke add-point <index> <x> <y>  - append a point to a stroke");
    println!("  hit <x> <y> [tolerance]           - check if a point touches any stroke");
    println!("  save <file.json | file.svg>       - save the canvas as JSON or SVG");
    println!("  help                              - show this message");
    println!("  quit                              - exit the program");
}

// Parses parts[index] into T, printing a message on failure
fn parse_arg<T: std::str::FromStr>(parts: &[&str], index: usize, name: &str) -> Option<T> {
    match parts.get(index) {
        None => {
            println!("Missing argument: <{name}>");
            None
        }
        Some(raw) => match raw.parse::<T>() {
            Ok(v) => Some(v),
            Err(_) => {
                println!("Invalid value for <{name}>: '{raw}'");
                None
            }
        },
    }
}

fn main() {
    // Reads a file path if the user provides one e.g. `cargo run -- custom.json`,
    // otherwise defaults to "canvas.json" in the root directory.
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("canvas.json"));

    let mut this_canvas = match Canvas::from_json_file(&path) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Failed to load {path}: {err}");
            return;
        }
    };

    println!("Welcome to my ink engine, type help to see commands");

    loop {
        print!("> ");
        let _ = io::stdout().flush();

        let mut response = String::new();
        match io::stdin().read_line(&mut response) {
            // EOF (e.g. Ctrl+D) - exit gracefully
            Ok(0) => break,
            Ok(_) => (),
            Err(e) => {
                println!("Failed to process your input: {e}");
                continue;
            }
        }
        let parts: Vec<&str> = response.split_whitespace().collect();

        // Ignore empty lines instead of panicking on parts[0]
        let Some(&command) = parts.first() else {
            continue;
        };

        match command {
            "help" => print_help(),
            "quit" | "exit" => break,
            "canvas" => match parts.get(1).copied() {
                Some("info") => {
                    // prints amount of strokes on the canvas and the dimentions of the canvas (overall bounding box)
                    println!("Strokes: {}", this_canvas.strokes.len());
                    match this_canvas.overall_bounding_box() {
                        Some(b) => println!(
                            "Bounding box: min ({:.2}, {:.2}), max ({:.2}, {:.2}), size {:.2} x {:.2}",
                            b.min_x,
                            b.min_y,
                            b.max_x,
                            b.max_y,
                            b.max_x - b.min_x,
                            b.max_y - b.min_y
                        ),
                        None => println!("Bounding box: none (canvas has no points)"),
                    }
                }
                Some("translate") => {
                    // receives x and y
                    // translate using translate function
                    let (Some(dx), Some(dy)) = (
                        parse_arg::<f32>(&parts, 2, "dx"),
                        parse_arg::<f32>(&parts, 3, "dy"),
                    ) else {
                        continue;
                    };
                    this_canvas.translate(dx, dy);
                    println!("Canvas translated by ({dx}, {dy})");
                }
                _ => println!("unrecognized command"),
            },
            "stroke" => match parts.get(1).copied() {
                Some("list") => {
                    // list every stroke with index and its length and its color
                    if this_canvas.strokes.is_empty() {
                        println!("Canvas contains no strokes");
                    }
                    for (i, stroke) in this_canvas.strokes.iter().enumerate() {
                        println!(
                            "[{i}] length: {:.2}, points: {}, color: {}",
                            // `+ 0.0` normalizes -0.0 (empty f32 sum) to 0.0 for display
                            stroke.length() + 0.0,
                            stroke.points.len(),
                            stroke.color.to_svg_rgb()
                        );
                    }
                }
                Some("bbox") => {
                    let Some(i) = parse_arg::<usize>(&parts, 2, "index") else {
                        continue;
                    };
                    // prints bounding box of this stroke
                    match this_canvas.strokes.get(i) {
                        None => println!("Stroke index {i} does not exist"),
                        Some(stroke) => match stroke.find_bounding_box() {
                            Some(b) => println!(
                                "Stroke {i} bounding box: min ({:.2}, {:.2}), max ({:.2}, {:.2})",
                                b.min_x, b.min_y, b.max_x, b.max_y
                            ),
                            None => println!("Stroke {i} has no points"),
                        },
                    }
                }
                Some("add-point") => {
                    // adds a point to a stroke index. eg usr input 1 30 21 (stroke index 1, (point 30,21))
                    let (Some(i), Some(x), Some(y)) = (
                        parse_arg::<usize>(&parts, 2, "index"),
                        parse_arg::<f32>(&parts, 3, "x"),
                        parse_arg::<f32>(&parts, 4, "y"),
                    ) else {
                        continue;
                    };
                    match this_canvas.strokes.get_mut(i) {
                        None => println!("Stroke index {i} does not exist"),
                        Some(stroke) => {
                            // Continue the stroke's timeline from its last point
                            let timestamp = stroke.points.last().map_or(0, |p| p.timestamp + 8);
                            stroke.add_point(Point::new(x, y, timestamp, DEFAULT_PRESSURE));
                            println!("Added point ({x}, {y}) to stroke {i}");
                        }
                    }
                }
                _ => println!("unrecognized command"),
            },
            "hit" => {
                // check if a user input point is in contact with a stroke via Canvas::hit_test
                let (Some(x), Some(y)) = (
                    parse_arg::<f32>(&parts, 1, "x"),
                    parse_arg::<f32>(&parts, 2, "y"),
                ) else {
                    continue;
                };
                let tolerance = if parts.len() > 3 {
                    match parse_arg::<f32>(&parts, 3, "tolerance") {
                        Some(t) => t,
                        None => continue,
                    }
                } else {
                    DEFAULT_TOLERANCE
                };
                if this_canvas.hit_test(Point::new(x, y, 0, 0.0), tolerance) {
                    println!("Hit! ({x}, {y}) touches a stroke (tolerance {tolerance})");
                } else {
                    println!("Miss. ({x}, {y}) does not touch any stroke (tolerance {tolerance})");
                }
            }
            "save" => {
                // eg input, mycanvas.json or drawing.svg
                let Some(&file) = parts.get(1) else {
                    println!("Missing argument: <file.json | file.svg>");
                    continue;
                };
                let lower = file.to_lowercase();
                let result = if lower.ends_with(".svg") {
                    this_canvas.save_to_svg(file).map_err(|e| e.to_string())
                } else if lower.ends_with(".json") {
                    this_canvas.to_json_file(file).map_err(|e| e.to_string())
                } else {
                    println!("Unsupported file type, use a .json or .svg extension");
                    continue;
                };
                match result {
                    Ok(()) => println!("Saved canvas to {file}"),
                    Err(e) => println!("Failed to save {file}: {e}"),
                }
            }
            _ => {
                println!("unrecognized input, type help to see commands");
                continue;
            }
        }
    }
}
