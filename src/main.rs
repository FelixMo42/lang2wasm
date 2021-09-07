// Kind

pub struct Kind {

}

impl Kind {
    fn to_string(&self) -> String {
        return format!("i32");
    }
}

// Param

pub struct Param {
    name: String,
    kind: Kind,
}

impl Param {
    fn to_string(&self) -> String {
        return format!("(param ${} {})", self.name, self.kind.to_string())
    }
}

// Value

pub enum Value {
    Number(i32),
    Add(Box<Value>, Box<Value>),
    Get(String),
}

impl Value {
    fn to_string(&self) -> String {
        match self {
            Value::Number(value) => format!("(i32.const {})", value),
            Value::Add(a, b) => format!("(i32.add {} {})", a.to_string(), b.to_string()),
            Value::Get(name) => format!("local.get ${}", name),
        }
    }
}

// Function

pub struct Function {
    name: String,
    params: Vec<Param>,
    result: Kind,
    body: Value
}

impl Function {
    fn to_string(&self) -> String {
        return format!("(func ${} {} (result {}) {}) (export \"{}\" (func ${}))",
            self.name,
            self.params.iter().map(|param| param.to_string()).collect::<Vec<String>>().join(" "),
            self.result.to_string(),
            self.body.to_string(),
            self.name,
            self.name
        );
    }
}

// Main

pub fn build() -> Result<Vec<u8>, wat::Error> {
    let function = Function {
        name: "add".to_string(),
        params: vec![
            Param { name: "a".to_string(), kind: Kind {} },
            Param { name: "a".to_string(), kind: Kind {} },
        ],
        result: Kind {},
        body: Value::Add(
            Box::new(Value::Get("a".to_string())),
            Box::new(Value::Get("b".to_string()))
        ),
    };

    let wat_source = format!("(module {})", function.to_string());

    return wat::parse_str(wat_source);
}

pub fn main() {
    match build() {
        Ok(bin) => std::fs::write("./res/main.wasm", &bin).unwrap(),
        Err(err) => eprint!("{}", err)
    }
}