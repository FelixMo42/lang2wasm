pub mod node;

pub mod scope;
pub mod lexer;
pub mod parser;

use crate::parser::parse;

pub fn build() -> Result<Vec<u8>, wat::Error> {
    let funcs = parse(r#"
        fib ( a i32 b i32 i i32 ) i32
            if i == 0
                a + b
                fib (
                    ( b a + b )
                    i - 1
                )

        main (  ) i32
            fib ( 1 1 10 )
    "#);

    let wat_source = format!(
        r#"(module {} (export "main" (func $main)))"#,
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