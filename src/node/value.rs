pub enum Value {
    Number(i32),
    Add(Box<Value>, Box<Value>),
    Get(String),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Number(value) => format!("(i32.const {})", value),
            Value::Add(a, b) => format!("(i32.add {} {})", a.to_string(), b.to_string()),
            Value::Get(name) => format!("local.get ${}", name),
        }
    }
}