use crate::precedence::{LnPrecedence, LnPrecedenceRange};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum LnPrefixOpr {}

impl LnPrefixOpr {
    pub fn fmt_str(self) -> &'static str {
        todo!()
    }

    pub fn outer_precedence(self) -> LnPrecedence {
        todo!()
    }

    pub fn precedence_range(self) -> LnPrecedenceRange {
        todo!()
    }
}
