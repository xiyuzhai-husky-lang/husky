use super::*;
use husky_declarative_signature::{
    TupleStructFieldDeclarativeSignatureTemplate, TupleStructTypeDeclarativeSignatureTemplate,
};

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TupleStructTypeEtherealSignatureTemplate {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: EtherealTemplateParameters,
    #[return_ref]
    pub fields: SmallVec<[TupleFieldEtherealSignatureTemplate; 2]>,
    pub instance_constructor_ritchie_ty: EtherealTermRitchie,
}

impl TupleStructTypeEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: TypePath,
        tmpl: TupleStructTypeDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters =
            EtherealTemplateParameters::from_declarative(db, tmpl.template_parameters(db))?;
        let fields = tmpl
            .fields(db)
            .iter()
            .copied()
            .map(|declarative_signature_template| {
                TupleFieldEtherealSignatureTemplate::from_declarative(
                    db,
                    declarative_signature_template,
                )
            })
            .collect::<EtherealSignatureResult<_>>()?;
        let instance_constructor_ritchie_ty =
            EtherealTermRitchie::from_declarative(db, tmpl.instance_constructor_ritchie_ty(db))?;
        Ok(Self::new(
            db,
            path,
            template_parameters,
            fields,
            instance_constructor_ritchie_ty,
        ))
    }

    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> EtherealTerm {
        self.instance_constructor_ritchie_ty(db).into()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TupleFieldEtherealSignatureTemplate {
    ty: EtherealTerm,
}

impl TupleFieldEtherealSignatureTemplate {
    fn from_declarative(
        db: &::salsa::Db,
        declarative_signature_template: TupleStructFieldDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        Ok(Self {
            ty: EtherealTerm::ty_from_declarative(db, declarative_signature_template.ty())?,
        })
    }

    pub fn ty(&self) -> EtherealTerm {
        self.ty
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TupleStructFieldEtherealSignature {
    ty: EtherealTerm,
}

impl TupleStructFieldEtherealSignature {
    pub fn ty(&self) -> EtherealTerm {
        self.ty
    }
}
