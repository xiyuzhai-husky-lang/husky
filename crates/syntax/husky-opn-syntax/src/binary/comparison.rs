#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryComparisonPunctuation {
    Eq,
    Neq,
    Geq,
    Greater,
    Leq,
    Less,
}

impl BinaryComparisonPunctuation {
    pub fn husky_code(self) -> &'static str {
        match self {
            BinaryComparisonPunctuation::Eq => "==",
            BinaryComparisonPunctuation::Neq => "!=",
            BinaryComparisonPunctuation::Greater => ">",
            BinaryComparisonPunctuation::Geq => ">=",
            BinaryComparisonPunctuation::Less => "<",
            BinaryComparisonPunctuation::Leq => "<=",
        }
    }
    pub fn spaced_husky_code(self) -> &'static str {
        match self {
            BinaryComparisonPunctuation::Greater => " > ",
            BinaryComparisonPunctuation::Geq => " >= ",
            BinaryComparisonPunctuation::Less => " < ",
            BinaryComparisonPunctuation::Leq => " <= ",
            BinaryComparisonPunctuation::Eq => " == ",
            BinaryComparisonPunctuation::Neq => " != ",
        }
    }

    pub fn rust_trait_name(self) -> &'static str {
        match self {
            BinaryComparisonPunctuation::Eq => todo!(),
            BinaryComparisonPunctuation::Neq => "ne",
            BinaryComparisonPunctuation::Geq => todo!(),
            BinaryComparisonPunctuation::Greater => todo!(),
            BinaryComparisonPunctuation::Leq => todo!(),
            BinaryComparisonPunctuation::Less => todo!(),
        }
    }
}
