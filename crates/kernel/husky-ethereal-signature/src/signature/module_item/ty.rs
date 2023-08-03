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

use crate::*;
use husky_coword::Ident;
use husky_declarative_signature::{
    HasDeclarativeSignatureTemplate, TypeDeclarativeSignature, TypeDeclarativeSignatureTemplate,
};
use husky_entity_path::TypePath;
use husky_print_utils::p;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum TypeEtherealSignatureTemplate {
    Enum(EnumTypeEtherealSignatureTemplate),
    PropsStruct(PropsStructTypeEtherealSignatureTemplate),
    UnitStruct(UnitStructTypeEtherealSignatureTemplate),
    TupleStruct(TupleStructTypeEtherealSignatureTemplate),
    Record(RecordTypeEtherealSignatureTemplate),
    Inductive(InductiveTypeEtherealSignatureTemplate),
    Structure(StructureTypeEtherealSignatureTemplate),
    Extern(ExternTypeEtherealSignatureTemplate),
    Union(UnionTypeEtherealSignatureTemplate),
}

impl TypeEtherealSignatureTemplate {
    pub fn template_parameters(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> &[EtherealTermTemplateParameter] {
        match self {
            TypeEtherealSignatureTemplate::Enum(template) => template.template_parameters(db),
            TypeEtherealSignatureTemplate::PropsStruct(template) => {
                template.template_parameters(db)
            }
            TypeEtherealSignatureTemplate::UnitStruct(template) => template.template_parameters(db),
            TypeEtherealSignatureTemplate::TupleStruct(template) => {
                template.template_parameters(db)
            }
            TypeEtherealSignatureTemplate::Record(template) => template.template_parameters(db),
            TypeEtherealSignatureTemplate::Inductive(template) => template.template_parameters(db),
            TypeEtherealSignatureTemplate::Structure(template) => template.template_parameters(db),
            TypeEtherealSignatureTemplate::Extern(template) => template.template_parameters(db),
            TypeEtherealSignatureTemplate::Union(template) => template.template_parameters(db),
        }
    }
}

impl HasEtherealSignatureTemplate for TypePath {
    type EtherealSignatureTemplate = TypeEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        ty_ethereal_signature_template(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
fn ty_ethereal_signature_template(
    db: &dyn EtherealSignatureDb,
    path: TypePath,
) -> EtherealSignatureResult<TypeEtherealSignatureTemplate> {
    Ok(match path.declarative_signature_template(db)? {
        TypeDeclarativeSignatureTemplate::Enum(declarative_signature_template) => {
            EnumTypeEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TypeDeclarativeSignatureTemplate::PropsStruct(declarative_signature_template) => {
            PropsStructTypeEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TypeDeclarativeSignatureTemplate::UnitStruct(declarative_signature_template) => {
            UnitStructTypeEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TypeDeclarativeSignatureTemplate::TupleStruct(declarative_signature_template) => {
            TupleStructTypeEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TypeDeclarativeSignatureTemplate::Record(declarative_signature_template) => {
            RecordTypeEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TypeDeclarativeSignatureTemplate::Inductive(declarative_signature_template) => {
            InductiveTypeEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TypeDeclarativeSignatureTemplate::Structure(declarative_signature_template) => {
            StructureTypeEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TypeDeclarativeSignatureTemplate::Extern(declarative_signature_template) => {
            ExternTypeEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TypeDeclarativeSignatureTemplate::Union(declarative_signature_template) => {
            UnionTypeEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
    })
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EtherealSignatureDb)]
#[enum_class::from_variants]
pub enum PropsFieldEtherealSignature {
    PropsStruct(PropsStructFieldEtherealSignature),
}

pub trait HasPropsFieldEtherealSignature: Copy {
    fn regular_field_ethereal_signature(
        self,
        db: &dyn EtherealSignatureDb,
        arguments: &[EtherealTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<PropsFieldEtherealSignature>;
}

impl HasPropsFieldEtherealSignature for TypePath {
    fn regular_field_ethereal_signature(
        self,
        db: &dyn EtherealSignatureDb,
        arguments: &[EtherealTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<PropsFieldEtherealSignature> {
        self.ethereal_signature_template(db)?
            .regular_field_ethereal_signature(db, arguments, ident)
    }
}

impl HasPropsFieldEtherealSignature for TypeEtherealSignatureTemplate {
    fn regular_field_ethereal_signature(
        self,
        db: &dyn EtherealSignatureDb,
        arguments: &[EtherealTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<PropsFieldEtherealSignature> {
        match self {
            TypeEtherealSignatureTemplate::Enum(ethereal_signature_template) => Nothing,
            TypeEtherealSignatureTemplate::PropsStruct(ethereal_signature_template) => {
                ethereal_signature_template.regular_field_ethereal_signature(db, arguments, ident)
            }
            TypeEtherealSignatureTemplate::UnitStruct(_) => todo!(),
            TypeEtherealSignatureTemplate::TupleStruct(_) => todo!(),
            TypeEtherealSignatureTemplate::Record(_) => todo!(),
            TypeEtherealSignatureTemplate::Inductive(_) => todo!(),
            TypeEtherealSignatureTemplate::Structure(_) => todo!(),
            TypeEtherealSignatureTemplate::Extern(_) => todo!(),
            TypeEtherealSignatureTemplate::Union(_) => todo!(),
        }
    }
}

impl PropsFieldEtherealSignature {
    pub fn ty(self) -> EtherealTerm {
        match self {
            PropsFieldEtherealSignature::PropsStruct(signature) => signature.ty(),
        }
    }
}
