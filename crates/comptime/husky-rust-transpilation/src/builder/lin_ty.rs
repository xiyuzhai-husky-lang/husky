use super::*;
use husky_javelin::template_argument::constant::JavelinConstant;
use husky_linket::template_argument::constant::LinConstant;

impl<E> TranspileToRustWith<E> for LinConstant {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        match self.0 {
            JavelinConstant::Unit(value) => builder.result += "()",
            JavelinConstant::Bool(value) => builder.write_display_copyable(value),
            JavelinConstant::Char(value) => builder.write_display_copyable(value),
            JavelinConstant::I8(value) => builder.write_display_copyable(value),
            JavelinConstant::I16(value) => builder.write_display_copyable(value),
            JavelinConstant::I32(value) => builder.write_display_copyable(value),
            JavelinConstant::I64(value) => builder.write_display_copyable(value),
            JavelinConstant::I128(value) => builder.write_display_copyable(value),
            JavelinConstant::ISize(value) => builder.write_display_copyable(value),
            JavelinConstant::U8(value) => builder.write_display_copyable(value),
            JavelinConstant::U16(value) => builder.write_display_copyable(value),
            JavelinConstant::U32(value) => builder.write_display_copyable(value),
            JavelinConstant::U64(value) => builder.write_display_copyable(value),
            JavelinConstant::U128(value) => builder.write_display_copyable(value),
            JavelinConstant::USize(value) => builder.write_display_copyable(value),
            JavelinConstant::R8(value) => builder.write_display_copyable(value),
            JavelinConstant::R16(value) => builder.write_display_copyable(value),
            JavelinConstant::R32(value) => builder.write_display_copyable(value),
            JavelinConstant::R64(value) => builder.write_display_copyable(value),
            JavelinConstant::R128(value) => builder.write_display_copyable(value),
            JavelinConstant::RSize(value) => builder.write_display_copyable(value),
            JavelinConstant::StaticLifetime => builder.static_lifetime(),
        }
    }
}
