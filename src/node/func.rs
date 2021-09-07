use crate::node::*;

pub struct Param {
    pub name: String,
    pub kind: Kind,
}

impl Param {
    pub fn to_string(&self) -> String {
        return format!("(param ${} {})", self.name, self.kind.to_string())
    }
}

//

pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub result: Kind,
    pub body: Value
}

impl Function {
    pub fn to_string(&self) -> String {
        return format!("(func ${} {} (result {}) {})",
            self.name,
            self.params.iter().map(|param| param.to_string()).collect::<Vec<String>>().join(" "),
            self.result.to_string(),
            self.body.to_string(),
        );
    }
}

pub fn function(name: &str, params: Vec<(&str, Kind)>, result: Kind, body: Value) -> Function {
    return Function {
        name: name.to_string(),
        params: params.into_iter().map(|(name, kind)| Param { name: name.to_string(), kind }).collect(),
        result,
        body,
    }
}