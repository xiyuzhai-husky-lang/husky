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

impl<E> TranspileToRustWith<E> for TermLiteral {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db();
        match self {
            TermLiteral::Unit => builder.write_str("()"),
            TermLiteral::Bool(value) => builder.write_display_copyable(value),
            TermLiteral::I8(value) => builder.write_display_copyable(value),
            TermLiteral::I16(value) => builder.write_display_copyable(value),
            TermLiteral::I32(value) => builder.write_display_copyable(value),
            TermLiteral::I64(lit) => builder.write_display_copyable(lit.value(db)),
            TermLiteral::I128(lit) => builder.write_display_copyable(lit.value(db)),
            TermLiteral::ISize(lit) => builder.write_display_copyable(lit.value(db)),
            TermLiteral::U8(value) => builder.write_display_copyable(value),
            TermLiteral::U16(value) => builder.write_display_copyable(value),
            TermLiteral::U32(value) => builder.write_display_copyable(value),
            TermLiteral::U64(lit) => builder.write_display_copyable(lit.value(db)),
            TermLiteral::U128(lit) => builder.write_display_copyable(lit.value(db)),
            TermLiteral::USize(lit) => builder.write_display_copyable(lit.value(db)),
            TermLiteral::R8(_) => todo!(),
            TermLiteral::R16(_) => todo!(),
            TermLiteral::R32(value) => builder.write_display_copyable(value),
            TermLiteral::R64(_) => todo!(),
            TermLiteral::R128(_) => todo!(),
            TermLiteral::RSize(_) => todo!(),
            TermLiteral::Nat(_) => todo!(),
            TermLiteral::F32(value) => builder.result += value.text(db),
            TermLiteral::F64(value) => builder.result += value.text(db),
            TermLiteral::String(lit) => {
                use std::fmt::Write;
                write!(builder.result, "{:?}", lit.data(db)).unwrap();
            }
            TermLiteral::StaticLifetime => todo!(),
        }
    }
}
