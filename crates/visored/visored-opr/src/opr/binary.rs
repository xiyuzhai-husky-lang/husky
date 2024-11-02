use crate::precedence::VdPrecedenceRange;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum VdBaseBinaryOpr {
    Add,
    Eq,
}

impl VdBaseBinaryOpr {
    pub fn fmt_str(self) -> &'static str {
        match self {
            VdBaseBinaryOpr::Add => "+",
            VdBaseBinaryOpr::Eq => "=",
        }
    }

    pub fn left_precedence_range(self) -> VdPrecedenceRange {
        match self {
            VdBaseBinaryOpr::Add => VdPrecedenceRange::ADD_LEFT,
            VdBaseBinaryOpr::Eq => VdPrecedenceRange::EQ_LEFT,
        }
    }

    pub fn right_precedence_range(self) -> VdPrecedenceRange {
        todo!()
    }

    pub fn latex_code(self) -> &'static str {
        match self {
            VdBaseBinaryOpr::Add => "+",
            VdBaseBinaryOpr::Eq => "=",
        }
    }

    pub fn precedence(self) -> crate::precedence::VdPrecedence {
        match self {
            VdBaseBinaryOpr::Add => crate::precedence::VdPrecedence::ADD,
            VdBaseBinaryOpr::Eq => crate::precedence::VdPrecedence::EQ,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum VdCompositeBinaryOpr {
    Add,
    Eq,
}

impl VdCompositeBinaryOpr {
    pub fn fmt_str(self) -> &'static str {
        match self {
            VdCompositeBinaryOpr::Add => "+",
            VdCompositeBinaryOpr::Eq => "=",
        }
    }

    pub fn left_precedence_range(self) -> VdPrecedenceRange {
        todo!()
    }

    pub fn right_precedence_range(self) -> VdPrecedenceRange {
        todo!()
    }

    pub fn latex_code(&self) -> &str {
        match self {
            VdCompositeBinaryOpr::Add => "+",
            VdCompositeBinaryOpr::Eq => "=",
        }
    }

    pub fn precedence(self) -> crate::precedence::VdPrecedence {
        match self {
            VdCompositeBinaryOpr::Add => crate::precedence::VdPrecedence::ADD,
            VdCompositeBinaryOpr::Eq => crate::precedence::VdPrecedence::EQ,
        }
    }
}
