mod ethereal;
mod hollow;
mod solid;

pub(crate) use self::hollow::*;
pub(crate) use self::solid::*;

use self::ethereal::*;
use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub enum FluffyTermData<'a> {
    Literal(TermLiteral),
    TypeOntology {
        path: TypePath,
        refined_path: Either<PreludeTypePath, CustomTypePath>,
        arguments: &'a [FluffyTerm],
        ty_ethereal_term: Option<EtherealTerm>,
    },
    PlaceTypeOntology {
        place: Place,
        ty_path: TypePath,
        refined_ty_path: Either<PreludeTypePath, CustomTypePath>,
        arguments: &'a [FluffyTerm],
        base_ty_ethereal_term: Option<EtherealTerm>,
    },
    Curry {
        curry_kind: CurryKind,
        variance: Variance,
        parameter_variable: Option<FluffyTerm>,
        parameter_ty: FluffyTerm,
        return_ty: FluffyTerm,
    },
    Hole(HoleKind, Hole),
    Category(TermCategory),
    Ritchie {
        ritchie_kind: RitchieKind,
        parameter_contracted_tys: &'a [FluffyTermRitchieParameterContractedType],
        return_ty: FluffyTerm,
    },
    PlaceHole {
        place: Place,
        hole_kind: HoleKind,
        hole: Hole,
    },
}

impl FluffyTerm {
    pub fn data<'a, 'b>(self, engine: &'a impl FluffyTermEngine<'b>) -> FluffyTermData<'a>
    where
        'b: 'a,
    {
        self.data_inner(engine.db(), engine.fluffy_terms())
    }

    pub fn data_inner<'a>(
        self,
        db: &'a dyn FluffyTermDb,
        fluffy_terms: &'a FluffyTerms,
    ) -> FluffyTermData<'a> {
        match self.nested() {
            NestedFluffyTerm::Ethereal(term) => ethereal_term_data(db, term),
            NestedFluffyTerm::Solid(term) => term.data2(fluffy_terms.solid_terms()).into(),
            NestedFluffyTerm::Hollow(term) => term.fluffy_data(db, fluffy_terms),
        }
    }
}

