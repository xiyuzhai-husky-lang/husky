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
    pub fn new_from_signature(
        engine: &mut impl FluffyTermEngine,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
        signature: SymbolSignature,
    ) -> EtherealTermResult<Self> {
        let ty = EtherealTerm::ty_from_declarative(engine.db(), signature.ty()?)?;
        Ok(Self::new(engine, current_syn_symbol_idx, ty.into()))
    }

    pub fn new(
        engine: &mut impl FluffyTermEngine,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
        ty: FluffyTerm,
    ) -> Self {
        // ad hoc
        Self(ty)
        // let expr_region_data = engine.expr_region_data();
        // let local_symbol_idx = current_syn_symbol_idx.into_local_symbol_idx(expr_region_data);
        // let place = match expr_region_data[current_syn_symbol_idx].modifier() {
        //     EphemSymbolModifier::None => Place::StackPure {
        //         location: local_symbol_idx.into(),
        //     },
        //     EphemSymbolModifier::Mut => Place::MutableStackOwned {
        //         location: local_symbol_idx.into(),
        //     },
        //     EphemSymbolModifier::RefMut => Place::RefMut {
        //         guard: Left(local_symbol_idx.into()),
        //     },
        //     EphemSymbolModifier::Const => Place::Const,
        //     EphemSymbolModifier::Ambersand(_) => todo!(),
        //     EphemSymbolModifier::AmbersandMut(_) => todo!(),
        //     EphemSymbolModifier::Le => todo!(),
        //     EphemSymbolModifier::Tilde => todo!(), // todo: handle variance
        // };
        // Self(match ty {
        //     FluffyTerm::Literal(_) => todo!(),
        //     FluffyTerm::Symbol(term) => SolidTerm::new(
        //         engine.fluffy_term_region_mut().solid_terms_mut(),
        //         SolidTermData::SymbolAtPlace { term, place },
        //     )
        //     .into(),
        //     FluffyTerm::Variable(_) => todo!(),
        //     FluffyTerm::EntityPath(path) => match path {
        //         TermEntityPath::Fugitive(_) => todo!(),
        //         TermEntityPath::Trait(_) => todo!(),
        //         TermEntityPath::TypeOntology(path) => {
        //             let data = SolidTermData::TypeOntologyAtPlace {
        //                 place,
        //                 ty_path: path,
        //                 refined_ty_path: path.refine(engine.db()),
        //                 arguments: smallvec![],
        //                 base_ty_term: Some(TermEntityPath::TypeOntology(path).into()),
        //             };
        //             SolidTerm::new(engine.fluffy_term_region_mut().solid_terms_mut(), data).into()
        //         }
        //         TermEntityPath::TypeInstance(_) => todo!(),
        //         TermEntityPath::TypeVariant(_) => todo!(),
        //     },
        //     FluffyTerm::Category(term) => match place {
        //         Place::Const => term.into(),
        //         Place::StackPure { location } => todo!(),
        //         Place::ImmutableStackOwned { location } => todo!(),
        //         Place::MutableStackOwned { location } => todo!(),
        //         Place::Transient => todo!(),
        //         Place::Ref { guard } => todo!(),
        //         Place::RefMut { guard } => todo!(),
        //         Place::Leashed => todo!(),
        //         Place::Todo => todo!(),
        //     },
        //     FluffyTerm::Universe(_) => todo!(),
        //     FluffyTerm::Curry(_) => todo!(),
        //     FluffyTerm::Ritchie(_) => todo!(),
        //     FluffyTerm::Abstraction(_) => todo!(),
        //     FluffyTerm::Application(term) => {
        //         let expansion = term.application_expansion(engine.db());
        //         match expansion.function() {
        //             TermFunctionReduced::TypeOntology(path) => {
        //                 let data = SolidTermData::TypeOntologyAtPlace {
        //                     place,
        //                     ty_path: path,
        //                     refined_ty_path: path.refine(engine.db()),
        //                     arguments: expansion
        //                         .arguments(engine.db())
        //                         .iter()
        //                         .map(|t| (*t).into())
        //                         .collect(),
        //                     base_ty_term: Some(term.into()),
        //                 };
        //                 SolidTerm::new(engine.fluffy_term_region_mut().solid_terms_mut(), data)
        //                     .into()
        //             }
        //             TermFunctionReduced::Trait(_) => todo!(),
        //             TermFunctionReduced::Other(_) => todo!(),
        //         }
        //     }
        //     FluffyTerm::Subitem(_) => todo!(),
        //     FluffyTerm::AsTraitSubitem(_) => todo!(),
        //     FluffyTerm::TraitConstraint(_) => todo!(),
        //     FluffyTerm::Solid(_) => {
        //         let (inner_place, base_ty) = ty.ty_data(engine);
        //         let place = match inner_place {
        //             // ad hoc
        //             Some(inner_place) => place,
        //             None => place,
        //         };
        //         let data = match base_ty {
        //             FluffyBaseTypeData::TypeOntology {
        //                 ty_path,
        //                 refined_ty_path,
        //                 ty_arguments,
        //                 ty_ethereal_term,
        //             } => SolidTermData::TypeOntologyAtPlace {
        //                 ty_path,
        //                 refined_ty_path,
        //                 arguments: ty_arguments.to_smallvec(),
        //                 base_ty_term: ty_ethereal_term,
        //                 place,
        //             },
        //             FluffyBaseTypeData::Curry {
        //                 curry_kind,
        //                 variance,
        //                 parameter_variable,
        //                 parameter_ty,
        //                 return_ty,
        //                 ty_ethereal_term,
        //             } => todo!(),
        //             FluffyBaseTypeData::Hole(_, _) => todo!(),
        //             FluffyBaseTypeData::Category(_) => todo!(),
        //             FluffyBaseTypeData::Ritchie {
        //                 ritchie_kind,
        //                 parameter_contracted_tys,
        //                 return_ty,
        //             } => todo!(),
        //             FluffyBaseTypeData::Symbol { term } => todo!(),
        //         };
        //         SolidTerm::new(engine.fluffy_term_region_mut().solid_terms_mut(), data).into()
        //     }
        //     FluffyTerm::Hollow(_) => {
        //         let data = match ty.data(engine) {
        //             FluffyTermData::Literal(_) => todo!(),
        //             FluffyTermData::TypeOntology {
        //                 ty_path: path,
        //                 refined_ty_path: refined_path,
        //                 ty_arguments: arguments,
        //                 ..
        //             } => HollowTermData::TypeOntologyAtPlace {
        //                 place,
        //                 ty_path: path,
        //                 refined_ty_path: refined_path,
        //                 ty_arguments: arguments.to_smallvec(),
        //             },
        //             FluffyTermData::TypeOntologyAtPlace { .. } => todo!(),
        //             FluffyTermData::Curry {
        //                 curry_kind,
        //                 variance,
        //                 parameter_variable,
        //                 parameter_ty,
        //                 return_ty,
        //                 ty_ethereal_term,
        //             } => todo!(),
        //             FluffyTermData::Hole(hole_kind, hole) => HollowTermData::PlaceHole {
        //                 place,
        //                 hole_kind,
        //                 hole,
        //             },
        //             FluffyTermData::Category(_) => todo!(),
        //             FluffyTermData::Ritchie {
        //                 ritchie_kind,
        //                 parameter_contracted_tys,
        //                 return_ty,
        //                 ..
        //             } => todo!(),
        //             FluffyTermData::HoleAtPlace {
        //                 place,
        //                 hole_kind,
        //                 hole,
        //             } => todo!(),
        //             FluffyTermData::Symbol { .. } => todo!(),
        //             FluffyTermData::SymbolAtPlace { .. } => todo!(),
        //             FluffyTermData::Variable { ty } => todo!(),
        //             FluffyTermData::TypeVariant { path } => todo!(),
        //         };
        //         HollowTerm::new(engine, data).into()
        //     }
        // })
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
        location: FluffyStackLocationIdx,
    },
    /// lvalue nonreference
    ImmutableStackOwned {
        location: FluffyStackLocationIdx,
    },
    /// lvalue nonreference
    MutableStackOwned {
        location: FluffyStackLocationIdx,
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
        guard: Either<FluffyStackLocationIdx, FluffyLifetime>,
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
        guard: Either<FluffyStackLocationIdx, FluffyLifetime>,
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
            (TermContract::Move, FluffyPlace::StackPure { location }) => todo!(),
            (TermContract::Move, FluffyPlace::ImmutableStackOwned { location }) => todo!(),
            (TermContract::Move, FluffyPlace::MutableStackOwned { location }) => todo!(),
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
            (TermContract::BorrowMut, FluffyPlace::RefMut { guard }) => todo!(),
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
pub struct FluffyStackLocationIdx(LocalSymbolIdx);

impl From<LocalSymbolIdx> for FluffyStackLocationIdx {
    fn from(idx: LocalSymbolIdx) -> Self {
        Self(idx)
    }
}

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
