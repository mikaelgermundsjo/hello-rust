use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let s = "Hello fellow Rustaceans!";
    let message = String::from(s);
    let chars = message.chars();
    let width = chars.count();
    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
