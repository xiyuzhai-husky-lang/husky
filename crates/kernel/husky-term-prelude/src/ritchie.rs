use crate::Contract;
use husky_entity_kind::ritchie::RitchieItemKind;

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum RitchieKind {
    Type(RitchieTypeKind),
    Trait(RitchieTraitKind),
}

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
                RitchieClosureKind::Tn => todo!(),
            },
        }
    }
}

impl std::fmt::Display for RitchieTypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum RitchieClosureKind {
    Fn,
    Gn,
    Vn,
    Pn,
    Qn,
    Tn,
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
                    RitchieItemKind::Tn => "tn(",
                },
                RitchieTypeKind::Closure(ritchie_closure_kind) => match ritchie_closure_kind {
                    RitchieClosureKind::Fn => "#closure fn(",
                    RitchieClosureKind::Gn => "#closure gn(",
                    RitchieClosureKind::Vn => "#closure vn(",
                    RitchieClosureKind::Pn => "#closure pn(",
                    RitchieClosureKind::Qn => "#closure qn(",
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum RitchieTraitKind {
    Fn,
    FnMut,
    FnOnce,
    Gn,
}
