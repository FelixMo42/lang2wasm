pub enum Kind {
    I32,
    Bool
}

const _POINTER_TYPE: &str = "i32";

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Kind::I32 => write!(f, "i32"),
            Kind::Bool => write!(f, "i32"),
            /*Kind::I64 => "i64".to_string(),
            Kind::F32 => "f32".to_string(),
            Kind::F64 => "f64".to_string(),
            Kind::Struct => POINTER_TYPE.to_string(),*/
        }
    }
}