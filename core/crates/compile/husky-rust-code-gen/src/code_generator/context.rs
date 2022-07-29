pub enum RustCodeGenContext {
    Normal,
    ThisFieldWithPrefix { prefix: &'static str },
}
