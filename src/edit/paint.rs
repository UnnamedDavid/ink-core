use crate::parser::PaintOptions;

const DEFAULT_RADIUS: f32 = 5.0;
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
    fn print_help() {
        println!("Paint mode lets you make changes on the canvas.");
        println!("Available Commands:");
        println!("  tool <pen | highligher | eraser>   --Change the brush you are using");
        println!("  draw <x> <y>                       --Draw point");
        println!("  save                               --Save your changes");
        println!("  quit                               --exit paint mode");
        println!("");
        println!("Available Options:");
        println!(
            "  -c --continued                     --Indicate this point is part of the stroke from previous point"
        );
        println!("  -r VALUE --radius VALUE             --Set pen radius of current brush");
        println!(
            "  -o VALUE --opacity VALUE             --Set opacity of current brush, available only to highligher"
        )
    }

    // paint mode interactive CLI
    fn execute(&self, positionals: &[&str], options: &[&PaintOptions]) -> PaintStatus {
        match positionals.first().copied() {
            Some("tool") => match positionals.get(1).copied() {
                Some(pen) => {
                    todo!();
                }
                _ => {
                    println!("missing argument <pen | highligher | eraser> ");
                    return PaintStatus::Continue;
                }
            },
            _ => {
                println!("unrecognized input, type help to see commands");
                return PaintStatus::Continue;
            }
        }
    }
}
