use crate::ParseArgError;

pub fn parse_arg<T: std::str::FromStr>(
    raw: Option<&&str>,
    name: &'static str,
) -> Result<T, ParseArgError> {
    let &arg = raw.ok_or(ParseArgError::MissingArgument(name))?;
    arg.parse::<T>().map_err(|_| ParseArgError::InvalidValue {
        name,
        found: arg.to_string(),
    })
}

// Parse command prompt from user and separate positionals and options
pub fn parse_command<'a>(
    parts: &[&'a str],
) -> Result<(Vec<&'a str>, Vec<PaintOptions>), ParseArgError> {
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
                // Advance iterator so value for options does not get misinterpretated as a positional
                let radius = parse_arg(iter.next(), "radius")?;
                options.push(PaintOptions::Radius(radius));
            }
            "-o" | "--opacity" => {
                let opacity = parse_arg(iter.next(), "opacity")?;
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
