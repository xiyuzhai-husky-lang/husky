use husky_eth_term::term::svar::EthSvar;
use husky_place::{place::Place, PlaceIdx};
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
        let new_place = match modifier {
            SvarModifier::Pure => FlyQuary::StackPure {
                place: engine.issue_new_stack_place_idx().into(),
            },
            SvarModifier::Owned => FlyQuary::ImmutableStackOwned {
                place: engine.issue_new_stack_place_idx().into(),
            },
            SvarModifier::Mut => todo!(),
            SvarModifier::Ref => todo!(),
            SvarModifier::RefMut => FlyQuary::RefMut {
                place: engine.issue_new_stack_place_idx().into(),
                lifetime: None,
            },
            SvarModifier::Const => FlyQuary::Const,
            SvarModifier::Ambersand(_) => todo!(),
            SvarModifier::AmbersandMut(_) => todo!(),
            SvarModifier::Le => todo!(),
            SvarModifier::Tilde => todo!(),
            SvarModifier::At => todo!(),
        };
        Self(ty.with_place(new_place))
    }

    pub fn new_variable_ty(
        engine: &mut impl FlyTermEngineMut,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
        modifier: SvarModifier,
        ty: FlyTerm,
    ) -> FlyTermResult<Self> {
        let new_place = match modifier {
            SvarModifier::Pure => match ty.place {
                Some(FlyQuary::Transient) | None => FlyQuary::ImmutableStackOwned {
                    place: engine.issue_new_stack_place_idx().into(),
                },
                Some(quary) => match ty.is_always_copyable(engine.db(), engine.fly_terms())? {
                    Some(true) => FlyQuary::ImmutableStackOwned {
                        place: engine.issue_new_stack_place_idx().into(),
                    },
                    Some(false) => match quary {
                        FlyQuary::Const => todo!(),
                        FlyQuary::StackPure { place }
                        | FlyQuary::ImmutableStackOwned { place }
                        | FlyQuary::MutableStackOwned { place } => {
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
                Some(FlyQuary::Transient) | None => FlyQuary::MutableStackOwned {
                    place: engine.issue_new_stack_place_idx().into(),
                },
                Some(place) => match ty.is_always_copyable(engine.db(), engine.fly_terms())? {
                    Some(true) => FlyQuary::MutableStackOwned {
                        place: engine.issue_new_stack_place_idx().into(),
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
        Ok(Self(ty.with_place(new_place)))
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
    /// - ImmutableStackOwned if base type is known to be copyable
    /// - ImmutableReferenced if base type is known to be noncopyable
    StackPure {
        place: Place,
    },
    /// lvalue nonreference
    ImmutableStackOwned {
        place: Place,
    },
    /// lvalue nonreference
    MutableStackOwned {
        place: Place,
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
        guard: Either<Place, FlyLifetime>,
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
        place: Place,
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
    pub(crate) fn bind(&self, contract: TermContract) -> FlyPlaceResult<()> {
        match (contract, self) {
            (TermContract::Const, FlyQuary::Const) => Ok(()),
            (TermContract::Const, _) => Err(FlyPlaceError::CannotConvertToConst),
            (TermContract::Leash, FlyQuary::Leashed) => Ok(()),
            (TermContract::Leash, _) => todo!("error"),
            (TermContract::Pure, _) => Ok(()),
            (TermContract::Move, FlyQuary::Const) => Ok(()),
            (TermContract::Move, FlyQuary::StackPure { place }) => Ok(()),
            (TermContract::Move, FlyQuary::ImmutableStackOwned { place }) => Ok(()),
            (TermContract::Move, FlyQuary::MutableStackOwned { place }) => Ok(()),
            (TermContract::Move, FlyQuary::Transient) => Ok(()),
            (TermContract::Move, FlyQuary::Ref { guard }) => Ok(()), // ad hoc
            (TermContract::Move, FlyQuary::RefMut { .. }) => todo!(),
            (TermContract::Move, FlyQuary::Leashed) => Ok(()),
            (TermContract::Move, FlyQuary::Todo) => todo!(),
            (TermContract::Borrow, FlyQuary::Const) => todo!(),
            (TermContract::Borrow, FlyQuary::StackPure { place }) => todo!(),
            (TermContract::Borrow, FlyQuary::ImmutableStackOwned { place }) => todo!(),
            (TermContract::Borrow, FlyQuary::MutableStackOwned { place }) => todo!(),
            (TermContract::Borrow, FlyQuary::Transient) => todo!(),
            (TermContract::Borrow, FlyQuary::Ref { guard }) => todo!(),
            (TermContract::Borrow, FlyQuary::RefMut { .. }) => todo!(),
            (TermContract::Borrow, FlyQuary::Leashed) => todo!(),
            (TermContract::Borrow, FlyQuary::Todo) => todo!(),
            (TermContract::BorrowMut, FlyQuary::Const) => todo!(),
            (TermContract::BorrowMut, FlyQuary::StackPure { place }) => todo!(),
            (TermContract::BorrowMut, FlyQuary::ImmutableStackOwned { place }) => todo!(),
            (TermContract::BorrowMut, FlyQuary::MutableStackOwned { place }) => todo!(),
            (TermContract::BorrowMut, FlyQuary::Transient) => Ok(()),
            (TermContract::BorrowMut, FlyQuary::Ref { guard }) => todo!(),
            (TermContract::BorrowMut, FlyQuary::RefMut { .. }) => Ok(()),
            (TermContract::BorrowMut, FlyQuary::Leashed) => todo!(),
            (TermContract::BorrowMut, FlyQuary::Todo) => todo!(),
            (TermContract::At, FlyQuary::Const) => todo!(),
            (TermContract::At, FlyQuary::StackPure { place }) => todo!(),
            (TermContract::At, FlyQuary::ImmutableStackOwned { place }) => todo!(),
            (TermContract::At, FlyQuary::MutableStackOwned { place }) => todo!(),
            (TermContract::At, FlyQuary::Transient) => todo!(),
            (TermContract::At, FlyQuary::Ref { guard }) => todo!(),
            (TermContract::At, FlyQuary::RefMut { .. }) => todo!(),
            (TermContract::At, FlyQuary::Leashed) => todo!(),
            (TermContract::At, FlyQuary::Todo) => todo!(),
            (TermContract::Move, FlyQuary::EtherealSymbol(_)) => todo!(),
            (TermContract::Borrow, FlyQuary::EtherealSymbol(_)) => todo!(),
            (TermContract::BorrowMut, FlyQuary::EtherealSymbol(_)) => todo!(),
            (TermContract::At, FlyQuary::EtherealSymbol(_)) => todo!(),
        }
    }

    pub fn place(self) -> Option<Place> {
        match self {
            FlyQuary::StackPure { place }
            | FlyQuary::ImmutableStackOwned { place }
            | FlyQuary::MutableStackOwned { place }
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
            FlyTermData::TypeOntology {
                ty_path,
                refined_ty_path,
                ty_arguments,
                ty_ethereal_term,
            } => todo!(),
            FlyTermData::Curry {
                toolchain,
                curry_kind,
                variance,
                parameter_hvar,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FlyTermData::Hole(_, _) => todo!(),
            FlyTermData::Sort(_) => todo!(),
            FlyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
            FlyTermData::Symbol { term, ty } => todo!(),
            FlyTermData::Hvar { .. } => todo!(),
            FlyTermData::TypeVariant { path } => todo!(),
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
