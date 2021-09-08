pub mod node;

use crate::node::*;
use std::str::Split;
use std::iter::Peekable;

fn parse(src: &str) -> Value {
    return eat_value(&mut src.split(" ").peekable()).unwrap()
}

fn eat_name(toks: &mut Peekable<Split<&str>>) -> Option<String> {
    return toks.next().map(|name| name.to_string());
}

fn eat_number(toks: &mut Peekable<Split<&str>>) -> Option<i32> {
    if let Some(num_str) = toks.peek() {
        if let Ok(num) = num_str.parse::<i32>() {
            toks.next();
            return Some(num);
        }
    }
    return None;
}

fn eat_args(toks: &mut Peekable<Split<&str>>) -> Option<Vec<Value>> {
    if toks.next_if(|tok| tok == &"(").is_some() {
        let mut values = vec! [];

        while toks.next_if(|tok| tok == &")").is_none() {
            values.push(eat_value(toks).unwrap());
        }

        return Some(values);
    }

    return None;
}

fn eat_value_part(toks: &mut Peekable<Split<&str>>) -> Option<Value> {
    if let Some(number) = eat_number(toks) {
        return Some(Value::Number(number));
    }

    if let Some(name) = eat_name(toks) {
        if name == "if" {
            if let Some(cond) = eat_value(toks) {
                if let Some(a) = eat_value(toks) {
                    if let Some(b) = eat_value(toks) {
                        return Some(Value::If(Box::new(cond), Box::new(a), Box::new(b)));
                    }
                }
            }
        }

        // Function
        if let Some(args) = eat_args(toks) {
            return Some(Value::Call(name, args));
        }

        // Get a variable
        return Some(Value::Get(name));
    }

    return None;
}

fn eat_value(toks: &mut Peekable<Split<&str>>) -> Option<Value> {
    if let Some(a) = eat_value_part(toks) {
        if let Some(op) = toks.next_if(|tok| tok == &"*" || tok == &"+" || tok == &"-" || tok == &"==") {
            if let Some(b) = eat_value_part(toks) {
                return Some(match op {
                    // Build the operators.
                    "*" => Value::Mul( Box::new(a), Box::new(b) ),
                    "+" => Value::Add( Box::new(a), Box::new(b) ),
                    "-" => Value::Sub( Box::new(a), Box::new(b) ),
                    "==" => Value::Eq( Box::new(a), Box::new(b) ),
                    

                    // This operator should allwais be a know one.
                    _ => unreachable!() 
                });
            }
        } else {
            return Some(a);
        }
    }

    return None;
}

pub fn build() -> Result<Vec<u8>, wat::Error> {
    let funcs = vec![
        function("mul", vec![ ("a", Kind::I32), ("b", Kind::I32) ], Kind::I32, parse("a * a")),

        function("add", vec![ ("a", Kind::I32), ("b", Kind::I32) ], Kind::I32, parse("mul ( a a ) + b")),

        function("main", vec![], Kind::I32, parse("if add ( 40 2 ) == 1602 1 0") ),
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