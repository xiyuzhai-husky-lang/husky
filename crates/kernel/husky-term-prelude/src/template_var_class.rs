#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemplateVariableClass {
    Phantom,
    Runtime,
    Comptime,
}

impl Default for TemplateVariableClass {
    fn default() -> Self {
        TemplateVariableClass::Comptime
    }
}
