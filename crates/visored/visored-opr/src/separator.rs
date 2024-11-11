use crate::precedence::{VdPrecedence, VdPrecedenceRange};

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSeparator {
    Base(VdBaseSeparator),
    Composite(VdCompositeSeparator),
}

impl VdSeparator {
    pub const EQ: Self = VdSeparator::Base(VdBaseSeparator::Eq);
    pub const NE: Self = VdSeparator::Base(VdBaseSeparator::Ne);
    pub const SUBSET: Self = VdSeparator::Base(VdBaseSeparator::Subset);
    pub const SUPSET: Self = VdSeparator::Base(VdBaseSeparator::Supset);
    pub const SUBSETEQ: Self = VdSeparator::Base(VdBaseSeparator::Subseteq);
    pub const SUPSETEQ: Self = VdSeparator::Base(VdBaseSeparator::Supseteq);
    pub const SUBSETEQQ: Self = VdSeparator::Base(VdBaseSeparator::Subseteqq);
    pub const SUPSETEQQ: Self = VdSeparator::Base(VdBaseSeparator::Supseteqq);
    pub const SUBSETNEQ: Self = VdSeparator::Base(VdBaseSeparator::Subsetneq);
    pub const SUPSETNEQ: Self = VdSeparator::Base(VdBaseSeparator::Supsetneq);
    pub const IN: Self = VdSeparator::Base(VdBaseSeparator::In);
    pub const SPACE: Self = VdSeparator::Base(VdBaseSeparator::Space);
    pub const COMMA: Self = VdSeparator::Base(VdBaseSeparator::Comma);
}

impl VdSeparator {
    pub fn precedence(self) -> VdPrecedence {
        match self {
            VdSeparator::Base(sep) => sep.precedence(),
            VdSeparator::Composite(sep) => sep.precedence(),
        }
    }

    pub fn left_precedence_range(self) -> VdPrecedenceRange {
        match self {
            VdSeparator::Base(slf) => slf.left_precedence_range(),
            VdSeparator::Composite(slf) => slf.left_precedence_range(),
        }
    }

    pub fn right_precedence_range(self) -> VdPrecedenceRange {
        match self {
            VdSeparator::Base(slf) => slf.right_precedence_range(),
            VdSeparator::Composite(slf) => slf.right_precedence_range(),
        }
    }

