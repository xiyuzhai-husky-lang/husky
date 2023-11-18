use super::*;

impl<E> TranspileToRust<E> for bool {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<E>) {
        builder.write_display_copyable(*self)
    }
}
