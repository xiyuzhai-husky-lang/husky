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
        todo!()
    }

    pub fn right_precedence_range(self) -> VdPrecedenceRange {
        todo!()
    }

    pub fn latex_code(&self) -> &str {
        match self {
            VdBaseBinaryOpr::Add => "+",
            VdBaseBinaryOpr::Eq => "=",
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
}
