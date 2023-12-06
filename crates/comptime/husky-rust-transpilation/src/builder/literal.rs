use super::*;

impl<E> TranspileToRustWith<E> for bool {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<E>) {
        builder.write_display_copyable(*self)
    }
}

// ad hoc
impl<E> TranspileToRustWith<E> for i32 {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<E>) {
        builder.write_display_copyable(*self)
    }
}

// ad hoc
impl<E> TranspileToRustWith<E> for usize {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<E>) {
        builder.write_display_copyable(*self)
    }
}
