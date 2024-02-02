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

use crate::*;
use husky_coword::Ident;
use husky_declarative_signature::{HasDecTemplate, TypeDecTemplate};
use husky_entity_path::TypePath;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
#[salsa::debug_with_db]
pub enum TypeEthTemplate {
    Enum(EnumTypeEthTemplate),
    PropsStruct(PropsStructTypeEthTemplate),
    UnitStruct(UnitStructTypeEthTemplate),
    TupleStruct(TupleStructTypeEthTemplate),
    Inductive(InductiveTypeEthTemplate),
    Structure(StructureTypeEthTemplate),
    Extern(ExternTypeEthTemplate),
    Union(UnionTypeEthTemplate),
}

impl TypeEthTemplate {
    pub fn template_parameters(self, db: &::salsa::Db) -> &[EtherealTemplateParameter] {
        match self {
            TypeEthTemplate::Enum(template) => template.template_parameters(db),
            TypeEthTemplate::PropsStruct(template) => template.template_parameters(db),
            TypeEthTemplate::UnitStruct(template) => template.template_parameters(db),
            TypeEthTemplate::TupleStruct(template) => template.template_parameters(db),
            TypeEthTemplate::Inductive(template) => template.template_parameters(db),
            TypeEthTemplate::Structure(template) => template.template_parameters(db),
            TypeEthTemplate::Extern(template) => template.template_parameters(db),
            TypeEthTemplate::Union(template) => template.template_parameters(db),
        }
    }

    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> Option<EtherealTerm> {
        match self {
            TypeEthTemplate::Enum(_) => None,
            TypeEthTemplate::PropsStruct(slf) => Some(slf.instance_constructor_ty(db).into()),
            TypeEthTemplate::UnitStruct(slf) => Some(slf.instance_constructor_ty(db).into()),
            TypeEthTemplate::TupleStruct(slf) => Some(slf.instance_constructor_ty(db).into()),
            TypeEthTemplate::Inductive(_) => None,
            TypeEthTemplate::Structure(_) => todo!(),
            TypeEthTemplate::Extern(_) => None,
            TypeEthTemplate::Union(_) => None,
        }
    }
}

impl HasEthTemplate for TypePath {
    type EthTemplate = TypeEthTemplate;

    fn ethereal_signature_template(
        self,
        db: &::salsa::Db,
    ) -> EtherealSignatureResult<Self::EthTemplate> {
        ty_ethereal_signature_template(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
fn ty_ethereal_signature_template(
    db: &::salsa::Db,
    path: TypePath,
) -> EtherealSignatureResult<TypeEthTemplate> {
    Ok(match path.declarative_signature_template(db)? {
        TypeDecTemplate::Enum(declarative_signature_template) => {
            EnumTypeEthTemplate::from_declarative(db, path, declarative_signature_template)?.into()
        }
        TypeDecTemplate::PropsStruct(declarative_signature_template) => {
            PropsStructTypeEthTemplate::from_declarative(db, path, declarative_signature_template)?
                .into()
        }
        TypeDecTemplate::UnitStruct(declarative_signature_template) => {
            UnitStructTypeEthTemplate::from_declarative(db, path, declarative_signature_template)?
                .into()
        }
        TypeDecTemplate::TupleStruct(declarative_signature_template) => {
            TupleStructTypeEthTemplate::from_declarative(db, path, declarative_signature_template)?
                .into()
        }
        TypeDecTemplate::Inductive(declarative_signature_template) => {
            InductiveTypeEthTemplate::from_declarative(db, path, declarative_signature_template)?
                .into()
        }
        TypeDecTemplate::Structure(declarative_signature_template) => {
            StructureTypeEthTemplate::from_declarative(db, path, declarative_signature_template)?
                .into()
        }
        TypeDecTemplate::Extern(declarative_signature_template) => {
            ExternTypeEthTemplate::from_declarative(db, path, declarative_signature_template)?
                .into()
        }
        TypeDecTemplate::Union(declarative_signature_template) => {
            UnionTypeEthTemplate::from_declarative(db, path, declarative_signature_template)?.into()
        }
    })
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum PropsFieldEtherealSignature {
    PropsStruct(PropsStructFieldEtherealSignature),
}

pub trait HasPropsFieldEtherealSignature: Copy {
    fn props_field_ethereal_signature(
        self,
        db: &::salsa::Db,
        arguments: &[EtherealTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<PropsFieldEtherealSignature>;
}

impl HasPropsFieldEtherealSignature for TypePath {
    fn props_field_ethereal_signature(
        self,
        db: &::salsa::Db,
        arguments: &[EtherealTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<PropsFieldEtherealSignature> {
        self.ethereal_signature_template(db)?
            .props_field_ethereal_signature(db, arguments, ident)
    }
}

impl HasPropsFieldEtherealSignature for TypeEthTemplate {
    fn props_field_ethereal_signature(
        self,
        db: &::salsa::Db,
        arguments: &[EtherealTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<PropsFieldEtherealSignature> {
        match self {
            TypeEthTemplate::Enum(_)
            | TypeEthTemplate::Inductive(_)
            | TypeEthTemplate::UnitStruct(_)
            | TypeEthTemplate::TupleStruct(_)
            | TypeEthTemplate::Extern(_) => Nothing,
            TypeEthTemplate::PropsStruct(ethereal_signature_template) => {
                ethereal_signature_template.props_field_ethereal_signature(db, arguments, ident)
            }
            TypeEthTemplate::Structure(_) => todo!(),
            TypeEthTemplate::Union(_) => todo!(),
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
