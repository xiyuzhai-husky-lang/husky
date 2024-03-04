use husky_eth_term::term::svar::EthSvar;
use husky_place::{place::EthPlace, PlaceInfo};
use thiserror::Error;

use super::*;

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

/// maybe this is comparable with viewtype or viewt@ype in ATS?
#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PlaceTypeData {
    qualified_place: FlyQuary,
    ty: EthTerm,
}

/// `PlaceQual` qualifies the place of a base type `T`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlyQuary {
    Const,
    /// reduce to
    /// - ImmutableOnStack if base type is known to be copyable
    /// - ImmutableReferenced if base type is known to be noncopyable
    StackPure {
        place: EthPlace,
    },
    /// lvalue nonreference
    ImmutableOnStack {
        place: EthPlace,
    },
    /// lvalue nonreference
    MutableOnStack {
        place: EthPlace,
    },
    // rvalue
    Transient,
    /// a place accessed through ref
    ///
    /// can be converted to
    /// - `&'a T`;
    ///
    ///     If guard is `Left(stack_location_idx)`
    ///     then `'a` is the time that location is borrowed;
    ///     else `'a` is equal to the lifetime of that guard.
    /// - `T` when `T` is copyable
    Ref {
        /// Guard is overwritten when composed with references.
        ///
        /// To see this, consider the following code
        ///
        /// ```husky
        /// struct A<'a> { x: &'a []i32}
        /// ```
        ///
        /// let `a` be a reference to `A<'b>`, then `a.x` is a valid for `'b` time,
        /// even if `a` is short lived.
        guard: Either<EthPlace, FlyLifetime>,
    },
    /// a place accessed through ref mut
    ///
    /// can be converted to
    /// - `&'a mut T`;
    ///
    ///     If guard is `Left(stack_location_idx)`
    ///     then `'a` is the time that location is borrowed;
    ///     else `'a` is equal to the lifetime of that guard.
    /// - `&'a T`;
    ///
    ///     If guard is `Left(stack_location_idx)`
    ///     then `'a` is the time that location is borrowed;
    ///     else `'a` is equal to the lifetime of that guard.
    /// - `T` when `T` is copyable
    RefMut {
        /// Guard is not overwritten when composed with references
        ///
        /// To see this, consider the following code
        ///
        /// ```husky
        /// struct A<'a> { mut x: &'a []i32}
        /// ```
        ///
        /// If `a` is a mutable reference of lifetime `'a` to `A<'b>`, then `a.x` is valid for `'a` time,
        /// even if `b` is long lived. So we should only care about the first lifetime.
        ///
        /// If `a` is a mutable variable on stack of type `A<'b>`, then `a.x` is valid as long as `a` is valid,
        /// even if `b` is long lived. So we should only care about the stack location.
        place: EthPlace,
        lifetime: Option<FlyLifetime>,
    },
    /// stored in database
    /// always immutable
    Leashed,
    Todo,
    #[deprecated(note = "consider more carefully")]
    EtherealSymbol(EthSvar),
}

impl FlyQuary {
    pub(crate) fn bind(&self, contract: Contract) -> FlyPlaceResult<()> {
        match (contract, self) {
            (Contract::Const, FlyQuary::Const) => Ok(()),
            (Contract::Const, _) => Err(FlyPlaceError::CannotConvertToConst),
            (Contract::Leash, FlyQuary::Leashed) => Ok(()),
            (Contract::Leash, _) => todo!("error"),
            (Contract::Pure, _) => Ok(()),
            (Contract::Move, FlyQuary::Const) => Ok(()),
            (Contract::Move, FlyQuary::StackPure { place }) => Ok(()),
            (Contract::Move, FlyQuary::ImmutableOnStack { place }) => Ok(()),
            (Contract::Move, FlyQuary::MutableOnStack { place }) => Ok(()),
            (Contract::Move, FlyQuary::Transient) => Ok(()),
            (Contract::Move, FlyQuary::Ref { guard }) => Ok(()), // ad hoc
            (Contract::Move, FlyQuary::RefMut { .. }) => todo!(),
            (Contract::Move, FlyQuary::Leashed) => Ok(()),
            (Contract::Move, FlyQuary::Todo) => todo!(),
            (Contract::Borrow, FlyQuary::Const) => todo!(),
            (Contract::Borrow, FlyQuary::StackPure { place }) => todo!(),
            (Contract::Borrow, FlyQuary::ImmutableOnStack { place }) => todo!(),
            (Contract::Borrow, FlyQuary::MutableOnStack { place }) => todo!(),
            (Contract::Borrow, FlyQuary::Transient) => todo!(),
            (Contract::Borrow, FlyQuary::Ref { guard }) => todo!(),
            (Contract::Borrow, FlyQuary::RefMut { .. }) => todo!(),
            (Contract::Borrow, FlyQuary::Leashed) => todo!(),
            (Contract::Borrow, FlyQuary::Todo) => todo!(),
            (Contract::BorrowMut, FlyQuary::Const) => todo!(),
            (Contract::BorrowMut, FlyQuary::StackPure { place }) => todo!(),
            (Contract::BorrowMut, FlyQuary::ImmutableOnStack { place }) => todo!(),
            (Contract::BorrowMut, FlyQuary::MutableOnStack { place }) => todo!(),
            (Contract::BorrowMut, FlyQuary::Transient) => Ok(()),
            (Contract::BorrowMut, FlyQuary::Ref { guard }) => todo!(),
            (Contract::BorrowMut, FlyQuary::RefMut { .. }) => Ok(()),
            (Contract::BorrowMut, FlyQuary::Leashed) => todo!(),
            (Contract::BorrowMut, FlyQuary::Todo) => todo!(),
            (Contract::At, FlyQuary::Const) => todo!(),
            (Contract::At, FlyQuary::StackPure { place }) => todo!(),
            (Contract::At, FlyQuary::ImmutableOnStack { place }) => todo!(),
            (Contract::At, FlyQuary::MutableOnStack { place }) => todo!(),
            (Contract::At, FlyQuary::Transient) => todo!(),
            (Contract::At, FlyQuary::Ref { guard }) => todo!(),
            (Contract::At, FlyQuary::RefMut { .. }) => todo!(),
            (Contract::At, FlyQuary::Leashed) => todo!(),
            (Contract::At, FlyQuary::Todo) => todo!(),
            (Contract::Move, FlyQuary::EtherealSymbol(_)) => todo!(),
            (Contract::Borrow, FlyQuary::EtherealSymbol(_)) => todo!(),
            (Contract::BorrowMut, FlyQuary::EtherealSymbol(_)) => todo!(),
            (Contract::At, FlyQuary::EtherealSymbol(_)) => todo!(),
        }
    }

    pub fn place(self) -> Option<EthPlace> {
        match self {
            FlyQuary::StackPure { place }
            | FlyQuary::ImmutableOnStack { place }
            | FlyQuary::MutableOnStack { place }
            | FlyQuary::Ref { guard: Left(place) }
            | FlyQuary::RefMut { place, .. } => Some(place),
            FlyQuary::EtherealSymbol(svar) => Some(svar.into()),
            FlyQuary::Const
            | FlyQuary::Transient
            | FlyQuary::Leashed
            | FlyQuary::Todo
            | FlyQuary::Ref { guard: Right(_) } => None,
        }
    }
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum FlyPlaceError {
    #[error("cannot convert to const")]
    CannotConvertToConst,
}

pub type FlyPlaceResult<T> = Result<T, FlyPlaceError>;

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
