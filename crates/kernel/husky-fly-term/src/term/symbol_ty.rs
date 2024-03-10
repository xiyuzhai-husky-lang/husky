use super::*;
use husky_place::PlaceInfo;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SymbolType(FlyTerm);

impl SymbolType {
    pub fn term(self) -> FlyTerm {
        self.0
    }
}

impl Into<FlyTerm> for SymbolType {
    fn into(self) -> FlyTerm {
        self.term()
    }
}

impl SymbolType {
    #[inline(always)]
    pub fn new_parameter_ty_from_signature(
        engine: &mut impl FlyTermEngineMut,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
        signature: DecSvarSignature,
    ) -> FlyTermResult<Self> {
        let ty = EthTerm::ty_from_dec(engine.db(), signature.ty()?)?;
        Ok(Self::new_parameter_ty(
            engine,
            current_syn_symbol_idx,
            signature.modifier(),
            ty.into(),
        ))
    }

    pub fn new_parameter_ty(
        engine: &mut impl FlyTermEngineMut,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
        modifier: SvarModifier,
        ty: FlyTerm,
    ) -> Self {
        let place_data = || {
            let Some(ident) = engine.syn_expr_region_data()[current_syn_symbol_idx].ident() else {
                let db = engine.db();
                p!(engine.syn_expr_region_data()[current_syn_symbol_idx]
                    .name()
                    .debug(db));
                unreachable!();
            };
            PlaceInfo::Parameter {
                current_syn_symbol_idx,
                ident,
            }
        };
        let quary = match modifier {
            SvarModifier::Pure => FlyQuary::StackPure {
                place: engine.issue_new_place_idx(place_data()).into(),
            },
            SvarModifier::Owned => FlyQuary::ImmutableOnStack {
                place: engine.issue_new_place_idx(place_data()).into(),
            },
            SvarModifier::Mut => todo!(),
            SvarModifier::Ref => todo!(),
            SvarModifier::RefMut => FlyQuary::RefMut {
                place: engine.issue_new_place_idx(place_data()).into(),
                lifetime: None,
            },
            SvarModifier::Const => FlyQuary::Const,
            SvarModifier::Ambersand(_) => todo!(),
            SvarModifier::AmbersandMut(_) => todo!(),
            SvarModifier::Le => todo!(),
            SvarModifier::Tilde => todo!(),
            SvarModifier::At => todo!(),
        };
        Self(ty.with_quary(quary))
    }

    pub fn new_variable_ty(
        engine: &mut impl FlyTermEngineMut,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
        modifier: SvarModifier,
        ty: FlyTerm,
    ) -> FlyTermResult<Self> {
        let ident = engine.syn_expr_region_data()[current_syn_symbol_idx]
            .ident()
            .unwrap();
        let place_data = PlaceInfo::Variable {
            current_syn_symbol_idx,
            ident,
        };
        let quary = match modifier {
            SvarModifier::Pure => match ty.place {
                Some(FlyQuary::Transient) | None => FlyQuary::ImmutableOnStack {
                    place: engine.issue_new_place_idx(place_data).into(),
                },
                Some(quary) => match ty.is_always_copyable(engine.db(), engine.fly_terms())? {
                    Some(true) => FlyQuary::ImmutableOnStack {
                        place: engine.issue_new_place_idx(place_data).into(),
                    },
                    Some(false) => match quary {
                        FlyQuary::Const => todo!(),
                        FlyQuary::StackPure { place }
                        | FlyQuary::ImmutableOnStack { place }
                        | FlyQuary::MutableOnStack { place } => {
                            FlyQuary::Ref { guard: Left(place) }
                        }
                        FlyQuary::Transient => unreachable!(),
                        FlyQuary::Ref { guard } => todo!(),
                        FlyQuary::RefMut { .. } => todo!(),
                        FlyQuary::Leashed => FlyQuary::Leashed,
                        FlyQuary::Todo => todo!(),
                        FlyQuary::EtherealSymbol(_) => todo!(),
                    },
                    None => todo!(),
                },
            },
            SvarModifier::Owned => todo!(),
            SvarModifier::Mut => match ty.place {
                Some(FlyQuary::Transient) | None => FlyQuary::MutableOnStack {
                    place: engine.issue_new_place_idx(place_data).into(),
                },
                Some(place) => match ty.is_always_copyable(engine.db(), engine.fly_terms())? {
                    Some(true) => FlyQuary::MutableOnStack {
                        place: engine.issue_new_place_idx(place_data).into(),
                    },
                    Some(false) => {
                        p!(ty.show(engine.db(), engine.fly_terms()));
                        todo!()
                    }
                    None => todo!(),
                },
            },
            SvarModifier::Ref => todo!(),
            SvarModifier::RefMut => todo!(),
            SvarModifier::Const => todo!(),
            SvarModifier::Ambersand(_) => todo!(),
            SvarModifier::AmbersandMut(_) => todo!(),
            SvarModifier::Le => todo!(),
            SvarModifier::Tilde => todo!(),
            SvarModifier::At => todo!(),
        };
        Ok(Self(ty.with_quary(quary)))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FlyLifetime {
    StaticLifetime,
}

impl FlyLifetime {
    pub(crate) fn from_term(term: FlyTerm, db: &::salsa::Db, terms: &mut FlyTerms) -> Self {
        match term.data_inner(db, terms) {
            FlyTermData::Literal(lit) => match lit {
                Literal::StaticLifetime => FlyLifetime::StaticLifetime,
                _ => todo!(),
            },
            FlyTermData::TypeOntology { .. } => todo!(),
            FlyTermData::Curry { .. } => todo!(),
            FlyTermData::Hole(_, _) => todo!(),
            FlyTermData::Sort(_) => todo!(),
            FlyTermData::Ritchie { .. } => todo!(),
            FlyTermData::Symbol { .. } => todo!(),
            FlyTermData::Hvar { .. } => todo!(),
            FlyTermData::TypeVariant { .. } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FlyLifetimeIdx {}

// #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
// pub struct PlaceTypeIdx(FlyTermIdx);

// impl Into<FlyTerm> for PlaceTypeIdx {
//     fn into(self) -> FlyTerm {
//         self.0.into()
//     }
// }
