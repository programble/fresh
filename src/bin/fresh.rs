extern crate fresh;

use fresh::generator::*;

fn main() {
    println!("{}", Char('a').generate(8));
    println!("{}", Hex.generate(8));
    println!("{}", Base64::default().generate(8));
}
