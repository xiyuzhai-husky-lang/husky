use crate::*;
use husky_hir_ty::trai::HirTrait;
use husky_syn_expr::{SynTemplateParameterSyndicateData, TemplateParameterSyndicate};
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = HirDeclDb)]
pub struct HirTemplateParameter {
    data: HirTemplateParameterData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = HirDeclDb)]
pub enum HirTemplateParameterData {
    Type { ident: Ident, traits: Vec<HirTrait> },
    Constant { ident: Ident, ty: HirType },
    Lifetime { label: Label },
    Place { label: Label },
}

impl HirTemplateParameter {
    pub fn from_syn(syndicate: &TemplateParameterSyndicate, builder: &HirDeclBuilder) -> Self {
        let data = match syndicate.data() {
            SynTemplateParameterSyndicateData::Type {
                ident_token,
                traits,
            } => todo!(),
            &SynTemplateParameterSyndicateData::Constant {
                const_token,
                ident_token,
                colon_token,
                ty_expr,
            } => HirTemplateParameterData::Constant {
                ident: ident_token.ident(),
                ty: builder.hir_ty(ty_expr),
            },
            SynTemplateParameterSyndicateData::Lifetime { label_token } => todo!(),
            SynTemplateParameterSyndicateData::Place { label_token } => todo!(),
        };
        Self { data }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirTemplateParameters(SmallVec<[HirTemplateParameter; 2]>);

impl HirTemplateParameters {
    pub fn from_syn(syndicates: &[TemplateParameterSyndicate], builder: &HirDeclBuilder) -> Self {
        HirTemplateParameters(
            syndicates
                .iter()
                .map(|syndicate| HirTemplateParameter::from_syn(syndicate, builder))
                .collect(),
        )
    }
}

impl std::ops::Deref for HirTemplateParameters {
    type Target = [HirTemplateParameter];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
