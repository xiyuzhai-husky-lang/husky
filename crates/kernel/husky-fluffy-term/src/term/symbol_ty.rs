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
    pub(crate) fn bind(&self, contract: Contract) -> FluffyPlaceResult<()> {
        match (contract, self) {
            (Contract::Const, FluffyPlace::Const) => Ok(()),
            (Contract::Const, _) => Err(FluffyPlaceError::CannotConvertToConst),
            (Contract::Leash, FluffyPlace::Leashed) => Ok(()),
            (Contract::Leash, _) => todo!("error"),
            (Contract::Pure, _) => Ok(()),
            (Contract::Move, FluffyPlace::Const) => Ok(()),
            (Contract::Move, FluffyPlace::StackPure { location }) => Ok(()),
            (Contract::Move, FluffyPlace::ImmutableStackOwned { location }) => Ok(()),
            (Contract::Move, FluffyPlace::MutableStackOwned { location }) => Ok(()),
            (Contract::Move, FluffyPlace::Transient) => Ok(()),
            (Contract::Move, FluffyPlace::Ref { guard }) => Ok(()),
            (Contract::Move, FluffyPlace::RefMut { guard }) => todo!(),
            (Contract::Move, FluffyPlace::Leashed) => todo!(),
            (Contract::Move, FluffyPlace::Todo) => todo!(),
            (Contract::Borrow, FluffyPlace::Const) => todo!(),
            (Contract::Borrow, FluffyPlace::StackPure { location }) => todo!(),
            (Contract::Borrow, FluffyPlace::ImmutableStackOwned { location }) => todo!(),
            (Contract::Borrow, FluffyPlace::MutableStackOwned { location }) => todo!(),
            (Contract::Borrow, FluffyPlace::Transient) => todo!(),
            (Contract::Borrow, FluffyPlace::Ref { guard }) => todo!(),
            (Contract::Borrow, FluffyPlace::RefMut { guard }) => todo!(),
            (Contract::Borrow, FluffyPlace::Leashed) => todo!(),
            (Contract::Borrow, FluffyPlace::Todo) => todo!(),
            (Contract::BorrowMut, FluffyPlace::Const) => todo!(),
            (Contract::BorrowMut, FluffyPlace::StackPure { location }) => todo!(),
            (Contract::BorrowMut, FluffyPlace::ImmutableStackOwned { location }) => todo!(),
            (Contract::BorrowMut, FluffyPlace::MutableStackOwned { location }) => todo!(),
            (Contract::BorrowMut, FluffyPlace::Transient) => Ok(()),
            (Contract::BorrowMut, FluffyPlace::Ref { guard }) => todo!(),
            (Contract::BorrowMut, FluffyPlace::RefMut { guard }) => Ok(()),
            (Contract::BorrowMut, FluffyPlace::Leashed) => todo!(),
            (Contract::BorrowMut, FluffyPlace::Todo) => todo!(),
            (Contract::Const, FluffyPlace::Const) => todo!(),
            (Contract::Const, FluffyPlace::StackPure { location }) => todo!(),
            (Contract::Const, FluffyPlace::ImmutableStackOwned { location }) => todo!(),
            (Contract::Const, FluffyPlace::MutableStackOwned { location }) => todo!(),
            (Contract::Const, FluffyPlace::Transient) => todo!(),
            (Contract::Const, FluffyPlace::Ref { guard }) => todo!(),
            (Contract::Const, FluffyPlace::RefMut { guard }) => todo!(),
            (Contract::Const, FluffyPlace::Leashed) => todo!(),
            (Contract::Const, FluffyPlace::Todo) => todo!(),
            (Contract::Leash, FluffyPlace::Const) => todo!(),
            (Contract::Leash, FluffyPlace::StackPure { location }) => todo!(),
            (Contract::Leash, FluffyPlace::ImmutableStackOwned { location }) => todo!(),
            (Contract::Leash, FluffyPlace::MutableStackOwned { location }) => todo!(),
            (Contract::Leash, FluffyPlace::Transient) => todo!(),
            (Contract::Leash, FluffyPlace::Ref { guard }) => todo!(),
            (Contract::Leash, FluffyPlace::RefMut { guard }) => todo!(),
            (Contract::Leash, FluffyPlace::Leashed) => todo!(),
            (Contract::Leash, FluffyPlace::Todo) => todo!(),
            (Contract::At, FluffyPlace::Const) => todo!(),
            (Contract::At, FluffyPlace::StackPure { location }) => todo!(),
            (Contract::At, FluffyPlace::ImmutableStackOwned { location }) => todo!(),
            (Contract::At, FluffyPlace::MutableStackOwned { location }) => todo!(),
            (Contract::At, FluffyPlace::Transient) => todo!(),
            (Contract::At, FluffyPlace::Ref { guard }) => todo!(),
            (Contract::At, FluffyPlace::RefMut { guard }) => todo!(),
            (Contract::At, FluffyPlace::Leashed) => todo!(),
            (Contract::At, FluffyPlace::Todo) => todo!(),
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
