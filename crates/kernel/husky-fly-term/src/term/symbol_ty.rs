use husky_eth_term::term::symbol::EthSymbol;
use husky_stack_location::StackLocationIdx;
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
        signature: DecSymbolSignature,
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
        modifier: SymbolModifier,
        ty: FlyTerm,
    ) -> Self {
        let new_place = match modifier {
            SymbolModifier::Pure => FlyPlace::StackPure {
                location: engine.issue_new_stack_location_idx(),
            },
            SymbolModifier::Owned => FlyPlace::ImmutableStackOwned {
                location: engine.issue_new_stack_location_idx(),
            },
            SymbolModifier::Mut => todo!(),
            SymbolModifier::Ref => todo!(),
            SymbolModifier::RefMut => FlyPlace::RefMut {
                guard: Left(engine.issue_new_stack_location_idx()),
            },
            SymbolModifier::Const => FlyPlace::Const,
            SymbolModifier::Ambersand(_) => todo!(),
            SymbolModifier::AmbersandMut(_) => todo!(),
            SymbolModifier::Le => todo!(),
            SymbolModifier::Tilde => todo!(),
            SymbolModifier::At => todo!(),
        };
        Self(ty.with_place(new_place))
    }

    pub fn new_variable_ty(
        engine: &mut impl FlyTermEngineMut,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
        modifier: SymbolModifier,
        ty: FlyTerm,
    ) -> FlyTermResult<Self> {
        let new_place = match modifier {
            SymbolModifier::Pure => match ty.place {
                Some(FlyPlace::Transient) | None => FlyPlace::ImmutableStackOwned {
                    location: engine.issue_new_stack_location_idx(),
                },
                Some(place) => match ty.is_always_copyable(engine.db(), engine.fluffy_terms())? {
                    Some(true) => FlyPlace::ImmutableStackOwned {
                        location: engine.issue_new_stack_location_idx(),
                    },
                    Some(false) => match place {
                        FlyPlace::Const => todo!(),
                        FlyPlace::StackPure { location }
                        | FlyPlace::ImmutableStackOwned { location }
                        | FlyPlace::MutableStackOwned { location } => FlyPlace::Ref {
                            guard: Left(location),
                        },
                        FlyPlace::Transient => unreachable!(),
                        FlyPlace::Ref { guard } => todo!(),
                        FlyPlace::RefMut { guard } => todo!(),
                        FlyPlace::Leashed => FlyPlace::Leashed,
                        FlyPlace::Todo => todo!(),
                        FlyPlace::EtherealSymbol(_) => todo!(),
                    },
                    None => todo!(),
                },
            },
            SymbolModifier::Owned => todo!(),
            SymbolModifier::Mut => match ty.place {
                Some(FlyPlace::Transient) | None => FlyPlace::MutableStackOwned {
                    location: engine.issue_new_stack_location_idx(),
                },
                Some(place) => match ty.is_always_copyable(engine.db(), engine.fluffy_terms())? {
                    Some(true) => FlyPlace::MutableStackOwned {
                        location: engine.issue_new_stack_location_idx(),
                    },
                    Some(false) => {
                        p!(ty.show(engine.db(), engine.fluffy_terms()));
                        todo!()
                    }
                    None => todo!(),
                },
            },
            SymbolModifier::Ref => todo!(),
            SymbolModifier::RefMut => todo!(),
            SymbolModifier::Const => todo!(),
            SymbolModifier::Ambersand(_) => todo!(),
            SymbolModifier::AmbersandMut(_) => todo!(),
            SymbolModifier::Le => todo!(),
            SymbolModifier::Tilde => todo!(),
            SymbolModifier::At => todo!(),
        };
        Ok(Self(ty.with_place(new_place)))
    }
}

/// maybe this is comparable with viewtype or viewt@ype in ATS?
#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PlaceTypeData {
    place: FlyPlace,
    ty: EthTerm,
}

/// `PlaceQual` qualifies the place of a base type `T`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlyPlace {
    Const,
    /// reduce to
    /// - ImmutableStackOwned if base type is known to be copyable
    /// - ImmutableReferenced if base type is known to be noncopyable
    StackPure {
        location: StackLocationIdx,
    },
    /// lvalue nonreference
    ImmutableStackOwned {
        location: StackLocationIdx,
    },
    /// lvalue nonreference
    MutableStackOwned {
        location: StackLocationIdx,
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
        guard: Either<StackLocationIdx, FlyLifetime>,
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
        guard: Either<StackLocationIdx, FlyLifetime>,
    },
    /// stored in database
    /// always immutable
    Leashed,
    Todo,
    EtherealSymbol(EthSymbol),
}

