use super::*;

impl TranspileToRust for bool {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        builder.write_display_copyable(*self)
    }
}
