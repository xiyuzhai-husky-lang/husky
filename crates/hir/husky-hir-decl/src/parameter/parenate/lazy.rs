use super::*;
use husky_hir_lazy_expr::HirLazyPatternExprIdx;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirLazyParenateParameter {
    SelfValue,
    Ordinary {
        pattern_expr_idx: HirLazyPatternExprIdx,
    },
    Keyed,
    Variadic,
}

impl HirLazyParenateParameter {
    pub(crate) fn from_self_value_parameter_syndicate(
        syndicate: Option<SelfValueParameterSyndicate>,
        db: &dyn HirDeclDb,
    ) -> Self {
        HirLazyParenateParameter::SelfValue
    }

    pub(crate) fn from_syn(
        syndicate: &ParenateParameterSyndicate,
        builder: &HirDeclBuilder,
    ) -> Option<Self> {
        Some(match syndicate {
            ParenateParameterSyndicate::Ordinary {
                syn_pattern_root,
                variables,
                colon,
                ty,
            } => HirLazyParenateParameter::Ordinary {
                pattern_expr_idx: todo!(),
            },
            ParenateParameterSyndicate::Variadic {
                dot_dot_dot_token,
                variadic_variant,
                symbol_modifier_keyword_group,
                ident_token,
                variable,
                colon,
                ty,
            } => HirLazyParenateParameter::Variadic,
            ParenateParameterSyndicate::Keyed {
                syn_pattern_root,
                symbol_modifier_keyword_group,
                ident_token,
                variable,
                colon,
                ty,
                eq_token,
                default,
            } => HirLazyParenateParameter::Keyed,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirLazyParenateParameters(SmallVec<[HirLazyParenateParameter; 4]>);

impl std::ops::Deref for HirLazyParenateParameters {
    type Target = SmallVec<[HirLazyParenateParameter; 4]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl HirLazyParenateParameters {
    pub(crate) fn from_syn(
        syndicates: &[ParenateParameterSyndicate],
        builder: &HirDeclBuilder,
    ) -> Self {
        Self(
            syndicates
                .iter()
                .filter_map(|syndicate| HirLazyParenateParameter::from_syn(syndicate, builder))
                .collect(),
        )
    }
}
