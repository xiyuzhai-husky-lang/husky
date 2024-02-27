use super::*;
use husky_entity_kind::ritchie::RitchieItemKind;
use husky_eth_term::term::ritchie::{EthRitchie, EtherealRitchieParameter};
use husky_term_prelude::ritchie::RitchieKind;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct GnFugitiveEthTemplate {
    pub path: FugitivePath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    pub ritchie_ty: EthRitchie,
}

impl GnFugitiveEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: FugitivePath,
        tmpl: MajorGnDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters =
            EthTemplateParameters::from_dec(db, tmpl.template_parameters(db))?;
        let ritchie_params: SmallVec<[_; 4]> = tmpl
            .parenate_parameters(db)
            .iter()
            .map(|&param| EtherealRitchieParameter::from_dec(param, db))
            .collect::<EthTermResult<SmallVec<[_; 4]>>>()?;
        let return_ty = EthTerm::ty_from_dec(db, tmpl.return_ty(db))?;
        let ritchie_ty =
            EthRitchie::new(db, RitchieKind::RITCHIE_TYPE_GN, ritchie_params, return_ty)?;
        Ok(Self::new(db, path, template_parameters, ritchie_ty))
    }
}
