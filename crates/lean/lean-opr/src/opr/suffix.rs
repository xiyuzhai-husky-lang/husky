use crate::precedence::LeanPrecedenceRange;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum LeanSuffixOpr {}

impl LeanSuffixOpr {
    pub fn fmt_str(self) -> &'static str {
        todo!()
    }

    pub fn precedence_range(self) -> LeanPrecedenceRange {
        todo!()
    }
}
