use super::*;
use husky_declarative_signature::{
    PropsStructFieldDeclarativeSignatureTemplate, PropsTypeStructDeclarativeSignatureTemplate,
};

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct PropsStructTypeEtherealSignatureTemplate {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: EtherealTermTemplateParameters,
    #[return_ref]
    pub fields: SmallVec<[RegularFieldEtherealSignatureTemplate; 4]>,
}

impl HasRegularFieldEtherealSignature for PropsStructTypeEtherealSignatureTemplate {
    fn regular_field_ethereal_signature(
        self,
        db: &dyn EtherealSignatureDb,
        arguments: &[EtherealTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<RegularFieldEtherealSignature> {
        let field = self
            .fields(db)
            .iter()
            .copied()
            .find(|field| field.ident == ident)?;
        JustOk(
            field
                .instantiate(self.template_parameters(db), arguments)
                .into(),
        )
    }
}

impl PropsStructTypeEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        path: TypePath,
        declarative_signature_template: PropsTypeStructDeclarativeSignatureTemplate,
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
                RegularFieldEtherealSignatureTemplate::from_declarative(
                    db,
                    declarative_signature_template,
                )
            })
            .collect::<EtherealSignatureResult<_>>()?;
        Ok(Self::new(db, path, template_parameters, fields))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = EtherealSignatureDb, jar= EtherealSignatureJar)]
pub struct RegularFieldEtherealSignatureTemplate {
    ident: Ident,
    ty: EtherealTerm,
}

impl RegularFieldEtherealSignatureTemplate {
    fn from_declarative(
        db: &dyn EtherealSignatureDb,
        declarative_signature_template: PropsStructFieldDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        Ok(Self {
            ident: declarative_signature_template.ident(),
            ty: EtherealTerm::ty_from_declarative(db, declarative_signature_template.ty())?,
        })
    }

    // todo: move this to trait
    fn instantiate(
        self,
        template_parameters: &EtherealTermTemplateParameters,
        arguments: &[EtherealTerm],
    ) -> PropsStructFieldEtherealSignature {
        if template_parameters.data().len() != arguments.len() {
            todo!()
        }

        if template_parameters.data().len() == 0 {
            return PropsStructFieldEtherealSignature {
                ident: self.ident,
                ty: self.ty,
            };
        }
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EtherealSignatureDb)]
pub struct PropsStructFieldEtherealSignature {
    ident: Ident,
    ty: EtherealTerm,
}

impl PropsStructFieldEtherealSignature {
    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn ty(&self) -> EtherealTerm {
        self.ty
    }
}
