#[derive(Clone)]
pub enum Value {
    Get(String),
    Call(String, Vec<Value>),

    // Consts
    Number(i32),

    // Basic wasm instructions
    Add(Box<Value>, Box<Value>),
    Sub(Box<Value>, Box<Value>),
    Mul(Box<Value>, Box<Value>),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Get(name) => format!("local.get ${}", name),
            Value::Call(name, args) => format!("(call ${} {})", name,
                args.iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),

            // Const
            Value::Number(value) => format!("(i32.const {})", value),

            // Basic wasm instructions
            Value::Add(a, b) => format!("(i32.add {} {})", a.to_string(), b.to_string()),
            Value::Sub(a, b) => format!("(i32.sub {} {})", a.to_string(), b.to_string()),
            Value::Mul(a, b) => format!("(i32.mul {} {})", a.to_string(), b.to_string()),
        }
    }
}