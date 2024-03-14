use super::*;

pub(crate) struct TupleFieldVariable(pub(crate) usize);

impl<E> TranspileToRustWith<E> for TupleFieldVariable {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        use std::fmt::Write;

        write!(builder.result, "v{}", self.0).unwrap()
    }
}
