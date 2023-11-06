use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirEagerParenateParameter {
    Ordinary {
        pattern_expr_idx: HirEagerPatternExprIdx,
        ty: HirType,
    },
    Keyed,
    Variadic,
}

impl HirEagerParenateParameter {
    pub(crate) fn from_syn(
        syndicate: &ParenateParameterSyndicate,
        builder: &HirDeclBuilder,
    ) -> Option<Self> {
        Some(match syndicate {
            &ParenateParameterSyndicate::Ordinary {
                syn_pattern_root,
                ty,
                ..
            } => HirEagerParenateParameter::Ordinary {
                pattern_expr_idx: builder.hir_eager_pattern_expr_idx(syn_pattern_root),
                ty: builder.hir_ty(ty),
            },
            ParenateParameterSyndicate::Variadic {
                dot_dot_dot_token,
                variadic_variant,
                symbol_modifier_keyword_group,
                ident_token,
                variable,
                colon,
                ty,
            } => HirEagerParenateParameter::Variadic,
            ParenateParameterSyndicate::Keyed {
                syn_pattern_root,
                symbol_modifier_keyword_group,
                ident_token,
                variable,
                colon,
                ty,
                eq_token,
                default,
            } => HirEagerParenateParameter::Keyed,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirEagerParenateParameters(SmallVec<[HirEagerParenateParameter; 4]>);

impl std::ops::Deref for HirEagerParenateParameters {
    type Target = SmallVec<[HirEagerParenateParameter; 4]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl HirEagerParenateParameters {
    pub(crate) fn from_syn(
        syndicates: &[ParenateParameterSyndicate],
        builder: &HirDeclBuilder,
    ) -> Self {
        Self(
            syndicates
                .iter()
                .filter_map(|syndicate| HirEagerParenateParameter::from_syn(syndicate, builder))
                .collect(),
        )
    }
}
