use super::*;
use husky_hir_lazy_expr::HirLazyPatternExprIdx;
use husky_syn_expr::SynVariadicParameterVariant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirLazyParenateParameter {
    SelfValue,
    Ordinary {
        pattern_expr_idx: HirLazyPatternExprIdx,
        ty: HirType,
    },
    Keyed {
        ident: Ident,
        ty: HirType,
    },
    Variadic {
        variant: HirLazyParenateParameterVariadicVariant,
        ty: HirType,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirLazyParenateParameterVariadicVariant {
    Vec,
}

impl From<&SynVariadicParameterVariant> for HirLazyParenateParameterVariadicVariant {
    fn from(value: &SynVariadicParameterVariant) -> Self {
        match value {
            SynVariadicParameterVariant::Default => todo!(),
            SynVariadicParameterVariant::Vec { .. } => HirLazyParenateParameterVariadicVariant::Vec,
        }
    }
}

impl HirLazyParenateParameter {
    pub(crate) fn from_self_value_parameter_syndicate(
        _syndicate: Option<SelfValueParameterSyndicate>,
        _db: &::salsa::Db,
    ) -> Self {
        HirLazyParenateParameter::SelfValue
    }

    pub(crate) fn from_syn(
        syndicate: &ParenateSynParameterData,
        builder: &HirDeclBuilder,
    ) -> Option<Self> {
        Some(match *syndicate {
            ParenateSynParameterData::Ordinary {
                syn_pattern_root,
                ty,
                ..
            } => HirLazyParenateParameter::Ordinary {
                pattern_expr_idx: builder.hir_lazy_pattern_expr_idx(syn_pattern_root),
                ty: builder.hir_ty(ty).unwrap(),
            },
            ParenateSynParameterData::Variadic {
                ref variadic_variant,
                ty,
                ..
            } => HirLazyParenateParameter::Variadic {
                variant: variadic_variant.into(),
                ty: builder.hir_ty(ty).unwrap(),
            },
            ParenateSynParameterData::Keyed {
                ident_token, ty, ..
            } => HirLazyParenateParameter::Keyed {
                ident: ident_token.ident(),
                ty: builder.hir_ty(ty).unwrap(),
            },
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
