use ink_core::Canvas;
use std::env;
use std::io::{self, Write};

fn main() {
    // Reads a file path if the user provides one e.g. `cargo run -- custom.json`,
    // otherwise defaults to "canvas.json" in the root directory.
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("canvas.json"));

    let this_canvas = Canvas::from_json_file(&path);

    let bbox_each = this_canvas.each_bounding_box();
    let bbox_total = this_canvas.overall_bounding_box();

    println!(
        "For each stroke on the canvas, the bounding box is {bbox_each:#?}, for the total canvas, the bounding box is {bbox_total:#?}"
    );

    // Wait until user to press enter to exit program so user can see the output
    println!("\nPress Enter to exit...");
    let _ = io::stdout().flush();
    let mut pause = String::new();
    let _ = io::stdin().read_line(&mut pause);
}
