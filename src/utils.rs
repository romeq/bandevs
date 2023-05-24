// returns colorized text
// possible colors:
// - red
// - green
// - yellow
// - blue
pub fn colorize(text: &str, color: &str) -> String {
    match color {
        "red" => format!("\x1b[31m{}\x1b[0m", text),
        "green" => format!("\x1b[32m{}\x1b[0m", text),
        "yellow" => format!("\x1b[33m{}\x1b[0m", text),
        "blue" => format!("\x1b[34m{}\x1b[0m", text),
        _ => format!("{}", text),
    }
}
