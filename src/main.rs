pub mod node;
pub mod lexer;
pub mod parser;

use crate::node::*;
use crate::parser::parse;

pub fn build() -> Result<Vec<u8>, wat::Error> {
    let funcs = vec![
        function("mul", vec![ ("a", Kind::I32), ("b", Kind::I32) ], Kind::I32, parse("a * a")),

        function("add", vec![ ("a", Kind::I32), ("b", Kind::I32) ], Kind::I32, parse("mul ( a a ) + b")),

        function("main", vec![], Kind::I32, parse(r#"
            if add ( 40 2 ) == 1602
                1
                0
        "#) ),
    ];

    let wat_source = format!(
        "(module {} (export \"main\" (func $main)))",
        funcs.iter().map(|func| func.to_string()).collect::<Vec<String>>().join("")
    );

    return wat::parse_str(wat_source);
}

pub fn main() {
    match build() {
        Ok(bin) => std::fs::write("./res/main.wasm", &bin).unwrap(),
        Err(err) => eprint!("{}", err)
    }
}