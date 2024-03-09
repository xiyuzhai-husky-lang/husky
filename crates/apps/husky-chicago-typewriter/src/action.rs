#[derive(Debug, Clone)]
pub enum TypeAction {
    Incr,
    Decr,
}

impl TypeAction {
    pub(crate) fn code(&self) -> &str {
        match self {
            TypeAction::Incr => "++",
            TypeAction::Decr => "--",
        }
    }
}
