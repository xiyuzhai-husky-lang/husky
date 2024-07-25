use super::*;

pub(crate) struct TupleFieldVariable(pub(crate) usize);

impl<E> TranspileToRustWith<E> for TupleFieldVariable {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        builder.word(&format!("v{}", self.0))
    }
}
