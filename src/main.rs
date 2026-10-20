use ink_core::Canvas;
use std::env;
use std::io::{self};

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

    // Provides user the length of stroke of their requested index on the Canvas
    loop {
        println!("Press ENTER to finish");
        println!(
            "Select which stroke on the canvas you want to see length [1-{}]: ",
            this_canvas.strokes.len()
        );
        let mut response = String::new();
        match io::stdin().read_line(&mut response) {
            Ok(_) => (),
            Err(e) => {
                println!("failed to process your input: {e}");
                continue;
            }
        }
        // Exit program if the user presses ENTER
        if response.trim().is_empty() {
            break;
        };

        let index = match response.trim().parse::<usize>() {
            // Handle when user did not enter an integer and out of range indexes
            Ok(result) if (1..=this_canvas.strokes.len()).contains(&result) => result - 1,
            _ => {
                println!("please enter an integer [1-{}]", this_canvas.strokes.len());
                continue;
            }
        };
        let the_stroke = &this_canvas.strokes[index];
        println!("{}", the_stroke.length())
    }
}
