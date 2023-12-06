use husky_term_prelude::Contract;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirEagerContract {
    None,
    Move,
    Borrow,
    BorrowMut,
    Const,
    Leash,
    At,
}

impl HirEagerContract {
    fn from_term(contract: Contract) -> Self {
        match contract {
            Contract::Pure => HirEagerContract::None,
            Contract::Move => HirEagerContract::Move,
            Contract::Borrow => HirEagerContract::Borrow,
            Contract::BorrowMut => HirEagerContract::BorrowMut,
            Contract::Const => HirEagerContract::Const,
            Contract::Leash => HirEagerContract::Leash,
            Contract::At => HirEagerContract::At,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
pub enum HirEagerParenateParameter {
    Ordinary {
        pattern_expr_idx: HirEagerPatternExprIdx,
        contract: HirEagerContract,
        ty: HirType,
    },
    Keyed,
    Variadic,
}

impl HirEagerParenateParameter {
    pub(crate) fn from_syn(
        syndicate: &ParenateSynParameterData,
        builder: &HirDeclBuilder,
    ) -> Option<Self> {
        Some(match syndicate {
            &ParenateSynParameterData::Ordinary {
                syn_pattern_root,
                ty,
                ..
            } => HirEagerParenateParameter::Ordinary {
                pattern_expr_idx: builder.hir_eager_pattern_expr_idx(syn_pattern_root),
                contract: HirEagerContract::from_term(
                    builder
                        .syn_expr_region_data()
                        .pattern_contract(syn_pattern_root.syn_pattern_expr_idx()),
                ),
                ty: builder.hir_ty(ty).unwrap(),
            },
            ParenateSynParameterData::Variadic {
                dot_dot_dot_token: _,
                variadic_variant: _,
                symbol_modifier_keyword_group: _,
                ident_token: _,
                variable: _,
                colon: _,
                ty: _,
            } => HirEagerParenateParameter::Variadic,
            ParenateSynParameterData::Keyed {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
pub struct HirEagerParenateParameters(SmallVec<[HirEagerParenateParameter; 4]>);

impl std::ops::Deref for HirEagerParenateParameters {
    type Target = SmallVec<[HirEagerParenateParameter; 4]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl HirEagerParenateParameters {
    pub(crate) fn from_syn(
        syndicates: &[ParenateSynParameterData],
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
