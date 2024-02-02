use super::*;

impl<E> TranspileToRustWith<E> for bool {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        builder.write_display_copyable(self)
    }
}

// ad hoc
impl<E> TranspileToRustWith<E> for i32 {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        builder.write_display_copyable(self)
    }
}

// ad hoc
impl<E> TranspileToRustWith<E> for usize {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        builder.write_display_copyable(self)
    }
}

impl<E> TranspileToRustWith<E> for Literal {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db();
        match self {
            Literal::Unit(()) => builder.write_str("()"),
            Literal::Bool(value) => builder.write_display_copyable(value),
            Literal::I8(value) => builder.write_display_copyable(value),
            Literal::I16(value) => builder.write_display_copyable(value),
            Literal::I32(value) => builder.write_display_copyable(value),
            Literal::I64(lit) => builder.write_display_copyable(lit.value(db)),
            Literal::I128(lit) => builder.write_display_copyable(lit.value(db)),
            Literal::ISize(lit) => builder.write_display_copyable(lit.value(db)),
            Literal::U8(value) => builder.write_display_copyable(value),
            Literal::U16(value) => builder.write_display_copyable(value),
            Literal::U32(value) => builder.write_display_copyable(value),
            Literal::U64(lit) => builder.write_display_copyable(lit.value(db)),
            Literal::U128(lit) => builder.write_display_copyable(lit.value(db)),
            Literal::USize(lit) => builder.write_display_copyable(lit.value(db)),
            Literal::R8(_) => todo!(),
            Literal::R16(_) => todo!(),
            Literal::R32(value) => builder.write_display_copyable(value),
            Literal::R64(_) => todo!(),
            Literal::R128(_) => todo!(),
            Literal::RSize(_) => todo!(),
            Literal::Nat(_) => todo!(),
            Literal::F32(value) => builder.result += value.text(db),
            Literal::F64(value) => builder.result += value.text(db),
            Literal::String(lit) => {
                use std::fmt::Write;
                write!(builder.result, "{:?}", lit.data(db)).unwrap();
            }
            Literal::StaticLifetime => todo!(),
        }
    }
}

impl<E> TranspileToRustWith<E> for &str {
    fn transpile_to_rust(self, _builder: &mut RustTranspilationBuilder<E>) {
        todo!()
    }
}

impl<'a, 'b, E> RustTranspilationBuilder<'a, 'b, E> {
    pub(crate) fn str_literal(&mut self, s: &str) {
        // ad hoc
        use std::fmt::Write;
        write!(self.result, "{:?}", s).unwrap()
    }
}
