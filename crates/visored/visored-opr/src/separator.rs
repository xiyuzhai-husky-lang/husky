use crate::precedence::VdPrecedence;

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
