use crate::db::HirDeclDb;
use husky_syn_expr::{ParenateParameterSyndicate, SelfValueParameterSyndicate};
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirParenateParameter {
    Ordinary,
    Keyed,
    Variadic,
}

impl HirParenateParameter {
    pub(crate) fn from_self_value_parameter_syndicate(
        syndicate: Option<SelfValueParameterSyndicate>,
        db: &dyn HirDeclDb,
    ) -> Self {
        HirParenateParameter::Ordinary
    }

    pub(crate) fn from(syndicate: &ParenateParameterSyndicate, db: &dyn HirDeclDb) -> Option<Self> {
        Some(match syndicate {
            ParenateParameterSyndicate::Ordinary {
                syn_pattern_root,
                variables,
                colon,
                ty,
            } => HirParenateParameter::Ordinary,
            ParenateParameterSyndicate::Variadic {
                dot_dot_dot_token,
                variadic_variant,
                symbol_modifier_keyword_group,
                ident_token,
                variable,
                colon,
                ty,
            } => HirParenateParameter::Variadic,
            ParenateParameterSyndicate::Keyed {
                syn_pattern_root,
                symbol_modifier_keyword_group,
                ident_token,
                variable,
                colon,
                ty,
                eq_token,
                default,
            } => HirParenateParameter::Keyed,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirParenateParameters(SmallVec<[HirParenateParameter; 4]>);

impl std::ops::Deref for HirParenateParameters {
    type Target = SmallVec<[HirParenateParameter; 4]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl HirParenateParameters {
    pub(crate) fn from_syn(syndicates: &[ParenateParameterSyndicate], db: &dyn HirDeclDb) -> Self {
        Self(
            syndicates
                .iter()
                .filter_map(|syndicate| HirParenateParameter::from(syndicate, db))
                .collect(),
        )
    }
}
