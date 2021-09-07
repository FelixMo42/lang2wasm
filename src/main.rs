pub mod node;

use crate::node::*;

pub fn build() -> Result<Vec<u8>, wat::Error> {
    let main = function("main", vec![], Kind {}, Value::Number(42));

    let wat_source = format!("(module {} (export \"{}\" (func ${})))", main.to_string(), main.name, main.name);

    return wat::parse_str(wat_source);
}

pub fn main() {
    match build() {
        Ok(bin) => std::fs::write("./res/main.wasm", &bin).unwrap(),
        Err(err) => eprint!("{}", err)
    }
}