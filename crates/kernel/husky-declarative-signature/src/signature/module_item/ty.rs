mod r#enum;
mod r#extern;
mod inductive;
mod props_struct;
mod record;
mod structure;
mod tuple_struct;
mod union;
mod unit_struct;

pub use self::inductive::*;
pub use self::props_struct::*;
pub use self::r#enum::*;
pub use self::r#extern::*;
pub use self::record::*;
pub use self::structure::*;
pub use self::tuple_struct::*;
pub use self::union::*;
pub use self::unit_struct::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb)]
pub enum TypeDeclarativeSignature {
    Enum(EnumTypeDeclarativeSignature),
    PropsStruct(PropsStructDeclarativeSignature),
    UnitStruct(UnitStructTypeDeclarativeSignature),
    TupleStruct(TupleStructTypeDeclarativeSignature),
    Record(RecordTypeDeclarativeSignature),
    Inductive(InductiveTypeDeclarativeSignature),
    Structure(StructureTypeDeclarativeSignature),
    Extern(ExternTypeDeclarativeSignature),
    Union(UnionTypeDeclarativeSignature),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb)]
#[enum_class::from_variants]
pub enum TypeDeclarativeSignatureTemplate {
    Enum(EnumTypeDeclarativeSignatureTemplate),
    PropsStruct(PropsStructTypeDeclarativeSignatureTemplate),
    UnitStruct(UnitStructTypeDeclarativeSignatureTemplate),
    TupleStruct(TupleStructTypeDeclarativeSignatureTemplate),
    Record(RecordTypeDeclarativeSignatureTemplate),
    Inductive(InductiveTypeDeclarativeSignatureTemplate),
    Structure(StructureTypeDeclarativeSignatureTemplate),
    Extern(ExternTypeDeclarativeSignatureTemplate),
    Union(UnionTypeDeclarativeSignatureTemplate),
}

impl TypeDeclarativeSignatureTemplate {
    pub fn template_parameters(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> &[DeclarativeTemplateParameter] {
        match self {
            TypeDeclarativeSignatureTemplate::Enum(decl) => decl.template_parameters(db),
            TypeDeclarativeSignatureTemplate::UnitStruct(decl) => decl.template_parameters(db),
            TypeDeclarativeSignatureTemplate::TupleStruct(decl) => decl.template_parameters(db),
            TypeDeclarativeSignatureTemplate::PropsStruct(decl) => decl.template_parameters(db),
            TypeDeclarativeSignatureTemplate::Record(decl) => decl.template_parameters(db),
            TypeDeclarativeSignatureTemplate::Inductive(decl) => decl.template_parameters(db),
            TypeDeclarativeSignatureTemplate::Structure(decl) => decl.template_parameters(db),
            TypeDeclarativeSignatureTemplate::Extern(decl) => decl.template_parameters(db),
            TypeDeclarativeSignatureTemplate::Union(decl) => decl.template_parameters(db),
        }
    }
}

impl HasDeclarativeSignatureTemplate for TypePath {
    type DeclarativeSignatureTemplate = TypeDeclarativeSignatureTemplate;

    #[inline(always)]
    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        ty_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn ty_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    path: TypePath,
) -> DeclarativeSignatureResult<TypeDeclarativeSignatureTemplate> {
    let decl = path.syn_decl(db)?;
    Ok(match decl {
        TypeSynDecl::Enum(decl) => {
            EnumTypeDeclarativeSignatureTemplate::from_decl(db, path, decl)?.into()
        }
        TypeSynDecl::PropsStruct(decl) => {
            PropsStructTypeDeclarativeSignatureTemplate::from_decl(db, path, decl)?.into()
        }
        TypeSynDecl::UnitStruct(decl) => {
            UnitStructTypeDeclarativeSignatureTemplate::from_decl(db, path, decl)?.into()
        }
        TypeSynDecl::TupleStruct(decl) => {
            TupleStructTypeDeclarativeSignatureTemplate::from_decl(db, path, decl)?.into()
        }
        TypeSynDecl::Record(decl) => {
            RecordTypeDeclarativeSignatureTemplate::from_decl(db, path, decl)?.into()
        }
        TypeSynDecl::Inductive(decl) => {
            InductiveTypeDeclarativeSignatureTemplate::from_decl(db, path, decl)?.into()
        }
        TypeSynDecl::Structure(decl) => {
            StructureTypeDeclarativeSignatureTemplate::from_decl(db, path, decl)?.into()
        }
        TypeSynDecl::Extern(decl) => {
            ExternTypeDeclarativeSignatureTemplate::from_decl(db, path, decl)?.into()
        }
        TypeSynDecl::Union(decl) => {
            UnionTypeDeclarativeSignatureTemplate::from_decl(db, path, decl)?.into()
        }
    })
}

fn construct_self_ty(
    db: &dyn DeclarativeSignatureDb,
    path: TypePath,
    template_parameters: &[DeclarativeTemplateParameter],
) -> DeclarativeTerm {
    let mut self_ty: DeclarativeTerm = path.into();
    for template_parameter in template_parameters {
        self_ty =
            DeclarativeTermExplicitApplication::new(db, self_ty, template_parameter.symbol().into())
                .into()
    }
    self_ty
}
