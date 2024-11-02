use crate::precedence::{VdPrecedence, VdPrecedenceRange};

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSeparator {
    Base(VdBaseSeparator),
    Composite(VdCompositeSeparator),
}

impl VdSeparator {
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
}

impl VdBaseSeparator {
    pub const SPACE: Self = Self::Space;
    pub const COMMA: Self = Self::Comma;
    pub const SEMICOLON: Self = Self::Semicolon;
    pub const ADD: Self = Self::Add;
    pub const MUL: Self = Self::Mul;
    pub const DOT: Self = Self::Dot;

    fn left_precedence_range(self) -> VdPrecedenceRange {
        match self {
            VdBaseSeparator::Space => VdPrecedenceRange::NoLess(VdPrecedence::SPACE),
            VdBaseSeparator::Comma => VdPrecedenceRange::NoLess(VdPrecedence::COMMA),
            VdBaseSeparator::Semicolon => VdPrecedenceRange::NoLess(VdPrecedence::SEMICOLON),
            VdBaseSeparator::Add => VdPrecedenceRange::NoLess(VdPrecedence::ADD),
            VdBaseSeparator::Mul | VdBaseSeparator::Dot => {
                VdPrecedenceRange::NoLess(VdPrecedence::MUL)
            }
        }
    }

    fn right_precedence_range(self) -> VdPrecedenceRange {
        match self {
            VdBaseSeparator::Space => VdPrecedenceRange::Greater(VdPrecedence::SPACE),
            VdBaseSeparator::Comma => VdPrecedenceRange::Greater(VdPrecedence::COMMA),
            VdBaseSeparator::Semicolon => VdPrecedenceRange::Greater(VdPrecedence::SEMICOLON),
            VdBaseSeparator::Add => VdPrecedenceRange::Greater(VdPrecedence::ADD),
            VdBaseSeparator::Mul => VdPrecedenceRange::Greater(VdPrecedence::MUL),
            VdBaseSeparator::Dot => VdPrecedenceRange::Greater(VdPrecedence::MUL),
        }
    }
}

impl VdBaseSeparator {
    pub fn precedence(self) -> VdPrecedence {
        match self {
            VdBaseSeparator::Space => VdPrecedence::SPACE,
            VdBaseSeparator::Comma => todo!(),
            VdBaseSeparator::Semicolon => VdPrecedence::SEMICOLON,
            VdBaseSeparator::Add => VdPrecedence::ADD,
            VdBaseSeparator::Mul => VdPrecedence::MUL,
            VdBaseSeparator::Dot => VdPrecedence::MUL,
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
