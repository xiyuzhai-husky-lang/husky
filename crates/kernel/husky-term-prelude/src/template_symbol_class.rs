#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TermTemplateSymbolClass {
    Phantom,
    Runtime,
    Comptime,
}

impl Default for TermTemplateSymbolClass {
    fn default() -> Self {
        TermTemplateSymbolClass::Comptime
    }
}
