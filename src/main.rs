use std::io::{self, Write};

fn main() {
    print!("==> What is the active card?: ");
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!(
        "**Current active card ({}), Burning Bonze!**",
        input.trim_end()
    );
}
