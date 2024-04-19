use husky_eth_term::term::{
    application::{EthApplication, TermFunctionReduced},
    ritchie::EthRitchie,
};

use super::*;

pub(super) fn ethereal_term_data<'a>(db: &'a ::salsa::Db, term: EthTerm) -> FlyTermData<'a> {
    match term {
        EthTerm::Literal(lit) => FlyTermData::Literal(lit),
        EthTerm::Symbol(term) => FlyTermData::SymbolicVariable {
            symbolic_variable: term,
            ty: term.ty(db).into(),
        },
        EthTerm::Hvar(term) => FlyTermData::LambdaVariable {
            ty: term.ty(db).into(),
            index: term.index(db),
        },
        EthTerm::EntityPath(path) => match path {
            ItemPathTerm::Fugitive(_) => todo!(),
            ItemPathTerm::Trait(_) => todo!(),
            ItemPathTerm::TypeOntology(ty_path) => FlyTermData::TypeOntology {
                ty_path,
                refined_ty_path: ty_path.refine(db),
                ty_arguments: &[],
                ty_ethereal_term: Some(path.into()),
            },
            ItemPathTerm::TypeInstance(_) => todo!(),
            ItemPathTerm::TypeVariant(path) => FlyTermData::TypeVariant { path },
        },
        EthTerm::Category(term) => FlyTermData::Sort(term),
        EthTerm::Universe(_) => todo!(),
        EthTerm::Curry(term) => FlyTermData::Curry {
            toolchain: term.toolchain(db),
            curry_kind: term.curry_kind(db),
            variance: term.variance(db),
            parameter_hvar: term.parameter_hvar(db).map(Into::into),
            parameter_ty: term.parameter_ty(db).into(),
            return_ty: term.return_ty(db).into(),
            ty_ethereal_term: Some(term),
        },
        EthTerm::Ritchie(term) => term_ritchie_fly_data(db, term).as_ref(),
        EthTerm::Abstraction(_) => todo!(),
        EthTerm::Application(term) => term_application_fly_data(db, term).as_ref(),
        EthTerm::TypeAsTraitItem(_) => todo!(),
        EthTerm::TraitConstraint(_) => todo!(),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TermRitchieFlyData {
    ritchie_kind: RitchieKind,
    parameter_contracted_tys: SmallVec<[FlyRitchieParameter; 2]>,
    variadics: (),
    keyed_parameter_contracted_tys: (),
    return_ty: EthTerm,
}

impl TermRitchieFlyData {
    fn as_ref<'a>(&'a self) -> FlyTermData<'a> {
        FlyTermData::Ritchie {
            ritchie_kind: self.ritchie_kind,
            parameter_contracted_tys: &self.parameter_contracted_tys,
            return_ty: self.return_ty.into(),
        }
    }

    fn as_ref2<'a>(&'a self) -> FlyBaseTypeData<'a> {
        FlyBaseTypeData::Ritchie {
            ritchie_kind: self.ritchie_kind,
            parameter_contracted_tys: &self.parameter_contracted_tys,
            return_ty: self.return_ty.into(),
        }
    }
}

#[salsa::tracked(jar = FlyTermJar, return_ref)]
pub(crate) fn term_ritchie_fly_data(db: &::salsa::Db, term: EthRitchie) -> TermRitchieFlyData {
    TermRitchieFlyData {
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
pub(crate) enum TermApplicationFlyData {
    TypeOntology {
        path: TypePath,
        refined_path: Either<PreludeTypePath, CustomTypePath>,
        arguments: SmallVec<[FlyTerm; 2]>,
        ty_ethereal_term: EthTerm,
    },
}

/// can't directly return FlyTermData<'_> because of lifetime
#[salsa::tracked(jar = FlyTermJar, return_ref)]
pub(crate) fn term_application_fly_data(
    db: &::salsa::Db,
    term: EthApplication,
) -> TermApplicationFlyData {
    let expansion = term.application_expansion(db);
    match expansion.function() {
        TermFunctionReduced::TypeOntology(path) => TermApplicationFlyData::TypeOntology {
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

impl TermApplicationFlyData {
    fn as_ref<'a>(&'a self) -> FlyTermData<'a> {
        match self {
            TermApplicationFlyData::TypeOntology {
                path,
                refined_path,
                arguments,
                ty_ethereal_term,
            } => FlyTermData::TypeOntology {
                ty_path: *path,
                refined_ty_path: *refined_path,
                ty_arguments: arguments,
                ty_ethereal_term: Some(*ty_ethereal_term),
            },
        }
    }

    fn as_ref2<'a>(&'a self) -> FlyBaseTypeData<'a> {
        match self {
            TermApplicationFlyData::TypeOntology {
                path,
                refined_path,
                arguments,
                ty_ethereal_term,
            } => FlyBaseTypeData::TypeOntology {
                ty_path: *path,
                refined_ty_path: *refined_path,
                ty_arguments: arguments,
                ty_ethereal_term: Some(*ty_ethereal_term),
            },
        }
    }
}

pub(super) fn ethereal_term_fly_base_ty_data<'a>(
    db: &'a ::salsa::Db,
    term: EthTerm,
) -> FlyBaseTypeData<'a> {
    match term {
        EthTerm::Literal(_) => todo!(),
        EthTerm::Symbol(symbol) => FlyBaseTypeData::SymbolicVariable {
            symbolic_variable: symbol,
        },
        EthTerm::Hvar(hvar) => FlyBaseTypeData::LambdaVariable {
            lambda_variable: hvar,
        },
        EthTerm::EntityPath(path) => match path {
            ItemPathTerm::Fugitive(_) => todo!(),
            ItemPathTerm::Trait(_) => todo!(),
            ItemPathTerm::TypeOntology(ty_path) => FlyBaseTypeData::TypeOntology {
                ty_path,
                refined_ty_path: ty_path.refine(db),
                ty_arguments: &[],
                ty_ethereal_term: Some(path.into()),
            },
            ItemPathTerm::TypeInstance(_) => todo!(),
            ItemPathTerm::TypeVariant(path) => unreachable!(),
        },
        EthTerm::Category(term) => FlyBaseTypeData::Category(term),
        EthTerm::Universe(_) => todo!(),
        EthTerm::Curry(term) => FlyBaseTypeData::Curry {
            curry_kind: term.curry_kind(db),
            variance: term.variance(db),
            parameter_hvar: term.parameter_hvar(db).map(Into::into),
            parameter_ty: term.parameter_ty(db).into(),
            return_ty: term.return_ty(db).into(),
            ty_ethereal_term: Some(term),
        },
        EthTerm::Ritchie(term) => term_ritchie_fly_data(db, term).as_ref2(),
        EthTerm::Abstraction(_) => todo!(),
        EthTerm::Application(term) => term_application_fly_data(db, term).as_ref2(),
        EthTerm::TypeAsTraitItem(_) => todo!(),
        EthTerm::TraitConstraint(_) => todo!(),
    }
}
