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
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdCompositeSeparator {
    Call,
}
impl VdCompositeSeparator {
    fn precedence(self) -> VdPrecedence {
        todo!()
    }
}