impl FlyPlace {
    pub(crate) fn bind(&self, contract: TermContract) -> FlyPlaceResult<()> {
        match (contract, self) {
            (TermContract::Const, FlyPlace::Const) => Ok(()),
            (TermContract::Const, _) => Err(FlyPlaceError::CannotConvertToConst),
            (TermContract::Leash, FlyPlace::Leashed) => Ok(()),
            (TermContract::Leash, _) => todo!("error"),
            (TermContract::Pure, _) => Ok(()),
            (TermContract::Move, FlyPlace::Const) => Ok(()),
            (TermContract::Move, FlyPlace::StackPure { location }) => Ok(()),
            (TermContract::Move, FlyPlace::ImmutableStackOwned { location }) => Ok(()),
            (TermContract::Move, FlyPlace::MutableStackOwned { location }) => Ok(()),
            (TermContract::Move, FlyPlace::Transient) => Ok(()),
            (TermContract::Move, FlyPlace::Ref { guard }) => Ok(()), // ad hoc
            (TermContract::Move, FlyPlace::RefMut { guard }) => todo!(),
            (TermContract::Move, FlyPlace::Leashed) => Ok(()),
            (TermContract::Move, FlyPlace::Todo) => todo!(),
            (TermContract::Borrow, FlyPlace::Const) => todo!(),
            (TermContract::Borrow, FlyPlace::StackPure { location }) => todo!(),
            (TermContract::Borrow, FlyPlace::ImmutableStackOwned { location }) => todo!(),
            (TermContract::Borrow, FlyPlace::MutableStackOwned { location }) => todo!(),
            (TermContract::Borrow, FlyPlace::Transient) => todo!(),
            (TermContract::Borrow, FlyPlace::Ref { guard }) => todo!(),
            (TermContract::Borrow, FlyPlace::RefMut { guard }) => todo!(),
            (TermContract::Borrow, FlyPlace::Leashed) => todo!(),
            (TermContract::Borrow, FlyPlace::Todo) => todo!(),
            (TermContract::BorrowMut, FlyPlace::Const) => todo!(),
            (TermContract::BorrowMut, FlyPlace::StackPure { location }) => todo!(),
            (TermContract::BorrowMut, FlyPlace::ImmutableStackOwned { location }) => todo!(),
            (TermContract::BorrowMut, FlyPlace::MutableStackOwned { location }) => todo!(),
            (TermContract::BorrowMut, FlyPlace::Transient) => Ok(()),
            (TermContract::BorrowMut, FlyPlace::Ref { guard }) => todo!(),
            (TermContract::BorrowMut, FlyPlace::RefMut { guard }) => Ok(()),
            (TermContract::BorrowMut, FlyPlace::Leashed) => todo!(),
            (TermContract::BorrowMut, FlyPlace::Todo) => todo!(),
            (TermContract::At, FlyPlace::Const) => todo!(),
            (TermContract::At, FlyPlace::StackPure { location }) => todo!(),
            (TermContract::At, FlyPlace::ImmutableStackOwned { location }) => todo!(),
            (TermContract::At, FlyPlace::MutableStackOwned { location }) => todo!(),
            (TermContract::At, FlyPlace::Transient) => todo!(),
            (TermContract::At, FlyPlace::Ref { guard }) => todo!(),
            (TermContract::At, FlyPlace::RefMut { guard }) => todo!(),
            (TermContract::At, FlyPlace::Leashed) => todo!(),
            (TermContract::At, FlyPlace::Todo) => todo!(),
            (TermContract::Move, FlyPlace::EtherealSymbol(_)) => todo!(),
            (TermContract::Borrow, FlyPlace::EtherealSymbol(_)) => todo!(),
            (TermContract::BorrowMut, FlyPlace::EtherealSymbol(_)) => todo!(),
            (TermContract::At, FlyPlace::EtherealSymbol(_)) => todo!(),
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
                parameter_rune,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FlyTermData::Hole(_, _) => todo!(),
            FlyTermData::Category(_) => todo!(),
            FlyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
            FlyTermData::Symbol { term, ty } => todo!(),
            FlyTermData::Rune { .. } => todo!(),
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
