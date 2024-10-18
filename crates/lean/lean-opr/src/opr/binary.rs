use crate::precedence::LeanPrecedenceRange;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum LeanBinaryOpr {}

impl LeanBinaryOpr {
    pub fn fmt_str(self) -> &'static str {
        todo!()
    }

    pub fn left_precedence_range(self) -> LeanPrecedenceRange {
        todo!()
    }

    pub fn right_precedence_range(self) -> LeanPrecedenceRange {
        todo!()
    }
}
