#[derive(Clone)]
pub enum Value {
    Get(String),
    Call(String, Vec<Value>),
    If(Box<Value>, Box<Value>, Box<Value>),

    // Consts
    Number(i32),

    // Operators
    Add(Box<Value>, Box<Value>),
    Sub(Box<Value>, Box<Value>),
    Mul(Box<Value>, Box<Value>),
    Eq(Box<Value>, Box<Value>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Get(name) => write!(f, "local.get ${}", name),
            Value::Call(name, args) => write!(f, "(call ${} {})", name,
                args.iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Value::If(cond, a, b) => write!(f, "(if (result i32) {} (then {}) (else {}))", cond, a, b),

            // Const
            Value::Number(value) => write!(f, "(i32.const {})", value),

            // Basic wasm instructions
            Value::Add(a, b) => write!(f, "(i32.add {} {})", a.to_string(), b.to_string()),
            Value::Sub(a, b) => write!(f, "(i32.sub {} {})", a.to_string(), b.to_string()),
            Value::Mul(a, b) => write!(f, "(i32.mul {} {})", a.to_string(), b.to_string()),
            Value::Eq(a, b) => write!(f, "(i32.eq {} {})", a.to_string(), b.to_string()),
        }
    }
}