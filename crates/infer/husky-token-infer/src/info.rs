#[derive(Debug, PartialEq, Eq)]
pub enum TokenInfo {
    None,
}

impl Default for TokenInfo {
    fn default() -> Self {
        TokenInfo::None
    }
}
