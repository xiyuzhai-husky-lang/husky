pub mod r#enum;
pub mod r#extern;
pub mod inductive;
pub mod props_struct;
pub mod structure;
pub mod tuple_struct;
pub mod union;
pub mod unit_struct;

use self::inductive::*;
use self::props_struct::*;
use self::r#enum::*;
use self::r#extern::*;
use self::structure::*;
use self::tuple_struct::*;
use self::union::*;
use self::unit_struct::*;
use super::*;
use husky_entity_path::path::major_item::ty::TypePath;
use husky_syn_decl::decl::major_item::ty::TypeSynDecl;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypeDecSignature {
    Enum(EnumDecSignature),
    PropsStruct(PropsStructDecSignature),
    UnitStruct(UnitStructDecSignature),
    TupleStruct(TupleStructDecSignature),
    Inductive(InductiveTypeDecSignature),
    Structure(StructureTypeDecSignature),
    Extern(ExternDecSignature),
    Union(UnionTypeDecSignature),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum TypeDecTemplate {
    Enum(EnumDecTemplate),
    PropsStruct(PropsStructDecTemplate),
    UnitStruct(UnitStructDecTemplate),
    TupleStruct(TupleStructDecTemplate),
    Inductive(InductiveDecTemplate),
    Structure(StructureTypeDecTemplate),
    Extern(ExternDecTemplate),
    Union(UnionTypeDecTemplate),
}

impl TypeDecTemplate {
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        match self {
            TypeDecTemplate::Enum(decl) => decl.template_parameters(db),
            TypeDecTemplate::UnitStruct(decl) => decl.template_parameters(db),
            TypeDecTemplate::TupleStruct(decl) => decl.template_parameters(db),
            TypeDecTemplate::PropsStruct(decl) => decl.template_parameters(db),
            TypeDecTemplate::Inductive(decl) => decl.template_parameters(db),
            TypeDecTemplate::Structure(decl) => decl.template_parameters(db),
            TypeDecTemplate::Extern(decl) => decl.template_parameters(db),
            TypeDecTemplate::Union(decl) => decl.template_parameters(db),
        }
    }
}

impl HasDecTemplate for TypePath {
    type DecTemplate = TypeDecTemplate;

    #[inline(always)]
    fn dec_template(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecTemplate> {
        ty_dec_template(db, self)
    }
}

#[salsa::tracked]
pub(crate) fn ty_dec_template(
    db: &::salsa::Db,
    path: TypePath,
) -> DecSignatureResult<TypeDecTemplate> {
    let decl = path.syn_decl(db)?;
    Ok(match decl {
        TypeSynDecl::Enum(decl) => EnumDecTemplate::from_decl(db, path, decl)?.into(),
        TypeSynDecl::PropsStruct(decl) => PropsStructDecTemplate::from_decl(db, path, decl)?.into(),
        TypeSynDecl::UnitStruct(decl) => UnitStructDecTemplate::from_decl(db, path, decl)?.into(),
        TypeSynDecl::TupleStruct(decl) => TupleStructDecTemplate::from_decl(db, path, decl)?.into(),
        TypeSynDecl::Inductive(decl) => InductiveDecTemplate::from_decl(db, path, decl)?.into(),
        TypeSynDecl::Structure(decl) => StructureTypeDecTemplate::from_decl(db, path, decl)?.into(),
        TypeSynDecl::Extern(decl) => ExternDecTemplate::from_decl(db, path, decl)?.into(),
        TypeSynDecl::Union(decl) => UnionTypeDecTemplate::from_decl(db, path, decl)?.into(),
    })
}

fn construct_self_ty(
    db: &::salsa::Db,
    path: TypePath,
    template_parameters: &[DeclarativeTemplateParameter],
) -> DecTerm {
    let mut self_ty: DecTerm = path.into();
    for template_parameter in template_parameters {
        self_ty = DecApplication::new(db, self_ty, template_parameter.symbol().into()).into()
    }
    self_ty
}
