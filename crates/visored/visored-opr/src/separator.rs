use crate::precedence::VdPrecedence;

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSeparator {
    Base(VdBaseSeparator),
    Composite(VdCompositeSeparator),
}

impl VdSeparator {
    pub const SPACE: Self = VdSeparator::Base(VdBaseSeparator::Space);
}

impl VdSeparator {
    pub fn precedence(self) -> VdPrecedence {
        match self {
            VdSeparator::Base(sep) => sep.precedence(),
            VdSeparator::Composite(sep) => sep.precedence(),
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
    pub fn precedence(self) -> VdPrecedence {
        match self {
            VdBaseSeparator::Space => VdPrecedence::SPACE,
            VdBaseSeparator::Comma => todo!(),
            VdBaseSeparator::Semicolon => todo!(),
            VdBaseSeparator::Add => todo!(),
            VdBaseSeparator::Mul => todo!(),
            VdBaseSeparator::Dot => todo!(),
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
pub enum VdCompositeSeparator {
    Call,
}

impl VdCompositeSeparator {
    pub fn precedence(self) -> VdPrecedence {
        todo!()
    }

    pub fn latex_code(self) -> &'static str {
        todo!()
    }
}
