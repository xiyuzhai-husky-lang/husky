mod ethereal;
mod hollow;
mod solid;

pub(crate) use self::ethereal::*;
pub(crate) use self::hollow::*;
pub(crate) use self::solid::*;

use crate::*;
use husky_dec_term::term::RuneIndex;
use husky_eth_signature::helpers::trai_for_ty::is_ty_term_always_copyable;
use husky_eth_term::term::{curry::EthCurry, rune::EthRune, symbol::EthSymbol};
use husky_term_prelude::literal::Literal;
use husky_vfs::Toolchain;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum FlyTermData<'a> {
    Literal(Literal),
    TypeOntology {
        ty_path: TypePath,
        refined_ty_path: Either<PreludeTypePath, CustomTypePath>,
        ty_arguments: &'a [FlyTerm],
        ty_ethereal_term: Option<EthTerm>,
    },
    Curry {
        toolchain: Toolchain,
        curry_kind: CurryKind,
        variance: Variance,
        parameter_rune: Option<RuneFlyTerm>,
        parameter_ty: FlyTerm,
        return_ty: FlyTerm,
        ty_ethereal_term: Option<EthCurry>,
    },
    Hole(HoleKind, Hole),
    Category(Category),
    Ritchie {
        ritchie_kind: RitchieKind,
        parameter_contracted_tys: &'a [FlyRitchieParameter],
        return_ty: FlyTerm,
    },
    Symbol {
        term: EthSymbol,
        ty: FlyTerm,
    },
    Rune {
        ty: FlyTerm,
        index: RuneIndex,
    },
    TypeVariant {
        path: TypeVariantPath,
    },
}

