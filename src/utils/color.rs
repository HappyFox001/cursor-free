pub fn green(text: &str) -> String {
    format!("\x1B[32m{}\x1B[0m", text)
}

pub fn red(text: &str) -> String {
    format!("\x1B[31m{}\x1B[0m", text)
}

pub fn yellow(text: &str) -> String {
    format!("\x1B[33m{}\x1B[0m", text)
}

pub fn blue(text: &str) -> String {
    format!("\x1B[34m{}\x1B[0m", text)
}