    pub fn latex_code(self) -> &'static str {
        match self {
            VdSeparator::Base(slf) => slf.latex_code(),
            VdSeparator::Composite(slf) => slf.latex_code(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdBaseSeparator {
    Space,
    Comma,
    Semicolon,
    Add,
    Mul,
    Dot,
    Eq,
    Subset,
    Supset,
    Subseteq,
    Supseteq,
    Subseteqq,
    Supseteqq,
    Subsetneq,
    Supsetneq,
    In,
    Notin,
    Times,
    Otimes,
    Ne,
}

impl VdBaseSeparator {
    pub const SPACE: Self = Self::Space;
    pub const COMMA: Self = Self::Comma;
    pub const SEMICOLON: Self = Self::Semicolon;
    pub const ADD: Self = Self::Add;
    pub const MUL: Self = Self::Mul;
    pub const DOT: Self = Self::Dot;
    pub const EQ: Self = Self::Eq;

    fn left_precedence_range(self) -> VdPrecedenceRange {
        match self {
            VdBaseSeparator::Space => VdPrecedenceRange::NoLess(VdPrecedence::SPACE),
            VdBaseSeparator::Comma => VdPrecedenceRange::NoLess(VdPrecedence::COMMA),
            VdBaseSeparator::Semicolon => VdPrecedenceRange::NoLess(VdPrecedence::SEMICOLON),
            VdBaseSeparator::Add => VdPrecedenceRange::NoLess(VdPrecedence::ADD_SUB),
            VdBaseSeparator::Mul | VdBaseSeparator::Dot => {
                VdPrecedenceRange::NoLess(VdPrecedence::MUL_DIV)
            }
            VdBaseSeparator::Eq
            | VdBaseSeparator::Ne
            | VdBaseSeparator::Subset
            | VdBaseSeparator::Supset
            | VdBaseSeparator::Subseteq
            | VdBaseSeparator::Supseteq
            | VdBaseSeparator::Subseteqq
            | VdBaseSeparator::Supseteqq
            | VdBaseSeparator::Subsetneq
            | VdBaseSeparator::Supsetneq
            | VdBaseSeparator::In
            | VdBaseSeparator::Notin => VdPrecedenceRange::NoLess(VdPrecedence::RELATION),
            VdBaseSeparator::Times => todo!(),
            VdBaseSeparator::Otimes => todo!(),
        }
    }

    fn right_precedence_range(self) -> VdPrecedenceRange {
        match self {
            VdBaseSeparator::Space => VdPrecedenceRange::Greater(VdPrecedence::SPACE),
            VdBaseSeparator::Comma => VdPrecedenceRange::Greater(VdPrecedence::COMMA),
            VdBaseSeparator::Semicolon => VdPrecedenceRange::Greater(VdPrecedence::SEMICOLON),
            VdBaseSeparator::Add => VdPrecedenceRange::Greater(VdPrecedence::ADD_SUB),
            VdBaseSeparator::Mul => VdPrecedenceRange::Greater(VdPrecedence::MUL_DIV),
            VdBaseSeparator::Dot => VdPrecedenceRange::Greater(VdPrecedence::MUL_DIV),
            VdBaseSeparator::Eq => VdPrecedenceRange::Greater(VdPrecedence::RELATION),
            VdBaseSeparator::Subset => todo!(),
            VdBaseSeparator::Supset => todo!(),
            VdBaseSeparator::Subseteq => todo!(),
            VdBaseSeparator::Supseteq => todo!(),
            VdBaseSeparator::Subseteqq => todo!(),
            VdBaseSeparator::Supseteqq => todo!(),
            VdBaseSeparator::Subsetneq => todo!(),
            VdBaseSeparator::Supsetneq => todo!(),
            VdBaseSeparator::In => todo!(),
            VdBaseSeparator::Notin => todo!(),
            VdBaseSeparator::Times => todo!(),
            VdBaseSeparator::Otimes => todo!(),
            VdBaseSeparator::Ne => todo!(),
        }
    }
}

impl VdBaseSeparator {
    pub fn precedence(self) -> VdPrecedence {
        match self {
            VdBaseSeparator::Space => VdPrecedence::SPACE,
            VdBaseSeparator::Comma => todo!(),
            VdBaseSeparator::Semicolon => VdPrecedence::SEMICOLON,
            VdBaseSeparator::Add => VdPrecedence::ADD_SUB,
            VdBaseSeparator::Mul | VdBaseSeparator::Times | VdBaseSeparator::Otimes => {
                VdPrecedence::MUL_DIV
            }
            VdBaseSeparator::Dot => VdPrecedence::MUL_DIV,
            VdBaseSeparator::Eq
            | VdBaseSeparator::Subset
            | VdBaseSeparator::Supset
            | VdBaseSeparator::Subseteq
            | VdBaseSeparator::Supseteq
            | VdBaseSeparator::Subseteqq
            | VdBaseSeparator::Supseteqq
            | VdBaseSeparator::Subsetneq
            | VdBaseSeparator::Supsetneq
            | VdBaseSeparator::In
            | VdBaseSeparator::Notin
            | VdBaseSeparator::Ne => VdPrecedence::RELATION,
        }
    }

    pub fn latex_code(self) -> &'static str {
        match self {
            VdBaseSeparator::Space => "␣",
            VdBaseSeparator::Comma => ",",
            VdBaseSeparator::Semicolon => ";",
            VdBaseSeparator::Add => "+",
            VdBaseSeparator::Mul => "\\times",
            VdBaseSeparator::Dot => "\\cdot",
            VdBaseSeparator::Eq => "=",
            VdBaseSeparator::Subset => "\\subseteq",
            VdBaseSeparator::Supset => "\\supseteq",
            VdBaseSeparator::Subseteq => "\\subseteqq",
            VdBaseSeparator::Supseteq => "\\supseteqq",
            VdBaseSeparator::Subseteqq => "\\subseteqqq",
            VdBaseSeparator::Supseteqq => "\\supseteqqq",
            VdBaseSeparator::Subsetneq => "\\subsetneq",
            VdBaseSeparator::Supsetneq => "\\supsetneq",
            VdBaseSeparator::In => "\\in",
            VdBaseSeparator::Notin => "\\notin",
            VdBaseSeparator::Times => "\\times",
            VdBaseSeparator::Otimes => "\\otimes",
            VdBaseSeparator::Ne => "\\neq",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdCompositeSeparator {}

impl VdCompositeSeparator {
    pub fn precedence(self) -> VdPrecedence {
        todo!()
    }

    pub fn latex_code(self) -> &'static str {
        todo!()
    }

    fn left_precedence_range(self) -> VdPrecedenceRange {
        match self {}
    }

    fn right_precedence_range(self) -> VdPrecedenceRange {
        match self {}
    }
}
