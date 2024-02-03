use super::*;
use husky_eth_term::term::ritchie::{EthRitchie, EtherealRitchieParameter};
use husky_term_prelude::{RitchieKind, TypeRitchieKind};

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct GnFugitiveEthTemplate {
    pub path: MajorFugitivePath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    pub ritchie_ty: EthRitchie,
}

impl GnFugitiveEthTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: MajorFugitivePath,
        tmpl: MajorGnDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters =
            EthTemplateParameters::from_declarative(db, tmpl.template_parameters(db))?;
        let ritchie_params: SmallVec<[_; 4]> = tmpl
            .parenate_parameters(db)
            .iter()
            .map(|&param| EtherealRitchieParameter::from_declarative(param, db))
            .collect::<EthTermResult<SmallVec<[_; 4]>>>()?;
        let return_ty = EthTerm::ty_from_declarative(db, tmpl.return_ty(db))?;
        let ritchie_ty = EthRitchie::new(
            db,
            RitchieKind::Type(TypeRitchieKind::Gn),
            ritchie_params,
            return_ty,
        )?;
        Ok(Self::new(db, path, template_parameters, ritchie_ty))
    }
}
