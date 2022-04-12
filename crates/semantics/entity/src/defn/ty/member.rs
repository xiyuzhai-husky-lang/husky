mod field;
mod method;

pub use field::*;
pub use method::*;

use super::*;
#[derive(Debug, PartialEq, Eq)]
pub enum TypeMemberDefn {
    Field(Arc<FieldDefn>),
    Method(Arc<MethodDefn>),
}

impl HasKey<CustomIdentifier> for TypeMemberDefn {
    fn key(&self) -> CustomIdentifier {
        match self {
            TypeMemberDefn::Field(field) => field.ident,
            TypeMemberDefn::Method(method) => method.ident,
        }
    }
}
