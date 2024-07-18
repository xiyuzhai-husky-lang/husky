mod ethereal;
mod hollow;
mod solid;

pub(crate) use self::ethereal::*;
pub(crate) use self::hollow::*;
pub(crate) use self::solid::*;

use crate::*;
use husky_dec_term::term::LambdaVariableIndex;
use husky_entity_path::path::{
    major_item::ty::{OtherTypePath, PreludeTypePath, TypePath},
    ty_variant::TypeVariantPath,
};
use husky_eth_signature::{
    helpers::trai_for_ty::is_ty_term_always_copyable, signature::package::PackageEthSignatureData,
};
use husky_eth_term::term::{
    curry::EthCurry, lambda_variable::EthLambdaVariable, symbolic_variable::EthSymbolicVariable,
};
use husky_term_prelude::{literal::Literal, ritchie::RitchieKind};
use husky_vfs::toolchain::Toolchain;
use path::major_item::{
    form::MajorFormPath,
    trai::{OtherTraitPath, PreludeTraitPath, TraitPath},
};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum FlyTermData<'a> {
    Literal(Literal),
    TypeOntology {
        ty_path: TypePath,
        refined_ty_path: Either<PreludeTypePath, OtherTypePath>,
        ty_arguments: &'a [FlyTerm],
        ty_ethereal_term: Option<EthTerm>,
    },
    Trait {
        trai_path: TraitPath,
        refined_trai_path: Either<PreludeTraitPath, OtherTraitPath>,
        trai_arguments: &'a [FlyTerm],
        trai_ethereal_term: Option<EthTerm>,
    },
    MajorTypeVar(MajorFormPath),
    Curry {
        toolchain: Toolchain,
        curry_kind: CurryKind,
        variance: Variance,
        parameter_hvar: Option<FlyHvar>,
        parameter_ty: FlyTerm,
        return_ty: FlyTerm,
        ty_ethereal_term: Option<EthCurry>,
    },
    Hole(HoleKind, Hole),
    Sort(Sort),
    Ritchie {
        ritchie_kind: RitchieKind,
        parameter_contracted_tys: &'a [FlyRitchieParameter],
        return_ty: FlyTerm,
    },
    SymbolicVariable {
        symbolic_variable: EthSymbolicVariable,
        ty: FlyTerm,
    },
    LambdaVariable {
        ty: FlyTerm,
        index: LambdaVariableIndex,
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
                        write!(s, " {}", ty_argument.show2(db, terms)).unwrap();
                    }
                    s
                }
            },
            FlyTermData::Trait { .. } => todo!(),
            FlyTermData::Curry {
                toolchain,
                curry_kind,
                variance,
                parameter_hvar,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => {
                if let Some(parameter_hvar) = parameter_hvar {
                    format!(
                        "<{}: {}> -> {}",
                        parameter_hvar.show2(db, terms),
                        parameter_ty.show2(db, terms),
                        return_ty.show2(db, terms)
                    )
                } else {
                    format!(
                        "{} -> {}",
                        parameter_ty.show2(db, terms),
                        return_ty.show2(db, terms)
                    )
                }
            }
            FlyTermData::Hole(hole_kind, _) => match hole_kind {
                HoleKind::UnspecifiedIntegerType => "_i".to_string(),
                HoleKind::UnspecifiedFloatType => "_f".to_string(),
                HoleKind::ImplicitType => "_t".to_string(),
                HoleKind::AnyOriginal | HoleKind::AnyDerived => "_a".to_string(),
            },
            FlyTermData::Sort(_) => "Type".to_string(),
            FlyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => match ritchie_kind {
                RitchieKind::Type(ritchi_ty_kind) => {
                    format!("{}(...) -> {}", ritchi_ty_kind, return_ty.show2(db, terms))
                }
                RitchieKind::Trait(_) => todo!(),
            },
            FlyTermData::SymbolicVariable {
                symbolic_variable: term,
                ty,
            } => format!("symbol({})", ty.show2(db, terms)),
            FlyTermData::LambdaVariable { ty, index: idx } => {
                format!("hvar({idx}, {})", ty.show2(db, terms))
            }
            FlyTermData::TypeVariant { path } => format!("{:?}", path.debug(db)),
            FlyTermData::MajorTypeVar(_) => todo!(),
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum FlyBaseTypeData<'a> {
    TypeOntology {
        ty_path: TypePath,
        refined_ty_path: Either<PreludeTypePath, OtherTypePath>,
        ty_arguments: &'a [FlyTerm],
        ty_ethereal_term: Option<EthTerm>,
    },
    Curry {
        curry_kind: CurryKind,
        variance: Variance,
        parameter_hvar: Option<FlyHvar>,
        parameter_ty: FlyTerm,
        return_ty: FlyTerm,
        ty_ethereal_term: Option<EthCurry>,
    },
    Hole(HoleKind, Hole),
    Sort(Sort),
    Ritchie {
        ritchie_kind: RitchieKind,
        parameter_contracted_tys: &'a [FlyRitchieParameter],
        return_ty: FlyTerm,
    },
    SymbolicVariable {
        symbolic_variable: EthSymbolicVariable,
    },
    LambdaVariable {
        lambda_variable: EthLambdaVariable,
    },
}

