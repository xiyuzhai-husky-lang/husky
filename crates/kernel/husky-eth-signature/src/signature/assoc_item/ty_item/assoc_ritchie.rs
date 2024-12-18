use super::*;
use husky_dec_signature::signature::assoc_item::ty_item::assoc_ritchie::TypeAssocRitchieDecTemplate;
use husky_entity_kind::ritchie::RitchieItemKind;
use husky_eth_term::term::ritchie::EthRitchie;

#[salsa::interned]
pub struct TypeAssocRitchieEthTemplate {
    pub path: TypeItemPath,
    // todo: is this necessary?
    pub self_ty: EthTerm,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    #[return_ref]
    pub parenate_parameters: EtherealParenateParameters,
    pub return_ty: EthTerm,
    pub ty: EthTerm,
}

impl TypeAssocRitchieEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: TypeItemPath,
        declarative_signature: TypeAssocRitchieDecTemplate,
    ) -> EthSignatureResult<Self> {
        let self_ty = EthTerm::ty_from_dec(db, declarative_signature.self_ty(db))?;
        let template_parameters =
            EthTemplateParameters::from_dec(db, declarative_signature.template_parameters(db))?;
        let parenate_parameters = EtherealParenateParameters::from_dec(
            db,
            declarative_signature.parenate_parameters(db),
        )?;
        let return_ty = EthTerm::ty_from_dec(db, declarative_signature.return_ty(db))?;
        let ty = EthRitchie::new(
            db,
            RitchieItemKind::Fn.into(),
            parenate_parameters.iter().copied(),
            return_ty,
        )?
        .into();
        Ok(Self::new(
            db,
            path,
            self_ty,
            template_parameters,
            parenate_parameters,
            return_ty,
            ty,
        ))
    }
}
