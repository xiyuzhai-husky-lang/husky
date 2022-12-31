#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryShortcuitLogicPunctuation {
    And,
    Or,
}

impl BinaryShortcuitLogicPunctuation {
    pub fn husky_code(self) -> &'static str {
        match self {
            BinaryShortcuitLogicPunctuation::And => "&&",
            BinaryShortcuitLogicPunctuation::Or => "||",
        }
    }

    pub fn spaced_husky_code(self) -> &'static str {
        match self {
            BinaryShortcuitLogicPunctuation::And => todo!(),
            BinaryShortcuitLogicPunctuation::Or => todo!(),
        }
    }
}
