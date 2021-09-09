use crate::node::*;

pub struct Param<'a> {
    pub name: &'a str,
    pub kind: Kind,
}

impl std::fmt::Display for Param<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", self.kind)
    }
}

//

pub struct Function<'a> {
    pub name: &'a str,
    pub params: Vec<Param<'a>>,
    pub result: Kind,
    pub body: Value<'a>
}

impl std::fmt::Display for Function<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "(func ${} (param {}) (result {}) {})",
            self.name,
            self.params.iter().map(|param| param.to_string()).collect::<Vec<String>>().join(" "),
            self.result,
            self.body,
        );
    }
}