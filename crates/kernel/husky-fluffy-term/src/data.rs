mod ethereal;
mod hollow;
mod solid;

use husky_declarative_term::RuneIndex;
use husky_ethereal_signature::helpers::trai_for_ty::is_ty_term_always_copyable;

pub(crate) use self::ethereal::*;
pub(crate) use self::hollow::*;
pub(crate) use self::solid::*;

use crate::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
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
        parameter_rune: Option<FluffyTermRune>,
        parameter_ty: FluffyTerm,
        return_ty: FluffyTerm,
        ty_ethereal_term: Option<EtherealTermCurry>,
    },
    Hole(HoleKind, Hole),
    Category(TermCategory),
    Ritchie {
        ritchie_kind: RitchieKind,
        parameter_contracted_tys: &'a [FluffyRitchieParameter],
        return_ty: FluffyTerm,
    },
    Symbol {
        term: EtherealTermSymbol,
        ty: FluffyTerm,
    },
    Rune {
        ty: FluffyTerm,
        idx: RuneIndex,
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
            FluffyTermData::Literal(lit) => format!("{}", lit.display(db)),
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
                    write!(s, "{}", ty_path.ident(db).data(db)).unwrap();
                    for ty_argument in ty_arguments.iter() {
                        write!(s, " {}", ty_argument.show(db, terms)).unwrap();
                    }
                    s
                }
            },
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => {
                if let Some(parameter_rune) = parameter_rune {
                    format!(
                        "<{}: {}> -> {}",
                        parameter_rune.show(db, terms),
                        parameter_ty.show(db, terms),
                        return_ty.show(db, terms)
                    )
                } else {
                    format!(
                        "{} -> {}",
                        parameter_ty.show(db, terms),
                        return_ty.show(db, terms)
                    )
                }
            }
            FluffyTermData::Hole(hole_kind, _) => match hole_kind {
                HoleKind::UnspecifiedIntegerType => "_i".to_string(),
                HoleKind::UnspecifiedFloatType => "_f".to_string(),
                HoleKind::ImplicitType => "_t".to_string(),
                HoleKind::Any => "_a".to_string(),
            },
            FluffyTermData::Category(_) => "Type".to_string(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => match ritchie_kind {
                RitchieKind::Type(ritchi_ty_kind) => match ritchi_ty_kind {
                    RitchieTypeKind::Fn => {
                        // for param in parameter_contracted_tys.iter() {
                        //     match param {
                        //         FluffyRitchieParameter::Regular(param) => {
                        //             p!(param.ty().show(db, terms))
                        //         }
                        //         FluffyRitchieParameter::Variadic(_) => (),
                        //         FluffyRitchieParameter::Keyed(_) => todo!(),
                        //     }
                        // }
                        format!("fn(...) -> {}", return_ty.show(db, terms))
                    }
                    RitchieTypeKind::Gn => {
                        format!("fn(...) -> {}", return_ty.show(db, terms))
                    }
                },
                RitchieKind::Trait(_) => todo!(),
            },
            FluffyTermData::Symbol { term, ty } => format!("symbol({})", ty.show(db, terms)),
            FluffyTermData::Rune { ty, idx } => format!("rune({idx}, {})", ty.show(db, terms)),
            FluffyTermData::TypeVariant { path } => format!("{:?}", path.debug(db)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db]
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
        parameter_rune: Option<FluffyTermRune>,
        parameter_ty: FluffyTerm,
        return_ty: FluffyTerm,
        ty_ethereal_term: Option<EtherealTermCurry>,
    },
    Hole(HoleKind, Hole),
    Category(TermCategory),
    Ritchie {
        ritchie_kind: RitchieKind,
        parameter_contracted_tys: &'a [FluffyRitchieParameter],
        return_ty: FluffyTerm,
    },
    Symbol {
        symbol: EtherealTermSymbol,
    },
    Rune {
        rune: EtherealTermRune,
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
            FluffyTermBase::Ethereal(term) => ethereal_term_fluffy_base_ty_data(db, term),
            FluffyTermBase::Solid(term) => term.data_inner(terms.solid_terms()).into(),
            FluffyTermBase::Hollow(term) => term.fluffy_base_ty_data(db, terms),
            FluffyTermBase::Place => todo!(),
        }
    }

    /// `None` means the notion is not applicable,
    /// because the term is either a non type or a conceptual type
    #[deprecated(note = "ad hoc implementation")]
    pub fn is_always_copyable(
        self,
        db: &::salsa::Db,
        terms: &FluffyTerms,
    ) -> FluffyTermResult<Option<bool>> {
        match self.base_ty_data_inner(db, terms) {
            FluffyBaseTypeData::TypeOntology {
                ty_path,
                refined_ty_path,
                ty_arguments,
                ty_ethereal_term,
            } => match ty_ethereal_term {
                Some(ty_ethereal_term) => {
                    is_ty_term_always_copyable(ty_ethereal_term, db).map_err(Into::into)
                }
                None => {
                    // p!(self.show(db, terms));
                    // todo!()
                    // ad hoc, something is wrong here
                    Ok(Some(false))
                }
            },
            FluffyBaseTypeData::Curry {
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => Ok(None),
            FluffyBaseTypeData::Hole(hole_kind, _) => match hole_kind {
                HoleKind::UnspecifiedIntegerType | HoleKind::UnspecifiedFloatType => Ok(Some(true)),
                HoleKind::ImplicitType => todo!(),
                HoleKind::Any => todo!(),
            },
            FluffyBaseTypeData::Category(_) => Ok(None),
            FluffyBaseTypeData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => match ritchie_kind {
                RitchieKind::Type(ritchie_ty_kind) => match ritchie_ty_kind {
                    RitchieTypeKind::Fn => Ok(Some(true)),
                    RitchieTypeKind::Gn => Ok(Some(true)),
                },
                RitchieKind::Trait(_) => todo!(),
            },
            FluffyBaseTypeData::Symbol { symbol: term } => Ok(Some(false)),
            FluffyBaseTypeData::Rune { rune } => todo!(), // ad hoc
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
