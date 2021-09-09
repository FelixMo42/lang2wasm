use crate::node::*;
use crate::lexer::*;
use crate::scope::Scope;

pub fn parse(src: &str) -> Vec<Function> {
    let mut funcs = vec![];
    let lexer = &mut Lexer::new(src);

    let global_scope = &Scope::new(vec! []);

    while let Some(func) = eat_func(lexer, global_scope) {
        funcs.push(func);
    }

    return funcs;
}

fn eat_name<'a>(toks: &mut Lexer<'a>, _scope: &Scope) -> Option<&'a str> {
    return toks.next_tokn();
}

fn eat_number(toks: &mut Lexer, _scope: &Scope) -> Option<i32> {
    toks.next_tokn_map_if(|tokn|
        if let Ok(num) = tokn.parse::<i32>() {
            Some(num)
        } else {
            None
        }
    )
}

fn eat_paran_list<'a, T>(toks: &mut Lexer<'a>, func: impl Fn(&mut Lexer<'a>) -> Option<T>) -> Option<Vec<T>> {
    if toks.next_tokn_if(|tok| tok == "(").is_some() {
        let mut values = vec! [];

        while toks.next_tokn_if(|tok| tok == ")").is_none() {
            values.push(func(toks).unwrap());
        }

        return Some(values);
    }

    return None;
}

fn eat_value_part<'a>(toks: &mut Lexer<'a>, scope: &Scope<'a>) -> Option<Value<'a>> {
    if let Some(tuple) = eat_paran_list(toks, |toks| eat_value(toks, scope)) {
        return Some(Value::Tuple(tuple));
    }

    if let Some(number) = eat_number(toks, scope) {
        return Some(Value::Number(number));
    }

    if let Some(name) = eat_name(toks, scope) {
        // If Statment
        if name == "if" {
            if let Some(cond) = eat_value(toks, scope) {
                if let Some(a) = eat_value(toks, scope) {
                    if let Some(b) = eat_value(toks, scope) {
                        return Some(Value::If(Box::new(cond), Box::new(a), Box::new(b)));
                    }
                }
            }
        }

        // Function
        if let Some(args) = eat_paran_list(toks, |toks| eat_value(toks, scope)) {
            return Some(Value::Call(name, args));
        }

        // Get a variable
        if let Some(variable) = scope.get(name) {
            return Some(Value::LocalVariable(variable.get_id()));
        } else {
            println!("cant find varriable {}", name);
        }
    }

    return None;
}

fn eat_value<'a>(toks: &mut Lexer<'a>, scope: &Scope<'a>) -> Option<Value<'a>> {
    if let Some(a) = eat_value_part(toks, scope) {
        if let Some(op) = toks.next_tokn_if(|tok| tok == "*" || tok == "+" || tok == "-" || tok == "==") {
            if let Some(b) = eat_value_part(toks, scope) {
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

fn eat_kind(toks: &mut Lexer, scope: &Scope) -> Option<Kind> {
    if let Some(list) = eat_paran_list(toks, |toks| eat_kind(toks, scope)) {
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

fn eat_param<'a>(toks: &mut Lexer<'a>, scope: &Scope<'a>) -> Option<Param<'a>> {
    if let Some(name) = eat_name(toks, scope) {
        if let Some(kind) = eat_kind(toks, scope) {
            return Some(Param { name, kind });
        }
    }

    return None;
}

fn eat_func<'a>(toks: &mut Lexer<'a>, scope: &Scope<'a>) -> Option<Function<'a>> {
    if let Some(name) = eat_name(toks, scope) {
        if let Some(params) = eat_paran_list(toks, |toks| eat_param(toks, scope)) {
            if let Some(result) = eat_kind(toks, scope) {
                let func_scope = scope.add_function_params(&params);
                if let Some(body) = eat_value(toks, &func_scope) {
                    return Some(Function {
                        name,
                        params,
                        result,
                        body,
                    });
                }
            }
        }
    }

    return None;
}