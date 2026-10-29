// Parses parts[index] into T, printing a message on failure
pub fn parse_arg<T: std::str::FromStr>(parts: &[&str], index: usize, name: &str) -> Option<T> {
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
