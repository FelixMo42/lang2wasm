pub mod node;

use crate::node::*;
use std::str::Split;
use std::iter::Peekable;

fn parse(src: &str) -> Value {
    return eat_value(&mut src.split(" ").peekable())
}

fn eat_name(toks: &mut Peekable<Split<&str>>) -> String {
    return toks.next().unwrap().to_string()
}

fn eat_value(toks: &mut Peekable<Split<&str>>) -> Value {
    if toks.next_if(|tok| tok == &"(").is_some() {
        let name = eat_name(toks);

        let mut args = vec! [];

        while toks.next_if(|tok| tok == &")").is_none() {
            args.push(eat_value(toks));
        }

        return match name.as_str() {
            "*" => Value::Mul( Box::new(args[0].clone()), Box::new(args[1].clone()) ),
            "+" => Value::Add( Box::new(args[0].clone()), Box::new(args[1].clone()) ),
            "-" => Value::Sub( Box::new(args[0].clone()), Box::new(args[1].clone()) ),

            _ => Value::Call(name, args)
        }
    }

    return Value::Get(eat_name(toks));
}

pub fn build() -> Result<Vec<u8>, wat::Error> {
    let funcs = vec![
        function("mul", vec![ ("a", Kind{}), ("b", Kind {}) ], Kind {}, parse("( * a a )")),

        function("add", vec![ ("a", Kind{}), ("b", Kind {}) ], Kind {}, parse("( + ( mul a a ) b )")),

        function("main", vec![], Kind {}, Value::Call("add".to_string(), vec![Value::Number(40), Value::Number(1)])),
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