impl<'a> FlyTermData<'a> {
    // todo: change to using show_aux
    #[inline(never)]
    pub fn show(&self, db: &::salsa::Db, terms: &FlyTerms) -> String {
        use salsa::DisplayWithDb;
        match self {
            FlyTermData::Literal(lit) => format!("{}", lit.display(db)),
            FlyTermData::TypeOntology {
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
            FlyTermData::Curry {
                toolchain,
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
            FlyTermData::Hole(hole_kind, _) => match hole_kind {
                HoleKind::UnspecifiedIntegerType => "_i".to_string(),
                HoleKind::UnspecifiedFloatType => "_f".to_string(),
                HoleKind::ImplicitType => "_t".to_string(),
                HoleKind::Any => "_a".to_string(),
            },
            FlyTermData::Category(_) => "Type".to_string(),
            FlyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => match ritchie_kind {
                RitchieKind::Type(ritchi_ty_kind) => match ritchi_ty_kind {
                    TypeRitchieKind::Fn => {
                        // for param in parameter_contracted_tys.iter() {
                        //     match param {
                        //         FlyRitchieParameter::Regular(param) => {
                        //             p!(param.ty().show(db, terms))
                        //         }
                        //         FlyRitchieParameter::Variadic(_) => (),
                        //         FlyRitchieParameter::Keyed(_) => todo!(),
                        //     }
                        // }
                        format!("fn(...) -> {}", return_ty.show(db, terms))
                    }
                    TypeRitchieKind::Gn => {
                        format!("fn(...) -> {}", return_ty.show(db, terms))
                    }
                },
                RitchieKind::Trait(_) => todo!(),
            },
            FlyTermData::Symbol { term, ty } => format!("symbol({})", ty.show(db, terms)),
            FlyTermData::Rune { ty, index: idx } => {
                format!("rune({idx}, {})", ty.show(db, terms))
            }
            FlyTermData::TypeVariant { path } => format!("{:?}", path.debug(db)),
        }
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum FlyBaseTypeData<'a> {
    TypeOntology {
        ty_path: TypePath,
        refined_ty_path: Either<PreludeTypePath, CustomTypePath>,
        ty_arguments: &'a [FlyTerm],
        ty_ethereal_term: Option<EthTerm>,
    },
    Curry {
        curry_kind: CurryKind,
        variance: Variance,
        parameter_rune: Option<RuneFlyTerm>,
        parameter_ty: FlyTerm,
        return_ty: FlyTerm,
        ty_ethereal_term: Option<EthCurry>,
    },
    Hole(HoleKind, Hole),
    Category(Category),
    Ritchie {
        ritchie_kind: RitchieKind,
        parameter_contracted_tys: &'a [FlyRitchieParameter],
        return_ty: FlyTerm,
    },
    Symbol {
        symbol: EthSymbol,
    },
    Rune {
        rune: EthRune,
    },
}

impl FlyTerm {
    pub fn data<'a, 'b>(self, engine: &'a impl FlyTermEngine<'b>) -> FlyTermData<'a>
    where
        'b: 'a,
    {
        self.data_inner(engine.db(), engine.fluffy_terms())
    }

    pub fn data_inner<'a>(self, db: &'a ::salsa::Db, terms: &'a FlyTerms) -> FlyTermData<'a> {
        match self.base_resolved_inner(terms) {
            FlyTermBase::Eth(term) => ethereal_term_data(db, term),
            FlyTermBase::Sol(term) => term.data_inner(terms.solid_terms()).into(),
            FlyTermBase::Hol(term) => term.fluffy_data(db, terms),
            FlyTermBase::Place => todo!(),
        }
    }

    pub fn base_ty_data<'a, 'b>(self, engine: &'a impl FlyTermEngine<'b>) -> FlyBaseTypeData<'a>
    where
        'b: 'a,
    {
        self.base_ty_data_inner(engine.db(), engine.fluffy_terms())
    }

    pub fn base_ty_data_inner<'a>(
        self,
        db: &'a ::salsa::Db,
        terms: &'a FlyTerms,
    ) -> FlyBaseTypeData<'a> {
        match self.base_resolved_inner(terms) {
            FlyTermBase::Eth(term) => ethereal_term_fluffy_base_ty_data(db, term),
            FlyTermBase::Sol(term) => term.data_inner(terms.solid_terms()).into(),
            FlyTermBase::Hol(term) => term.fluffy_base_ty_data(db, terms),
            FlyTermBase::Place => todo!(),
        }
    }

    /// `None` means the notion is not applicable,
    /// because the term is either a non type or a conceptual type
    #[deprecated(note = "ad hoc implementation")]
    pub fn is_always_copyable(
        self,
        db: &::salsa::Db,
        terms: &FlyTerms,
    ) -> FlyTermResult<Option<bool>> {
        match self.base_ty_data_inner(db, terms) {
            FlyBaseTypeData::TypeOntology {
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
            FlyBaseTypeData::Curry {
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => Ok(None),
            FlyBaseTypeData::Hole(hole_kind, _) => match hole_kind {
                HoleKind::UnspecifiedIntegerType | HoleKind::UnspecifiedFloatType => Ok(Some(true)),
                HoleKind::ImplicitType => todo!(),
                HoleKind::Any => todo!(),
            },
            FlyBaseTypeData::Category(_) => Ok(None),
            FlyBaseTypeData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => match ritchie_kind {
                RitchieKind::Type(ritchie_ty_kind) => match ritchie_ty_kind {
                    TypeRitchieKind::Fn => Ok(Some(true)),
                    TypeRitchieKind::Gn => Ok(Some(true)),
                },
                RitchieKind::Trait(_) => todo!(),
            },
            FlyBaseTypeData::Symbol { symbol: term } => Ok(Some(false)),
            FlyBaseTypeData::Rune { rune } => todo!(), // ad hoc
        }
    }
}

pub(crate) struct FlyTermDataKindMerger<'a> {
    has_err: bool,
    has_hol: bool,
    has_sol: bool,
    hol_terms: &'a HolTerms,
}

impl<'a> FlyTermDataKindMerger<'a> {
    pub(crate) fn new(hol_terms: &'a impl std::borrow::Borrow<HolTerms>) -> Self {
        Self {
            has_err: false,
            has_sol: false,
            has_hol: false,
            hol_terms: hol_terms.borrow(),
        }
    }

    pub(crate) fn accept_one(&mut self, term: FlyTerm) {
        if term.place().is_some() {
            self.has_sol = true
        }
        match term.resolve_progress(self.hol_terms) {
            TermResolveProgress::UnresolvedHol => self.has_hol = true,
            TermResolveProgress::ResolvedEth(_) => (),
            TermResolveProgress::ResolvedSol(_) => self.has_sol = true,
            TermResolveProgress::Err => self.has_err = true,
        }
    }

    pub(crate) fn accept(&mut self, terms: impl IntoIterator<Item = FlyTerm>) {
        for term in terms {
            self.accept_one(term)
        }
    }

    pub(crate) fn data_kind(self) -> FlyTermDataKind {
        if self.has_err {
            FlyTermDataKind::Err
        } else if self.has_hol {
            FlyTermDataKind::Hollow
        } else if self.has_sol {
            FlyTermDataKind::Solid
        } else {
            FlyTermDataKind::Ethereal
        }
    }
}

pub(crate) enum FlyTermDataKind {
    Err,
    Ethereal,
    Solid,
    Hollow,
}
