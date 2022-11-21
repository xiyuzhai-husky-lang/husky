mod enum_ty;
mod struct_ty;

pub use enum_ty::*;
pub use struct_ty::*;

#[derive(Debug, PartialEq, Eq)]
pub enum TypeDefn {
    Enum(EnumDefn),
    StructLike(StructDefn),
}
