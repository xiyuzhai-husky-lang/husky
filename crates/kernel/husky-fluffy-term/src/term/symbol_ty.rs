use husky_stack_location::StackLocationIdx;
use thiserror::Error;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub struct SymbolType(FluffyTerm);

impl SymbolType {
    pub fn term(self) -> FluffyTerm {
        self.0
    }
}

impl Into<FluffyTerm> for SymbolType {
    fn into(self) -> FluffyTerm {
        self.term()
    }
}

impl SymbolType {
    #[inline(always)]
    pub fn new_parameter_ty_from_signature(
        engine: &mut impl FluffyTermEngine,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
        signature: SymbolSignature,
    ) -> FluffyTermResult<Self> {
        let ty = EtherealTerm::ty_from_declarative(engine.db(), signature.ty()?)?;
        Ok(Self::new_parameter_ty(
            engine,
            current_syn_symbol_idx,
            signature.modifier(),
            ty.into(),
        ))
    }

    pub fn new_parameter_ty(
        engine: &mut impl FluffyTermEngine,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
        modifier: SymbolModifier,
        ty: FluffyTerm,
    ) -> Self {
        let new_place = match modifier {
            SymbolModifier::Pure => FluffyPlace::StackPure {
                location: engine.issue_new_stack_location_idx(),
            },
            SymbolModifier::Owned => FluffyPlace::ImmutableStackOwned {
                location: engine.issue_new_stack_location_idx(),
            },
            SymbolModifier::Mut => todo!(),
            SymbolModifier::Ref => todo!(),
            SymbolModifier::RefMut => FluffyPlace::RefMut {
                guard: Left(engine.issue_new_stack_location_idx()),
            },
            SymbolModifier::Const => FluffyPlace::Const,
            SymbolModifier::Ambersand(_) => todo!(),
            SymbolModifier::AmbersandMut(_) => todo!(),
            SymbolModifier::Le => todo!(),
            SymbolModifier::Tilde => todo!(),
        };
        Self(ty.with_place(new_place))
    }

    pub fn new_variable_ty(
        engine: &mut impl FluffyTermEngine,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
        modifier: SymbolModifier,
        ty: FluffyTerm,
    ) -> FluffyTermResult<Self> {
        let new_place = match modifier {
            SymbolModifier::Pure => match ty.place {
                Some(FluffyPlace::Transient) | None => FluffyPlace::ImmutableStackOwned {
                    location: engine.issue_new_stack_location_idx(),
                },
                Some(place) => match ty.is_always_copyable(engine)? {
                    true => FluffyPlace::ImmutableStackOwned {
                        location: engine.issue_new_stack_location_idx(),
                    },
                    false => match place {
                        FluffyPlace::Const => todo!(),
                        FluffyPlace::StackPure { location }
                        | FluffyPlace::ImmutableStackOwned { location }
                        | FluffyPlace::MutableStackOwned { location } => FluffyPlace::Ref {
                            guard: Left(location),
                        },
                        FluffyPlace::Transient => unreachable!(),
                        FluffyPlace::Ref { guard } => todo!(),
                        FluffyPlace::RefMut { guard } => todo!(),
                        FluffyPlace::Leashed => todo!(),
                        FluffyPlace::Todo => todo!(),
                    },
                },
            },
            SymbolModifier::Owned => todo!(),
            SymbolModifier::Mut => match ty.place {
                Some(FluffyPlace::Transient) | None => FluffyPlace::MutableStackOwned {
                    location: engine.issue_new_stack_location_idx(),
                },
                Some(place) => match ty.is_always_copyable(engine)? {
                    true => FluffyPlace::MutableStackOwned {
                        location: engine.issue_new_stack_location_idx(),
                    },
                    false => {
                        p!(ty.show(engine.db(), engine.fluffy_terms()));
                        todo!()
                    }
                },
            },
            SymbolModifier::Ref => todo!(),
            SymbolModifier::RefMut => todo!(),
            SymbolModifier::Const => todo!(),
            SymbolModifier::Ambersand(_) => todo!(),
            SymbolModifier::AmbersandMut(_) => todo!(),
            SymbolModifier::Le => todo!(),
            SymbolModifier::Tilde => todo!(),
        };
        Ok(Self(ty.with_place(new_place)))
    }
}

/// maybe this is comparable with viewtype or viewt@ype in ATS?
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub struct PlaceTypeData {
    place: FluffyPlace,
    ty: EtherealTerm,
}

/// `PlaceQual` qualifies the place of a base type `T`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FluffyPlace {
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
        guard: Either<StackLocationIdx, FluffyLifetime>,
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
        guard: Either<StackLocationIdx, FluffyLifetime>,
    },
    /// stored in database
    /// always immutable
    Leashed,
    Todo,
}

