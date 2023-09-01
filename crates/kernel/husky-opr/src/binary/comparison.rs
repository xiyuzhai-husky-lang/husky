#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryComparisonOpr {
    Eq,
    Neq,
    Geq,
    Greater,
    Leq,
    Less,
}

impl BinaryComparisonOpr {
    pub fn husky_code(self) -> &'static str {
        match self {
            BinaryComparisonOpr::Eq => "==",
            BinaryComparisonOpr::Neq => "!=",
            BinaryComparisonOpr::Greater => ">",
            BinaryComparisonOpr::Geq => ">=",
            BinaryComparisonOpr::Less => "<",
            BinaryComparisonOpr::Leq => "<=",
        }
    }
    pub fn spaced_husky_code(self) -> &'static str {
        match self {
            BinaryComparisonOpr::Greater => " > ",
            BinaryComparisonOpr::Geq => " >= ",
            BinaryComparisonOpr::Less => " < ",
            BinaryComparisonOpr::Leq => " <= ",
            BinaryComparisonOpr::Eq => " == ",
            BinaryComparisonOpr::Neq => " != ",
        }
    }

    pub fn rust_trait_name(self) -> &'static str {
        match self {
            BinaryComparisonOpr::Eq => todo!(),
            BinaryComparisonOpr::Neq => "ne",
            BinaryComparisonOpr::Geq => todo!(),
            BinaryComparisonOpr::Greater => todo!(),
            BinaryComparisonOpr::Leq => todo!(),
            BinaryComparisonOpr::Less => todo!(),
        }
    }
}
