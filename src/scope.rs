use crate::node::*;

#[derive(Clone, Copy)]
pub enum Variable {
    Local(usize)
}

impl Variable {
    pub fn get_id(&self) -> usize {
        match self {
            Variable::Local(id) => id.clone()
        }
    }
}

#[derive(Clone)]
pub struct Scope<'a> {
    parent: Option<Box<Scope<'a>>>,
    variables: Vec<(&'a str, Variable)>
}

impl<'a, 'b> Scope<'a> {
    pub fn new(variables: Vec<(&'a str, Variable)>) -> Scope<'a> {
        return Scope {
            variables,
            parent: None,
        }
    }

    pub fn add_function_params(&self, params: &Vec<Param<'a>>) -> Scope<'a> {
        let variables = params
            .iter()
            .enumerate()
            .map(|(i, param)| (param.name, Variable::Local(i)))
            .collect::<Vec<(&'a str, Variable)>>();

        return Scope { variables, parent: Some(Box::new(self.clone())) };
    }

    pub fn get(&self, var: &'a str) -> Option<&Variable> {
        for (name, variable) in &self.variables {
            if &var == name {
                return Some(&variable)
            }
        }

        if let Some(parent) = &self.parent {
            return parent.get(var);
        }

        return None;
    }
}