impl FluffyPlace {
    pub(crate) fn bind(&self, contract: TermContract) -> FluffyPlaceResult<()> {
        match (contract, self) {
            (TermContract::Const, FluffyPlace::Const) => Ok(()),
            (TermContract::Const, _) => Err(FluffyPlaceError::CannotConvertToConst),
            (TermContract::Leash, FluffyPlace::Leashed) => Ok(()),
            (TermContract::Leash, _) => todo!("error"),
            (TermContract::Pure, _) => Ok(()),
            (TermContract::Move, FluffyPlace::Const) => Ok(()),
            (TermContract::Move, FluffyPlace::StackPure { location }) => Ok(()),
            (TermContract::Move, FluffyPlace::ImmutableStackOwned { location }) => Ok(()),
            (TermContract::Move, FluffyPlace::MutableStackOwned { location }) => Ok(()),
            (TermContract::Move, FluffyPlace::Transient) => Ok(()),
            (TermContract::Move, FluffyPlace::Ref { guard }) => Ok(()),
            (TermContract::Move, FluffyPlace::RefMut { guard }) => todo!(),
            (TermContract::Move, FluffyPlace::Leashed) => todo!(),
            (TermContract::Move, FluffyPlace::Todo) => todo!(),
            (TermContract::Borrow, FluffyPlace::Const) => todo!(),
            (TermContract::Borrow, FluffyPlace::StackPure { location }) => todo!(),
            (TermContract::Borrow, FluffyPlace::ImmutableStackOwned { location }) => todo!(),
            (TermContract::Borrow, FluffyPlace::MutableStackOwned { location }) => todo!(),
            (TermContract::Borrow, FluffyPlace::Transient) => todo!(),
            (TermContract::Borrow, FluffyPlace::Ref { guard }) => todo!(),
            (TermContract::Borrow, FluffyPlace::RefMut { guard }) => todo!(),
            (TermContract::Borrow, FluffyPlace::Leashed) => todo!(),
            (TermContract::Borrow, FluffyPlace::Todo) => todo!(),
            (TermContract::BorrowMut, FluffyPlace::Const) => todo!(),
            (TermContract::BorrowMut, FluffyPlace::StackPure { location }) => todo!(),
            (TermContract::BorrowMut, FluffyPlace::ImmutableStackOwned { location }) => todo!(),
            (TermContract::BorrowMut, FluffyPlace::MutableStackOwned { location }) => todo!(),
            (TermContract::BorrowMut, FluffyPlace::Transient) => Ok(()),
            (TermContract::BorrowMut, FluffyPlace::Ref { guard }) => todo!(),
            (TermContract::BorrowMut, FluffyPlace::RefMut { guard }) => Ok(()),
            (TermContract::BorrowMut, FluffyPlace::Leashed) => todo!(),
            (TermContract::BorrowMut, FluffyPlace::Todo) => todo!(),
            (TermContract::Const, FluffyPlace::Const) => todo!(),
            (TermContract::Const, FluffyPlace::StackPure { location }) => todo!(),
            (TermContract::Const, FluffyPlace::ImmutableStackOwned { location }) => todo!(),
            (TermContract::Const, FluffyPlace::MutableStackOwned { location }) => todo!(),
            (TermContract::Const, FluffyPlace::Transient) => todo!(),
            (TermContract::Const, FluffyPlace::Ref { guard }) => todo!(),
            (TermContract::Const, FluffyPlace::RefMut { guard }) => todo!(),
            (TermContract::Const, FluffyPlace::Leashed) => todo!(),
            (TermContract::Const, FluffyPlace::Todo) => todo!(),
            (TermContract::Leash, FluffyPlace::Const) => todo!(),
            (TermContract::Leash, FluffyPlace::StackPure { location }) => todo!(),
            (TermContract::Leash, FluffyPlace::ImmutableStackOwned { location }) => todo!(),
            (TermContract::Leash, FluffyPlace::MutableStackOwned { location }) => todo!(),
            (TermContract::Leash, FluffyPlace::Transient) => todo!(),
            (TermContract::Leash, FluffyPlace::Ref { guard }) => todo!(),
            (TermContract::Leash, FluffyPlace::RefMut { guard }) => todo!(),
            (TermContract::Leash, FluffyPlace::Leashed) => todo!(),
            (TermContract::Leash, FluffyPlace::Todo) => todo!(),
            (TermContract::At, FluffyPlace::Const) => todo!(),
            (TermContract::At, FluffyPlace::StackPure { location }) => todo!(),
            (TermContract::At, FluffyPlace::ImmutableStackOwned { location }) => todo!(),
            (TermContract::At, FluffyPlace::MutableStackOwned { location }) => todo!(),
            (TermContract::At, FluffyPlace::Transient) => todo!(),
            (TermContract::At, FluffyPlace::Ref { guard }) => todo!(),
            (TermContract::At, FluffyPlace::RefMut { guard }) => todo!(),
            (TermContract::At, FluffyPlace::Leashed) => todo!(),
            (TermContract::At, FluffyPlace::Todo) => todo!(),
        }
    }
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum FluffyPlaceError {
    #[error("cannot convert to const")]
    CannotConvertToConst,
}

pub type FluffyPlaceResult<T> = Result<T, FluffyPlaceError>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FluffyLifetime {
    StaticLifetime,
}

impl FluffyLifetime {
    pub(crate) fn from_term(term: FluffyTerm, db: &::salsa::Db, terms: &mut FluffyTerms) -> Self {
        match term.data_inner(db, terms) {
            FluffyTermData::Literal(lit) => match lit {
                TermLiteral::StaticLifetime => FluffyLifetime::StaticLifetime,
                _ => todo!(),
            },
            FluffyTermData::TypeOntology {
                ty_path,
                refined_ty_path,
                ty_arguments,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
            FluffyTermData::Symbol { term, ty } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FluffyLifetimeIdx {}

// #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
// pub struct PlaceTypeIdx(FluffyTermIdx);

// impl Into<FluffyTerm> for PlaceTypeIdx {
//     fn into(self) -> FluffyTerm {
//         self.0.into()
//     }
// }