impl FlyTerm {
    pub fn data<'a, 'b>(self, engine: &'a impl FlyTermEngine<'b>) -> FlyTermData<'a>
    where
        'b: 'a,
    {
        self.data2(engine.db(), engine.fly_terms())
    }

    pub fn data2<'a>(self, db: &'a ::salsa::Db, terms: &'a FlyTerms) -> FlyTermData<'a> {
        match self.base_resolved_inner(terms) {
            FlyTermBase::Eth(term) => ethereal_term_data(db, term),
            FlyTermBase::Sol(term) => term.data_inner(terms.sol_terms()).into(),
            FlyTermBase::Hol(term) => term.fly_data(db, terms),
            FlyTermBase::Place => todo!(),
        }
    }

    pub fn base_ty_data<'a, 'b>(self, engine: &'a impl FlyTermEngine<'b>) -> FlyBaseTypeData<'a>
    where
        'b: 'a,
    {
        self.base_ty_data_inner(engine.db(), engine.fly_terms())
    }

    pub fn base_ty_data_inner<'a>(
        self,
        db: &'a ::salsa::Db,
        terms: &'a FlyTerms,
    ) -> FlyBaseTypeData<'a> {
        match self.base_resolved_inner(terms) {
            FlyTermBase::Eth(term) => ethereal_term_fly_base_ty_data(db, term),
            FlyTermBase::Sol(term) => term.data_inner(terms.sol_terms()).into(),
            FlyTermBase::Hol(term) => term.fly_base_ty_data(db, terms),
            FlyTermBase::Place => todo!(),
        }
    }

    /// `None` means the notion is not applicable,
    /// because the term is either a non type or a conceptual type
    #[deprecated(note = "ad hoc implementation")]
    pub fn is_always_copyable<'db>(
        self,
        db: &'db ::salsa::Db,
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
                parameter_hvar,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => Ok(None),
            FlyBaseTypeData::Hole(hole_kind, _) => match hole_kind {
                HoleKind::UnspecifiedIntegerType | HoleKind::UnspecifiedFloatType => Ok(Some(true)),
                HoleKind::ImplicitType => todo!(),
                HoleKind::AnyOriginal | HoleKind::AnyDerived => todo!(),
            },
            FlyBaseTypeData::Sort(_) => Ok(None),
            FlyBaseTypeData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => match ritchie_kind {
                RitchieKind::Type(_) => Ok(Some(true)),
                RitchieKind::Trait(_) => todo!(),
            },
            FlyBaseTypeData::SymbolicVariable {
                symbolic_variable: term,
            } => Ok(Some(false)),
            FlyBaseTypeData::LambdaVariable {
                lambda_variable: hvar,
            } => todo!(), // ad hoc
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
        if term.quary().is_some() {
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
