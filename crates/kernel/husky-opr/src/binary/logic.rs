use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryShortcuitLogicOpr {
    And,
    Or,
}

impl BinaryShortcuitLogicOpr {
    pub fn husky_code(self) -> &'static str {
        match self {
            BinaryShortcuitLogicOpr::And => "&&",
            BinaryShortcuitLogicOpr::Or => "||",
        }
    }

    pub fn spaced_husky_code(self) -> &'static str {
        match self {
            BinaryShortcuitLogicOpr::And => todo!(),
            BinaryShortcuitLogicOpr::Or => todo!(),
        }
    }
}

impl HasPrecedence for BinaryShortcuitLogicOpr {
    #[inline(always)]
    fn precedence(self) -> Precedence {
        match self {
            BinaryShortcuitLogicOpr::And => Precedence::And,
            BinaryShortcuitLogicOpr::Or => Precedence::Or,
        }
    }
}
