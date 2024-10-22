use crate::precedence::LnPrecedenceRange;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum LnBinaryOpr {
    Add,
}

impl LnBinaryOpr {
    pub fn fmt_str(self) -> &'static str {
        todo!()
    }

    pub fn left_precedence_range(self) -> LnPrecedenceRange {
        todo!()
    }

    pub fn right_precedence_range(self) -> LnPrecedenceRange {
        todo!()
    }
}
