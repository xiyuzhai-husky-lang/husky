use super::*;
use husky_declarative_signature::{
    TupleStructFieldDeclarativeSignatureTemplate, TupleStructTypeDeclarativeSignatureTemplate,
};

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TupleStructTypeEtherealSignatureTemplate {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: EtherealTermTemplateParameters,
    #[return_ref]
    pub fields: SmallVec<[TupleFieldEtherealSignatureTemplate; 2]>,
}

impl TupleStructTypeEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        path: TypePath,
        declarative_signature_template: TupleStructTypeDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters = EtherealTermTemplateParameters::from_declarative(
            db,
            declarative_signature_template.template_parameters(db),
        )?;
        let fields = declarative_signature_template
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
        Ok(Self::new(db, path, template_parameters, fields))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TupleFieldEtherealSignatureTemplate {
    ty: EtherealTerm,
}

impl TupleFieldEtherealSignatureTemplate {
    fn from_declarative(
        db: &dyn EtherealSignatureDb,
        declarative_signature_template: TupleStructFieldDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        Ok(Self {
            ty: EtherealTerm::ty_from_declarative(db, declarative_signature_template.ty())?,
        })
    }

    // todo: move this to trait
    fn instantiate(
        self,
        template_parameters: &EtherealTermTemplateParameters,
        arguments: &[EtherealTerm],
    ) -> TupleStructFieldEtherealSignature {
        if template_parameters.data().len() != arguments.len() {
            todo!()
        }

        if template_parameters.data().len() == 0 {
            return TupleStructFieldEtherealSignature { ty: self.ty };
        }
        todo!()
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
