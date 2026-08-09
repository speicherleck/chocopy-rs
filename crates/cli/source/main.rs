use std::env;

use owo_colors::OwoColorize;

fn main() {
    println!("{} from {}!", "Hello".red().bold(), "cli".red().bold());

    let arguments: Vec<String> = env::args().collect();

    dbg!(arguments);
}
