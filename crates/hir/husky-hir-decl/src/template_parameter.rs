use crate::*;
use husky_hir_ty::trai::HirTrait;
use husky_syn_expr::{TemplateParameterSyndicate, TemplateParameterSyndicateData};
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
            TemplateParameterSyndicateData::Type {
                ident_token,
                traits,
            } => HirTemplateParameterData::Type {
                ident: ident_token.ident(),
                traits: match traits {
                    Some(_) =>
                    /* ad hoc */
                    {
                        vec![]
                    }
                    None => vec![],
                },
            },
            &TemplateParameterSyndicateData::Constant {
                const_token,
                ident_token,
                colon_token,
                ty_expr,
            } => HirTemplateParameterData::Constant {
                ident: ident_token.ident(),
                ty: builder.hir_ty(ty_expr),
            },
            TemplateParameterSyndicateData::Lifetime { label_token } => {
                HirTemplateParameterData::Lifetime {
                    label: label_token.label(),
                }
            }
            TemplateParameterSyndicateData::Place { label_token } => {
                HirTemplateParameterData::Place {
                    label: label_token.label(),
                }
            }
        };
        Self { data }
    }

    pub fn data(&self) -> &HirTemplateParameterData {
        &self.data
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
