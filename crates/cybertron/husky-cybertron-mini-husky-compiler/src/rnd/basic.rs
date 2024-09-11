use husky_rng_utils::XRng;

pub enum Type {
    Bool,
    Int,
    Float,
}

impl Type {
    pub fn repr(self) -> &'static str {
        match self {
            Type::Bool => "Bool",
            Type::Int => "Int",
            Type::Float => "Float",
        }
    }
}

struct BasicCodeGenerator {
    rng: XRng,
}
