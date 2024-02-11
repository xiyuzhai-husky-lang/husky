#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemplateSvarClass {
    Phantom,
    Runtime,
    Comptime,
}

impl Default for TemplateSvarClass {
    fn default() -> Self {
        TemplateSvarClass::Comptime
    }
}
