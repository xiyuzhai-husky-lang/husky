use crate::precedence::VdPrecedenceRange;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum VdBinaryOpr {
    Add,
}

impl VdBinaryOpr {
    pub fn fmt_str(self) -> &'static str {
        match self {
            VdBinaryOpr::Add => "+",
            // Add more cases here as you implement more binary operators
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
            VdBinaryOpr::Add => "+",
            // Add more cases here as you implement more binary operators
        }
    }
}
