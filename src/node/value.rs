#[derive(Clone)]
pub enum Value<'a> {
    LocalVariable(usize),

    Call(&'a str, Vec<Value<'a>>),
    If(Box<Value<'a>>, Box<Value<'a>>, Box<Value<'a>>),

    Tuple(Vec<Value<'a>>),

    // Consts
    Number(i32),

    // Operators
    Add(Box<Value<'a>>, Box<Value<'a>>),
    Sub(Box<Value<'a>>, Box<Value<'a>>),
    Mul(Box<Value<'a>>, Box<Value<'a>>),
    Eq(Box<Value<'a>>, Box<Value<'a>>),
}

impl std::fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::LocalVariable(id) => write!(f, "local.get {}", id),
            Value::Call(name, args) => write!(f, "(call ${} {})", name,
                args.iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Value::If(cond, a, b) => write!(f, "(if (result i32) {} (then {}) (else {}))", cond, a, b),

            Value::Tuple(values) => write!(f, "{}",
                values.iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),

            // Const
            Value::Number(value) => write!(f, "(i32.const {})", value),

            // Basic wasm instructions
            Value::Add(a, b) => write!(f, "(i32.add {} {})", a, b),
            Value::Sub(a, b) => write!(f, "(i32.sub {} {})", a, b),
            Value::Mul(a, b) => write!(f, "(i32.mul {} {})", a, b),
            Value::Eq(a, b) => write!(f, "(i32.eq {} {})", a, b),
        }
    }
}