use crate::Contract;
use husky_entity_kind::ritchie::RitchieItemKind;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum RitchieKind {
    Type(RitchieTypeKind),
    Trait(RitchieTraitKind),
}

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum RitchieTypeKind {
    // todo: add item path
    Item(RitchieItemKind),
    // todo: each closure should be unique
    Closure(RitchieClosureKind),
}

impl RitchieTypeKind {
    pub fn function_contract(self) -> Contract {
        match self {
            RitchieTypeKind::Item(ritchie_item_kind) => Contract::Pure,
            RitchieTypeKind::Closure(ritchie_closure_kind) => match ritchie_closure_kind {
                RitchieClosureKind::Fn => todo!(),
                RitchieClosureKind::Gn => todo!(),
                RitchieClosureKind::Vn => todo!(),
                RitchieClosureKind::Pn => todo!(),
                RitchieClosureKind::Qn => todo!(),
                RitchieClosureKind::Bn => todo!(),
                RitchieClosureKind::Sn => todo!(),
                RitchieClosureKind::Tn => todo!(),
            },
        }
    }

    pub fn display_str(self) -> &'static str {
        match self {
            RitchieTypeKind::Item(slf) => slf.display_str(),
            RitchieTypeKind::Closure(slf) => slf.display_str(),
        }
    }
}

impl std::fmt::Display for RitchieTypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display_str().fmt(f)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum RitchieClosureKind {
    Fn,
    Gn,
    Vn,
    Pn,
    Qn,
    Bn,
    Sn,
    Tn,
}
impl RitchieClosureKind {
    fn display_str(self) -> &'static str {
        match self {
            RitchieClosureKind::Fn => "{closure} Fn",
            RitchieClosureKind::Gn => "{closure} Gn",
            RitchieClosureKind::Vn => "{closure} Vn",
            RitchieClosureKind::Pn => "{closure} Pn",
            RitchieClosureKind::Qn => "{closure} Qn",
            RitchieClosureKind::Bn => "{closure} Bn",
            RitchieClosureKind::Sn => "{closure} Sn",
            RitchieClosureKind::Tn => "{closure} Tn",
        }
    }
}

impl From<RitchieItemKind> for RitchieKind {
    fn from(kind: RitchieItemKind) -> Self {
        RitchieKind::Type(kind.into())
    }
}

impl From<RitchieClosureKind> for RitchieKind {
    fn from(kind: RitchieClosureKind) -> Self {
        RitchieKind::Type(kind.into())
    }
}

impl RitchieKind {
    pub const RITCHIE_TYPE_FN: Self = RitchieKind::Type(RitchieTypeKind::Item(RitchieItemKind::Fn));
    pub const RITCHIE_TYPE_GN: Self = RitchieKind::Type(RitchieTypeKind::Item(RitchieItemKind::Gn));
    pub const RITCHIE_TYPE_VN: Self = RitchieKind::Type(RitchieTypeKind::Item(RitchieItemKind::Vn));
    pub const RITCHIE_TYPE_PN: Self = RitchieKind::Type(RitchieTypeKind::Item(RitchieItemKind::Pn));
    pub const RITCHIE_TYPE_QN: Self = RitchieKind::Type(RitchieTypeKind::Item(RitchieItemKind::Qn));
    pub const RITCHIE_TYPE_TN: Self = RitchieKind::Type(RitchieTypeKind::Item(RitchieItemKind::Tn));

    pub fn code(self) -> &'static str {
        match self {
            RitchieKind::Type(ritchie_ty_kind) => match ritchie_ty_kind {
                RitchieTypeKind::Item(ritchie_item_kind) => match ritchie_item_kind {
                    RitchieItemKind::Fn => "fn(",
                    RitchieItemKind::Gn => "gn(",
                    RitchieItemKind::Vn => "vn(",
                    RitchieItemKind::Pn => "pn(",
                    RitchieItemKind::Qn => "qn(",
                    RitchieItemKind::Bn => "bn(",
                    RitchieItemKind::Sn => "sn(",
                    RitchieItemKind::Tn => "tn(",
                },
                RitchieTypeKind::Closure(ritchie_closure_kind) => match ritchie_closure_kind {
                    RitchieClosureKind::Fn => "#closure fn(",
                    RitchieClosureKind::Gn => "#closure gn(",
                    RitchieClosureKind::Vn => "#closure vn(",
                    RitchieClosureKind::Pn => "#closure pn(",
                    RitchieClosureKind::Qn => "#closure qn(",
                    RitchieClosureKind::Bn => "#closure bn(",
                    RitchieClosureKind::Sn => "#closure sn(",
                    RitchieClosureKind::Tn => "#closure tn(",
                },
            },
            RitchieKind::Trait(ritchie_trai_kind) => match ritchie_trai_kind {
                RitchieTraitKind::Fn => "Fn(",
                RitchieTraitKind::FnMut => "FnMut(",
                RitchieTraitKind::FnOnce => "FnOnce(",
                RitchieTraitKind::Gn => "Gn(",
            },
        }
    }

    pub fn ritchie_ty_kind(self) -> Option<RitchieTypeKind> {
        match self {
            RitchieKind::Type(ritchie_ty_kind) => Some(ritchie_ty_kind),
            RitchieKind::Trait(_) => None,
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum RitchieTraitKind {
    Fn,
    FnMut,
    FnOnce,
    Gn,
}
