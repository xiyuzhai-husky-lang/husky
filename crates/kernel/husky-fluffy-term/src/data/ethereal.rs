use super::*;

pub(super) fn ethereal_term_data<'a>(db: &'a ::salsa::Db, term: EthTerm) -> FluffyTermData<'a> {
    match term {
        EthTerm::Literal(lit) => FluffyTermData::Literal(lit),
        EthTerm::Symbol(term) => FluffyTermData::Symbol {
            term,
            ty: term.ty(db).into(),
        },
        EthTerm::Rune(term) => FluffyTermData::Rune {
            ty: term.ty(db).into(),
            index: term.index(db),
        },
        EthTerm::EntityPath(path) => match path {
            ItemPathTerm::Fugitive(_) => todo!(),
            ItemPathTerm::Trait(_) => todo!(),
            ItemPathTerm::TypeOntology(ty_path) => FluffyTermData::TypeOntology {
                ty_path,
                refined_ty_path: ty_path.refine(db),
                ty_arguments: &[],
                ty_ethereal_term: Some(path.into()),
            },
            ItemPathTerm::TypeInstance(_) => todo!(),
            ItemPathTerm::TypeVariant(path) => FluffyTermData::TypeVariant { path },
        },
        EthTerm::Category(term) => FluffyTermData::Category(term),
        EthTerm::Universe(_) => todo!(),
        EthTerm::Curry(term) => FluffyTermData::Curry {
            toolchain: term.toolchain(db),
            curry_kind: term.curry_kind(db),
            variance: term.variance(db),
            parameter_rune: term.parameter_rune(db).map(Into::into),
            parameter_ty: term.parameter_ty(db).into(),
            return_ty: term.return_ty(db).into(),
            ty_ethereal_term: Some(term),
        },
        EthTerm::Ritchie(term) => term_ritchie_fluffy_data(db, term).as_ref(),
        EthTerm::Abstraction(_) => todo!(),
        EthTerm::Application(term) => term_application_fluffy_data(db, term).as_ref(),
        EthTerm::TypeAsTraitItem(_) => todo!(),
        EthTerm::TraitConstraint(_) => todo!(),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TermRitchieFluffyData {
    ritchie_kind: RitchieKind,
    parameter_contracted_tys: SmallVec<[FluffyRitchieParameter; 2]>,
    variadics: (),
    keyed_parameter_contracted_tys: (),
    return_ty: EthTerm,
}

impl TermRitchieFluffyData {
    fn as_ref<'a>(&'a self) -> FluffyTermData<'a> {
        FluffyTermData::Ritchie {
            ritchie_kind: self.ritchie_kind,
            parameter_contracted_tys: &self.parameter_contracted_tys,
            return_ty: self.return_ty.into(),
        }
    }

    fn as_ref2<'a>(&'a self) -> FluffyBaseTypeData<'a> {
        FluffyBaseTypeData::Ritchie {
            ritchie_kind: self.ritchie_kind,
            parameter_contracted_tys: &self.parameter_contracted_tys,
            return_ty: self.return_ty.into(),
        }
    }
}

#[salsa::tracked(jar = FluffyTermJar, return_ref)]
pub(crate) fn term_ritchie_fluffy_data(
    db: &::salsa::Db,
    term: RitchieEthTerm,
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
        ty_ethereal_term: EthTerm,
    },
}

/// can't directly return FluffyTermData<'_> because of lifetime
#[salsa::tracked(jar = FluffyTermJar, return_ref)]
pub(crate) fn term_application_fluffy_data(
    db: &::salsa::Db,
    term: ApplicationEthTerm,
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
                ty_arguments: arguments,
                ty_ethereal_term: Some(*ty_ethereal_term),
            },
        }
    }

    fn as_ref2<'a>(&'a self) -> FluffyBaseTypeData<'a> {
        match self {
            TermApplicationFluffyData::TypeOntology {
                path,
                refined_path,
                arguments,
                ty_ethereal_term,
            } => FluffyBaseTypeData::TypeOntology {
                ty_path: *path,
                refined_ty_path: *refined_path,
                ty_arguments: arguments,
                ty_ethereal_term: Some(*ty_ethereal_term),
            },
        }
    }
}

pub(super) fn ethereal_term_fluffy_base_ty_data<'a>(
    db: &'a ::salsa::Db,
    term: EthTerm,
) -> FluffyBaseTypeData<'a> {
    match term {
        EthTerm::Literal(_) => todo!(),
        EthTerm::Symbol(symbol) => FluffyBaseTypeData::Symbol { symbol },
        EthTerm::Rune(rune) => FluffyBaseTypeData::Rune { rune },
        EthTerm::EntityPath(path) => match path {
            ItemPathTerm::Fugitive(_) => todo!(),
            ItemPathTerm::Trait(_) => todo!(),
            ItemPathTerm::TypeOntology(ty_path) => FluffyBaseTypeData::TypeOntology {
                ty_path,
                refined_ty_path: ty_path.refine(db),
                ty_arguments: &[],
                ty_ethereal_term: Some(path.into()),
            },
            ItemPathTerm::TypeInstance(_) => todo!(),
            ItemPathTerm::TypeVariant(path) => unreachable!(),
        },
        EthTerm::Category(term) => FluffyBaseTypeData::Category(term),
        EthTerm::Universe(_) => todo!(),
        EthTerm::Curry(term) => FluffyBaseTypeData::Curry {
            curry_kind: term.curry_kind(db),
            variance: term.variance(db),
            parameter_rune: term.parameter_rune(db).map(Into::into),
            parameter_ty: term.parameter_ty(db).into(),
            return_ty: term.return_ty(db).into(),
            ty_ethereal_term: Some(term),
        },
        EthTerm::Ritchie(term) => term_ritchie_fluffy_data(db, term).as_ref2(),
        EthTerm::Abstraction(_) => todo!(),
        EthTerm::Application(term) => term_application_fluffy_data(db, term).as_ref2(),
        EthTerm::TypeAsTraitItem(_) => todo!(),
        EthTerm::TraitConstraint(_) => todo!(),
    }
}
