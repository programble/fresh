use fresh::generator::{Generator, Char, Hex, Base64};

pub fn password(gen_type: &str, length: usize) -> String {
    match gen_type {
        "char" => Char::default().generate(length),
        "hex" => Hex::default().generate(length),
        "base64" => Base64::default().generate(length),
        _ => panic!("invalid generator type {}", gen_type),
    }
}
