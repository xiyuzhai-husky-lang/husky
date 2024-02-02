mod r#enum;
mod r#extern;
mod inductive;
mod props_struct;
mod structure;
mod tuple_struct;
mod union;
mod unit_struct;

pub use self::inductive::*;
pub use self::props_struct::*;
pub use self::r#enum::*;
pub use self::r#extern::*;
pub use self::structure::*;
pub use self::tuple_struct::*;
pub use self::union::*;
pub use self::unit_struct::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub enum TypeDecSignature {
    Enum(EnumTypeDecSignature),
    PropsStruct(PropsStructDecSignature),
    UnitStruct(UnitStructTypeDecSignature),
    TupleStruct(TupleStructTypeDecSignature),
    Inductive(InductiveTypeDecSignature),
    Structure(StructureTypeDecSignature),
    Extern(ExternTypeDecSignature),
    Union(UnionTypeDecSignature),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum TypeDecTemplate {
    Enum(EnumTypeDecTemplate),
    PropsStruct(PropsStructTypeDecTemplate),
    UnitStruct(UnitStructTypeDecTemplate),
    TupleStruct(TupleStructTypeDecTemplate),
    Inductive(InductiveTypeDecTemplate),
    Structure(StructureTypeDecTemplate),
    Extern(ExternTypeDecTemplate),
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

#[salsa::tracked(jar = DecSignatureJar)]
pub(crate) fn ty_dec_template(
    db: &::salsa::Db,
    path: TypePath,
) -> DecSignatureResult<TypeDecTemplate> {
    let decl = path.syn_decl(db)?;
    Ok(match decl {
        TypeSynDecl::Enum(decl) => EnumTypeDecTemplate::from_decl(db, path, decl)?.into(),
        TypeSynDecl::PropsStruct(decl) => {
            PropsStructTypeDecTemplate::from_decl(db, path, decl)?.into()
        }
        TypeSynDecl::UnitStruct(decl) => {
            UnitStructTypeDecTemplate::from_decl(db, path, decl)?.into()
        }
        TypeSynDecl::TupleStruct(decl) => {
            TupleStructTypeDecTemplate::from_decl(db, path, decl)?.into()
        }
        TypeSynDecl::Inductive(decl) => InductiveTypeDecTemplate::from_decl(db, path, decl)?.into(),
        TypeSynDecl::Structure(decl) => StructureTypeDecTemplate::from_decl(db, path, decl)?.into(),
        TypeSynDecl::Extern(decl) => ExternTypeDecTemplate::from_decl(db, path, decl)?.into(),
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
        self_ty = ApplicationDecTerm::new(db, self_ty, template_parameter.symbol().into()).into()
    }
    self_ty
}
