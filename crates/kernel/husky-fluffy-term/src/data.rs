mod ethereal;
mod hollow;
mod solid;

pub(crate) use self::ethereal::*;
pub(crate) use self::hollow::*;
pub(crate) use self::solid::*;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub enum FluffyTermData<'a> {
    Literal(TermLiteral),
    TypeOntology {
        ty_path: TypePath,
        refined_ty_path: Either<PreludeTypePath, CustomTypePath>,
        ty_arguments: &'a [FluffyTerm],
        ty_ethereal_term: Option<EtherealTerm>,
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
    Variable {
        ty: FluffyTerm,
    },
    TypeVariant {
        path: TypeVariantPath,
    },
}

impl<'a> FluffyTermData<'a> {
    // todo: change to using show_aux
    #[inline(never)]
    pub fn show(&self, db: &::salsa::Db, terms: &FluffyTerms) -> String {
        use salsa::DisplayWithDb;
        match self {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path,
                refined_ty_path,
                ty_arguments,
                ty_ethereal_term,
            } => match ty_ethereal_term {
                Some(base_ty_term) => format!("{}", base_ty_term.display(db)),
                None => {
                    use std::fmt::Write;
                    let mut s = String::default();
                    write!(s, "{}", ty_path.ident(db).data(db));
                    for ty_argument in ty_arguments.iter() {
                        write!(s, " {}", ty_argument.show(db, terms));
                    }
                    s
                }
            },
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => {
                if parameter_variable.is_some() {
                    todo!()
                }
                format!(
                    "{} -> {}",
                    parameter_ty.show(db, terms),
                    return_ty.show(db, terms)
                )
            }
            FluffyTermData::Hole(hole_kind, _) => match hole_kind {
                HoleKind::UnspecifiedIntegerType => "_i".to_string(),
                HoleKind::UnspecifiedFloatType => "_f".to_string(),
                HoleKind::ImplicitType => "_t".to_string(),
                HoleKind::Any => "_a".to_string(),
            },
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => match ritchie_kind {
                RitchieKind::Type(ritchi_ty_kind) => match ritchi_ty_kind {
                    RitchieTypeKind::Fn => {
                        for param in parameter_contracted_tys.iter() {
                            match param {
                                FluffyTermRitchieParameter::Regular(param) => {
                                    p!(param.ty().show(db, terms))
                                }
                                FluffyTermRitchieParameter::Variadic(_) => todo!(),
                                FluffyTermRitchieParameter::Keyed(_) => todo!(),
                            }
                        }
                        format!("fn(...) -> {}", return_ty.show(db, terms))
                    }
                    RitchieTypeKind::Gn => todo!(),
                },
                RitchieKind::Trait(_) => todo!(),
            },
            FluffyTermData::Symbol { term, ty } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub enum FluffyBaseTypeData<'a> {
    TypeOntology {
        ty_path: TypePath,
        refined_ty_path: Either<PreludeTypePath, CustomTypePath>,
        ty_arguments: &'a [FluffyTerm],
        ty_ethereal_term: Option<EtherealTerm>,
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
    Category(TermCategory),
    Ritchie {
        ritchie_kind: RitchieKind,
        parameter_contracted_tys: &'a [FluffyTermRitchieParameter],
        return_ty: FluffyTerm,
    },
    Symbol {
        term: EtherealTermSymbol,
    },
}

impl FluffyTerm {
    pub fn data<'a, 'b>(self, engine: &'a impl FluffyTermEngine<'b>) -> FluffyTermData<'a>
    where
        'b: 'a,
    {
        self.data_inner(engine.db(), engine.fluffy_terms())
    }

    pub fn data_inner<'a>(self, db: &'a ::salsa::Db, terms: &'a FluffyTerms) -> FluffyTermData<'a> {
        match self.base_resolved_inner(terms) {
            FluffyTermBase::Ethereal(term) => ethereal_term_data(db, term),
            FluffyTermBase::Solid(term) => term.data_inner(terms.solid_terms()).into(),
            FluffyTermBase::Hollow(term) => term.fluffy_data(db, terms),
            FluffyTermBase::Place => todo!(),
        }
    }

    pub fn base_ty_data<'a, 'b>(
        self,
        engine: &'a impl FluffyTermEngine<'b>,
    ) -> FluffyBaseTypeData<'a>
    where
        'b: 'a,
    {
        self.base_ty_data_inner(engine.db(), engine.fluffy_terms())
    }

    pub fn base_ty_data_inner<'a>(
        self,
        db: &'a ::salsa::Db,
        terms: &'a FluffyTerms,
    ) -> FluffyBaseTypeData<'a> {
        match self.base_resolved_inner(terms) {
            FluffyTermBase::Ethereal(term) => ethereal_term_data2(db, term),
            FluffyTermBase::Solid(term) => term.data_inner(terms.solid_terms()).into(),
            FluffyTermBase::Hollow(term) => term.fluffy_base_ty_data(db, terms),
            FluffyTermBase::Place => todo!(),
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
        if term.place().is_some() {
            self.has_solid = true
        }
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
