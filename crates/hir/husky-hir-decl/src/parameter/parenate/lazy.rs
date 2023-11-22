use super::*;
use husky_hir_lazy_expr::HirLazyPatternExprIdx;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirLazyParenateParameter {
    SelfValue,
    Ordinary {
        pattern_expr_idx: HirLazyPatternExprIdx,
        ty: HirType,
    },
    Keyed,
    Variadic,
}

impl HirLazyParenateParameter {
    pub(crate) fn from_self_value_parameter_syndicate(
        _syndicate: Option<SelfValueParameterSyndicate>,
        _db: &dyn HirDeclDb,
    ) -> Self {
        HirLazyParenateParameter::SelfValue
    }

    pub(crate) fn from_syn(
        syndicate: &ParenateSynParameterData,
        _builder: &HirDeclBuilder,
    ) -> Option<Self> {
        Some(match syndicate {
            ParenateSynParameterData::Ordinary {
                syn_pattern_root: _,
                variables: _,
                colon: _,
                ty: _,
            } => HirLazyParenateParameter::Ordinary {
                pattern_expr_idx: todo!(),
                ty: todo!(),
            },
            ParenateSynParameterData::Variadic {
                dot_dot_dot_token: _,
                variadic_variant: _,
                symbol_modifier_keyword_group: _,
                ident_token: _,
                variable: _,
                colon: _,
                ty: _,
            } => HirLazyParenateParameter::Variadic,
            ParenateSynParameterData::Keyed {
                syn_pattern_root: _,
                symbol_modifier_keyword_group: _,
                ident_token: _,
                variable: _,
                colon: _,
                ty: _,
                eq_token: _,
                default: _,
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
        syndicates: &[ParenateSynParameterData],
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
