#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemplateVariableClass {
    /// monomorphic
    Mono,
    /// polymorphic
    Poly,
    /// phantomic
    Phan,
}

impl Default for TemplateVariableClass {
    fn default() -> Self {
        TemplateVariableClass::Mono
    }
}
