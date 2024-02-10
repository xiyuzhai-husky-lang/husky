#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemplateVarClass {
    Phantom,
    Runtime,
    Comptime,
}

impl Default for TemplateVarClass {
    fn default() -> Self {
        TemplateVarClass::Comptime
    }
}