// todo: replace this with macro
impl<'a, _Db: EtherealTermDb + ?Sized> ::salsa::DebugWithDb<_Db> for FluffyTermData<'a> {
    fn fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
        _db: &_Db,
        _level: ::salsa::DebugFormatLevel,
    ) -> ::std::fmt::Result {
        #[allow(unused_imports)]
        use ::salsa::debug::helper::Fallback;
        match self {
            FluffyTermData::Literal(ref v0) => {
                let mut debug_tuple = &mut f.debug_tuple("FluffyTermData::Literal");
                debug_tuple = debug_tuple.field(&::salsa::debug::helper::SalsaDebug::<
                    TermLiteral,
                    _Db,
                >::salsa_debug(
                    #[allow(clippy::needless_borrow)]
                    &v0,
                    _db,
                    _level.next(),
                ));
                debug_tuple.finish()
            }
            FluffyTermData::TypeOntology {
                ref path,
                ref refined_path,
                ref arguments,
                ..
            } => {
                let mut debug_struct = &mut f.debug_struct("FluffyTermData::TypeOntology");
                debug_struct = debug_struct.field(
                    "path",
                    &::salsa::debug::helper::SalsaDebug::<TypePath, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        path,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "refined_path",
                    &::salsa::debug::helper::SalsaDebug::<
                        Either<PreludeTypePath, CustomTypePath>,
                        _Db,
                    >::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        refined_path,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "argument_tys",
                    &::salsa::debug::helper::SalsaDebug::<&'a [FluffyTerm], _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        arguments,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct.finish()
            }
            FluffyTermData::PlaceTypeOntology {
                ref place,
                ty_path: ref path,
                refined_ty_path: ref refined_path,
                ref arguments,
                ..
            } => {
                let mut debug_struct = &mut f.debug_struct("FluffyTermData::PlaceTypeOntology");
                debug_struct = debug_struct.field(
                    "place",
                    &::salsa::debug::helper::SalsaDebug::<Place, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        place,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "path",
                    &::salsa::debug::helper::SalsaDebug::<TypePath, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        path,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "refined_path",
                    &::salsa::debug::helper::SalsaDebug::<
                        Either<PreludeTypePath, CustomTypePath>,
                        _Db,
                    >::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        refined_path,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "argument_tys",
                    &::salsa::debug::helper::SalsaDebug::<&'a [FluffyTerm], _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        arguments,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct.finish()
            }
            FluffyTermData::Curry {
                ref curry_kind,
                ref variance,
                ref parameter_variable,
                ref parameter_ty,
                ref return_ty,
            } => {
                let mut debug_struct = &mut f.debug_struct("FluffyTermData::Curry");
                debug_struct = debug_struct.field(
                    "curry_kind",
                    &::salsa::debug::helper::SalsaDebug::<CurryKind, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        curry_kind,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "variance",
                    &::salsa::debug::helper::SalsaDebug::<Variance, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        variance,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "parameter_variable",
                    &::salsa::debug::helper::SalsaDebug::<Option<FluffyTerm>, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        parameter_variable,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "parameter_ty",
                    &::salsa::debug::helper::SalsaDebug::<FluffyTerm, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        parameter_ty,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "return_ty",
                    &::salsa::debug::helper::SalsaDebug::<FluffyTerm, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        return_ty,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct.finish()
            }
            FluffyTermData::Hole(ref v0, ref v1) => {
                let mut debug_tuple = &mut f.debug_tuple("FluffyTermData::Hole");
                debug_tuple = debug_tuple.field(
                    &::salsa::debug::helper::SalsaDebug::<HoleKind, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        &v0,
                        _db,
                        _level.next(),
                    ),
                );
                debug_tuple = debug_tuple.field(&::salsa::debug::helper::SalsaDebug::<
                    HollowTerm,
                    _Db,
                >::salsa_debug(
                    #[allow(clippy::needless_borrow)]
                    &v1,
                    _db,
                    _level.next(),
                ));
                debug_tuple.finish()
            }
            FluffyTermData::Category(ref v0) => {
                let mut debug_tuple = &mut f.debug_tuple("FluffyTermData::Category");
                debug_tuple = debug_tuple.field(&::salsa::debug::helper::SalsaDebug::<
                    TermCategory,
                    _Db,
                >::salsa_debug(
                    #[allow(clippy::needless_borrow)]
                    &v0,
                    _db,
                    _level.next(),
                ));
                debug_tuple.finish()
            }
            FluffyTermData::Ritchie {
                ref ritchie_kind,
                ref parameter_contracted_tys,
                ref return_ty,
            } => {
                let mut debug_struct = &mut f.debug_struct("FluffyTermData::Ritchie");
                debug_struct = debug_struct.field(
                    "ritchie_kind",
                    &::salsa::debug::helper::SalsaDebug::<RitchieKind, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        ritchie_kind,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "parameter_contracted_tys",
                    &::salsa::debug::helper::SalsaDebug::<
                        &'a [FluffyTermRitchieParameterContractedType],
                        _Db,
                    >::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        parameter_contracted_tys,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "return_ty",
                    &::salsa::debug::helper::SalsaDebug::<FluffyTerm, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        return_ty,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct.finish()
            }
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
        }
    }
}

#[salsa::tracked(jar = FluffyTermJar, return_ref)]
pub(crate) fn term_ritchie_fluffy_data(
    db: &dyn FluffyTermDb,
    term: EtherealTermRitchie,
) -> SmallVec<[FluffyTermRitchieParameterContractedType; 2]> {
    term.parameter_contracted_tys(db)
        .iter()
        .copied()
        .map(Into::into)
        .collect()
}

#[salsa::tracked(jar = FluffyTermJar, return_ref)]
pub(crate) fn term_application_fluffy_data(
    db: &dyn FluffyTermDb,
    term: EtherealTermApplication,
) -> TermApplicationFluffyData {
    let expansion = term.application_expansion(db);
    match expansion.function() {
        TermFunctionReduced::TypeOntology(path) => TermApplicationFluffyData::TypeOntology {
            path,
            refined_path: path.refine(db),
            arguments: expansion
                .arguments(db)
                .iter()
                .copied()
                .map(Into::into)
                .collect(),
        },
        TermFunctionReduced::Trait(_) => todo!(),
        TermFunctionReduced::Other(_) => todo!(),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum TermApplicationFluffyData {
    TypeOntology {
        path: TypePath,
        refined_path: Either<PreludeTypePath, CustomTypePath>,
        arguments: SmallVec<[FluffyTerm; 2]>,
    },
}

impl TermApplicationFluffyData {
    fn to_fluffy<'a>(&'a self, ty_ethereal_term: EtherealTerm) -> FluffyTermData<'a> {
        match self {
            TermApplicationFluffyData::TypeOntology {
                path,
                refined_path,
                arguments,
            } => FluffyTermData::TypeOntology {
                path: *path,
                refined_path: *refined_path,
                arguments,
                ty_ethereal_term: Some(ty_ethereal_term),
            },
        }
    }
}
