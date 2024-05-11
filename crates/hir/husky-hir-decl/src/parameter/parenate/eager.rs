use husky_hir_ty::ritchie::HirContract;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
pub enum HirEagerParenateParameter {
    Simple {
        pattern_idx: HirEagerPatternIdx,
        contract: HirContract,
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
            &ParenateParameterSyndicate::Simple {
                syn_pattern_root,
                ty,
                ..
            } => HirEagerParenateParameter::Simple {
                pattern_idx: builder.hir_eager_pattern_idx(syn_pattern_root),
                contract: HirContract::from_contract(
                    builder
                        .syn_expr_region_data()
                        .pattern_contract(syn_pattern_root.syn_pattern_idx()),
                ),
                ty: builder.hir_ty(ty).unwrap(),
            },
            ParenateParameterSyndicate::Variadic {
                dot_dot_dot_token: _,
                variadic_variant: _,
                symbol_modifier_keyword_group: _,
                ident_token: _,
                variable: _,
                colon: _,
                ty: _,
            } => HirEagerParenateParameter::Variadic,
            ParenateParameterSyndicate::Keyed {
                syn_pattern_root: _,
                symbol_modifier_keyword_group: _,
                ident_token: _,
                variable: _,
                colon: _,
                ty: _,
                eq_token: _,
                default: _,
            } => HirEagerParenateParameter::Keyed,
        })
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirEagerParenateParameters(SmallVec<[HirEagerParenateParameter; 4]>);

impl std::ops::Deref for HirEagerParenateParameters {
    type Target = SmallVec<[HirEagerParenateParameter; 4]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for &HirEagerParenateParameters {
    type Item = HirEagerParenateParameter;

    type IntoIter = impl Iterator<Item = HirEagerParenateParameter>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter().copied()
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
