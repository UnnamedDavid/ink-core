// Parses parts[index] into T, printing a message on failure
pub fn parse_arg<T: std::str::FromStr>(
    positionals: &[&str],
    index: usize,
    name: &str,
) -> Option<T> {
    match positionals.get(index) {
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

// Parse command prompt from user and separate positionals and options
pub fn parse_command<'a>(
    parts: &'a [&'a str],
) -> Result<(Vec<&'a str>, Vec<PaintOptions>), String> {
    let mut positionals: Vec<&str> = Vec::new();
    let mut options: Vec<PaintOptions> = Vec::new();

    let mut iter = parts.iter();

    while let Some(&item) = iter.next() {
        // Parse options
        match item {
            "-c" | "--continued" => {
                options.push(PaintOptions::Continued);
            }
            "-r" | "--radius" => {
                let value = iter
                    // Advance iterator so value for options does not get misinterpretated as a positional
                    .next()
                    .ok_or_else(|| String::from("Expected a float value for radius"))?;

                let radius = value
                    .parse::<f32>()
                    .map_err(|_| String::from("Expected a float value for radius"))?;

                options.push(PaintOptions::Radius(radius));
            }
            "-o" | "--opacity" => {
                let value = iter
                    .next()
                    .ok_or_else(|| String::from("Expected a float value for opactiy"))?;

                let opacity = value
                    .parse::<f32>()
                    .map_err(|_| String::from("Expected a float value for opactiy"))?;

                options.push(PaintOptions::Opacity(opacity));
            }
            // Parse positionals
            _ => positionals.push(item),
        }
    }

    Ok((positionals, options))
}

// Available options for painting mode
pub enum PaintOptions {
    Continued,
    Radius(f32),
    Opacity(f32),
}
