use super::*;
use husky_dec_signature::signature::major_item::ty::props_struct::{
    PropsStructDecTemplate, PropsStructFieldDecTemplate,
};
use husky_eth_term::term::ritchie::EthRitchie;

#[salsa::interned]
pub struct PropsStructEthTemplate {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    #[return_ref]
    pub fields: SmallVec<[PropsFieldEthTemplate; 4]>,
    pub instance_constructor_ritchie_ty: EthRitchie,
}

impl HasPropsFieldEthSignature for PropsStructEthTemplate {
    fn props_field_ethereal_signature(
        self,
        db: &::salsa::Db,
        arguments: &[EthTerm],
        ident: Ident,
    ) -> EthSignatureMaybeResult<PropsFieldEthSignature> {
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

impl PropsStructEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: TypePath,
        tmpl: PropsStructDecTemplate,
    ) -> EthSignatureResult<Self> {
        let template_parameters =
            EthTemplateParameters::from_dec(db, tmpl.template_parameters(db))?;
        let fields = tmpl
            .fields(db)
            .iter()
            .copied()
            .map(|dec_template| PropsFieldEthTemplate::from_dec(db, dec_template))
            .collect::<EthSignatureResult<_>>()?;
        let instance_constructor_ritchie_ty =
            EthRitchie::from_dec(db, tmpl.instance_constructor_ritchie_ty(db))?;
        Ok(Self::new(
            db,
            path,
            template_parameters,
            fields,
            instance_constructor_ritchie_ty,
        ))
    }

    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> EthTerm {
        self.instance_constructor_ritchie_ty(db).into()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
pub struct PropsFieldEthTemplate {
    self_ty: EthTerm,
    ident: Ident,
    ty: EthTerm,
}

impl PropsFieldEthTemplate {
    fn from_dec(
        db: &::salsa::Db,
        dec_template: PropsStructFieldDecTemplate,
    ) -> EthSignatureResult<Self> {
        Ok(Self {
            self_ty: EthTerm::ty_from_dec(db, dec_template.self_ty())?,
            ident: dec_template.ident(),
            ty: EthTerm::ty_from_dec(db, dec_template.ty())?,
        })
    }

    // todo: move this to trait
    fn instantiate(
        self,
        template_parameters: &EthTemplateParameters,
        arguments: &[EthTerm],
    ) -> PropsStructFieldEthSignature {
        if template_parameters.data().len() != arguments.len() {
            todo!()
        }

        if template_parameters.data().len() == 0 {
            return PropsStructFieldEthSignature {
                self_ty: self.self_ty,
                ident: self.ident,
                ty: self.ty,
            };
        }
        todo!()
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn ty(&self) -> EthTerm {
        self.ty
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PropsStructFieldEthSignature {
    self_ty: EthTerm,
    ident: Ident,
    ty: EthTerm,
}

impl PropsStructFieldEthSignature {
    pub fn self_ty(&self) -> EthTerm {
        self.self_ty
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn ty(&self) -> EthTerm {
        self.ty
    }
}
