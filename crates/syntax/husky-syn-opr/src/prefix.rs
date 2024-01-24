#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SynPrefixOpr {
    Minus,  // -
    Not,    // !$0
    Tilde,  // ~
    Ref,    // &
    Option, // ?
}

impl SynPrefixOpr {
    pub fn code(self) -> &'static str {
        match self {
            SynPrefixOpr::Minus => "-",
            SynPrefixOpr::Not => "!",
            SynPrefixOpr::Tilde => "!",
            SynPrefixOpr::Ref => "&",
            SynPrefixOpr::Option => "?",
        }
    }
}
