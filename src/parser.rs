use crate::node::*;
use crate::lexer::*;

pub fn parse(src: &str) -> Vec<Function> {
    let mut funcs = vec![];
    let lexer = &mut Lexer::new(src);

    while let Some(func) = eat_func(lexer) {
        funcs.push(func);
    }

    return funcs;
}

fn eat_name(toks: &mut Lexer) -> Option<String> {
    return toks.next_tokn().map(|name| name.to_string());
}

fn eat_number(toks: &mut Lexer) -> Option<i32> {
    toks.next_tokn_map_if(|tokn|
        if let Ok(num) = tokn.parse::<i32>() {
            Some(num)
        } else {
            None
        }
    )
}

fn eat_paran_list<T>(toks: &mut Lexer, func: impl Fn(&mut Lexer) -> Option<T>) -> Option<Vec<T>> {
    if toks.next_tokn_if(|tok| tok == "(").is_some() {
        let mut values = vec! [];

        while toks.next_tokn_if(|tok| tok == ")").is_none() {
            values.push(func(toks).unwrap());
        }

        return Some(values);
    }

    return None;
}

fn eat_value_part(toks: &mut Lexer) -> Option<Value> {
    if let Some(tuple) = eat_paran_list(toks, |toks| eat_value(toks)) {
        return Some(Value::Tuple(tuple));
    }

    if let Some(number) = eat_number(toks) {
        return Some(Value::Number(number));
    }

    if let Some(name) = eat_name(toks) {
        // If Statment
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
        if let Some(args) = eat_paran_list(toks, |toks| eat_value(toks)) {
            return Some(Value::Call(name, args));
        }

        // Get a variable
        return Some(Value::Get(name));
    }

    return None;
}

fn eat_value(toks: &mut Lexer) -> Option<Value> {
    if let Some(a) = eat_value_part(toks) {
        if let Some(op) = toks.next_tokn_if(|tok| tok == "*" || tok == "+" || tok == "-" || tok == "==") {
            if let Some(b) = eat_value_part(toks) {
                return Some(match op {
                    // Build the operators.
                    "*"  => Value::Mul( Box::new(a), Box::new(b) ),
                    "+"  => Value::Add( Box::new(a), Box::new(b) ),
                    "-"  => Value::Sub( Box::new(a), Box::new(b) ),
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

fn eat_kind(toks: &mut Lexer) -> Option<Kind> {
    if let Some(list) = eat_paran_list(toks, |toks| eat_kind(toks)) {
        return Some( Kind::Join(list) );
    }

    toks.next_tokn_map_if(|tokn|
        match tokn {
            "i32"  => Some(Kind::I32),
            "bool" => Some(Kind::Bool),

            _ => None,
        }
    )
}

fn eat_param(toks: &mut Lexer) -> Option<Param> {
    if let Some(name) = eat_name(toks) {
        if let Some(kind) = eat_kind(toks) {
            return Some(Param { name, kind });
        }
    }

    return None;
}

fn eat_func(toks: &mut Lexer) -> Option<Function> {
    if let Some(name) = eat_name(toks) {
        if let Some(params) = eat_paran_list(toks, |toks| eat_param(toks)) {
            if let Some(result) = eat_kind(toks) {
                if let Some(body) = eat_value(toks) {
                    return Some(Function {
                        name,
                        params,
                        result,
                        body,
                    })
                }
            }
        }
    }

    return None;
}