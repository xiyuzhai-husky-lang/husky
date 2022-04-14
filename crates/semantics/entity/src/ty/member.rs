mod field;
mod method;

pub use field::*;
use map_collect::MapCollect;
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

#[derive(Debug, PartialEq, Eq)]
pub enum MemberDefn {
    TypeField(Arc<FieldDefn>),
    TypeMethod(Arc<MethodDefn>),
}

impl From<&TypeMemberDefn> for MemberDefn {
    fn from(type_member_defn: &TypeMemberDefn) -> Self {
        match type_member_defn {
            TypeMemberDefn::Field(field_defn) => Self::TypeField(field_defn.clone()),
            TypeMemberDefn::Method(method_defn) => Self::TypeMethod(method_defn.clone()),
        }
    }
}

impl MemberDefn {
    pub fn collect_all(
        type_members: &[TypeMemberDefn],
        trait_impls: &[Arc<TraitImplDefn>],
    ) -> Vec<MemberDefn> {
        let mut members = type_members.map(|type_member| type_member.into());
        for trait_impl in trait_impls {
            todo!()
        }
        members
    }
}
