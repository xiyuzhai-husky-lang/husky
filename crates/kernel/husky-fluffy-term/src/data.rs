mod ethereal;
mod hollow;
mod solid;

pub(crate) use self::hollow::*;
pub(crate) use self::solid::*;

use self::ethereal::*;
use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub enum FluffyTermData<'a> {
    Literal(TermLiteral),
    TypeOntology {
        ty_path: TypePath,
        refined_ty_path: Either<PreludeTypePath, CustomTypePath>,
        arguments: &'a [FluffyTerm],
        ty_ethereal_term: Option<EtherealTerm>,
    },
    TypeOntologyAtPlace {
        ty_path: TypePath,
        refined_ty_path: Either<PreludeTypePath, CustomTypePath>,
        arguments: &'a [FluffyTerm],
        base_ty_ethereal_term: Option<EtherealTerm>,
        place: Place,
    },
    Curry {
        curry_kind: CurryKind,
        variance: Variance,
        parameter_variable: Option<FluffyTerm>,
        parameter_ty: FluffyTerm,
        return_ty: FluffyTerm,
        ty_ethereal_term: Option<EtherealTermCurry>,
    },
    Hole(HoleKind, Hole),
    HoleAtPlace {
        hole_kind: HoleKind,
        hole: Hole,
        place: Place,
    },
    Category(TermCategory),
    Ritchie {
        ritchie_kind: RitchieKind,
        parameter_contracted_tys: &'a [FluffyTermRitchieParameter],
        return_ty: FluffyTerm,
    },
    Symbol {
        term: EtherealTermSymbol,
        ty: FluffyTerm,
    },
    SymbolAtPlace {
        term: EtherealTermSymbol,
        place: Place,
    },
    Variable {
        ty: FluffyTerm,
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

#[derive(Debug, PartialEq, Eq)]
pub struct TermRitchieFluffyData {
    ritchie_kind: RitchieKind,
    parameter_contracted_tys: SmallVec<[FluffyTermRitchieParameter; 2]>,
    variadics: (),
    keyed_parameter_contracted_tys: (),
    return_ty: EtherealTerm,
}

impl TermRitchieFluffyData {
    fn as_ref<'a>(&'a self) -> FluffyTermData<'a> {
        FluffyTermData::Ritchie {
            ritchie_kind: self.ritchie_kind,
            parameter_contracted_tys: &self.parameter_contracted_tys,
            return_ty: self.return_ty.into(),
        }
    }
}

#[salsa::tracked(jar = FluffyTermJar, return_ref)]
pub(crate) fn term_ritchie_fluffy_data(
    db: &dyn FluffyTermDb,
    term: EtherealTermRitchie,
) -> TermRitchieFluffyData {
    TermRitchieFluffyData {
        ritchie_kind: term.ritchie_kind(db),
        parameter_contracted_tys: term
            .parameter_contracted_tys(db)
            .iter()
            .copied()
            .map(Into::into)
            .collect(),
        variadics: (),
        keyed_parameter_contracted_tys: (),
        return_ty: term.return_ty(db),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum TermApplicationFluffyData {
    TypeOntology {
        path: TypePath,
        refined_path: Either<PreludeTypePath, CustomTypePath>,
        arguments: SmallVec<[FluffyTerm; 2]>,
        ty_ethereal_term: EtherealTerm,
    },
}

/// can't directly return FluffyTermData<'_> because of lifetime
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
            ty_ethereal_term: term.into(),
        },
        TermFunctionReduced::Trait(_) => todo!(),
        TermFunctionReduced::Other(_) => todo!(),
    }
}

impl TermApplicationFluffyData {
    fn as_ref<'a>(&'a self) -> FluffyTermData<'a> {
        match self {
            TermApplicationFluffyData::TypeOntology {
                path,
                refined_path,
                arguments,
                ty_ethereal_term,
            } => FluffyTermData::TypeOntology {
                ty_path: *path,
                refined_ty_path: *refined_path,
                arguments,
                ty_ethereal_term: Some(*ty_ethereal_term),
            },
        }
    }
}

pub(crate) struct FluffyTermDataKindMerger<'a> {
    has_err: bool,
    has_hollow: bool,
    has_solid: bool,
    hollow_terms: &'a HollowTerms,
}

impl<'a> FluffyTermDataKindMerger<'a> {
    pub(crate) fn new(hollow_terms: &'a impl std::borrow::Borrow<HollowTerms>) -> Self {
        Self {
            has_err: false,
            has_solid: false,
            has_hollow: false,
            hollow_terms: hollow_terms.borrow(),
        }
    }

    pub(crate) fn accept_one(&mut self, term: FluffyTerm) {
        match term.resolve_progress(self.hollow_terms) {
            TermResolveProgress::UnresolvedHollow => self.has_hollow = true,
            TermResolveProgress::ResolvedEthereal(_) => (),
            TermResolveProgress::ResolvedSolid(_) => self.has_solid = true,
            TermResolveProgress::Err => self.has_err = true,
        }
    }

    pub(crate) fn accept(&mut self, terms: impl IntoIterator<Item = FluffyTerm>) {
        for term in terms {
            self.accept_one(term)
        }
    }

    pub(crate) fn data_kind(self) -> FluffyTermDataKind {
        if self.has_err {
            todo!()
        } else if self.has_hollow {
            FluffyTermDataKind::Hollow
        } else if self.has_solid {
            FluffyTermDataKind::Solid
        } else {
            FluffyTermDataKind::Ethereal
        }
    }
}

pub(crate) enum FluffyTermDataKind {
    Err,
    Ethereal,
    Solid,
    Hollow,
}
