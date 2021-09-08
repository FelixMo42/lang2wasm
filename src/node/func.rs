use crate::node::*;

pub struct Param {
    pub name: String,
    pub kind: Kind,
}

impl std::fmt::Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "(param ${} {})", self.name, self.kind)
    }
}

//

pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub result: Kind,
    pub body: Value
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "(func ${} {} (result {}) {})",
            self.name,
            self.params.iter().map(|param| param.to_string()).collect::<Vec<String>>().join(" "),
            self.result,
            self.body,
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