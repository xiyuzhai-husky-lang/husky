use super::*;
use vec_like::VecSet;

/// unlike RawTermSymbols
/// Some(RawTermPlaceholders { unaccounted_variables: Default::default() })
/// means different from None
///
/// the former implies that variables exists, but all accounted
#[salsa::tracked(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermPlaceholders {
    /// unaccounted means the variable is not declared within this term
    #[return_ref]
    pub unaccounted_variables: VecSet<RawTermPlaceholder>,
}

impl RawTermPlaceholders {
    #[inline(always)]
    pub(crate) fn contains(self, db: &dyn RawTermDb, variable: RawTermPlaceholder) -> bool {
        self.unaccounted_variables(db).has(variable)
    }

    #[inline(always)]
    fn merge(fst: impl Into<Option<Self>>, snd: impl Into<Option<Self>>) -> Option<Self> {
        let fst: Option<_> = fst.into();
        let snd: Option<_> = snd.into();
        match (fst, snd) {
            (None, None) => None,
            (None, Some(snd)) => Some(snd),
            (Some(fst), None) => Some(fst),
            (Some(_fst), Some(_snd)) => todo!(),
        }
    }

    #[inline(always)]
    fn remove(
        variables: impl Into<Option<Self>>,
        _variable: impl Into<Option<RawTermPlaceholder>>,
    ) -> Option<Self> {
        let _variables = variables.into()?;
        todo!()
    }
}
impl RawTerm {
    pub fn contains_variable(self, db: &dyn RawTermDb, variable: RawTermPlaceholder) -> bool {
        self.variables(db)
            .map(|raw_term_variables| raw_term_variables.contains(db, variable))
            .unwrap_or_default()
    }

    pub(crate) fn variables(self, db: &dyn RawTermDb) -> Option<RawTermPlaceholders> {
        match self {
            RawTerm::Literal(_) => todo!(),
            RawTerm::Hole(variable) => Some(RawTermPlaceholders::new(
                db,
                VecSet::new_one_elem_set(variable),
            )),
            RawTerm::Symbol(symbol) => None,
            RawTerm::EntityPath(path) => match path {
                RawTermEntityPath::Form(_) => todo!(),
                RawTermEntityPath::Trait(_) | RawTermEntityPath::Type(_) => None,
            },
            RawTerm::Category(_) => None,
            RawTerm::Universe(_) => None,
            RawTerm::Curry(raw_term) => raw_term_curry_placeholders(db, raw_term),
            RawTerm::Ritchie(raw_term) => raw_term_ritchie_variables(db, raw_term),
            RawTerm::Abstraction(_) => todo!(),
            RawTerm::ExplicitApplication(raw_term) => raw_term_application_variables(db, raw_term),
            RawTerm::ExplicitApplicationOrRitchieCall(_raw_ty) => todo!(),
            RawTerm::Subentity(_) => todo!(),
            RawTerm::AsTraitSubentity(_) => todo!(),
            RawTerm::TraitConstraint(_) => todo!(),
            RawTerm::LeashOrBitNot(_) => todo!(),
            RawTerm::List(_) => todo!(),
        }
    }
}

#[salsa::tracked(jar = RawTermJar)]
pub(crate) fn raw_term_curry_placeholders(
    db: &dyn RawTermDb,
    term: RawTermCurry,
) -> Option<RawTermPlaceholders> {
    let parameter_ty_variables = term.parameter_ty(db).variables(db);
    let return_ty_variables = term.return_ty(db).variables(db);
    RawTermPlaceholders::merge(
        parameter_ty_variables,
        RawTermPlaceholders::remove(return_ty_variables, term.parameter_variable(db)),
    )
}

#[salsa::tracked(jar = RawTermJar)]
pub(crate) fn raw_term_ritchie_variables(
    db: &dyn RawTermDb,
    term: RawTermRitchie,
) -> Option<RawTermPlaceholders> {
    let mut variables: Option<RawTermPlaceholders> = None;
    for parameter_ty in term.parameter_tys(db) {
        variables = RawTermPlaceholders::merge(variables, parameter_ty.ty().variables(db))
    }
    RawTermPlaceholders::merge(variables, term.return_ty(db).variables(db))
}

#[salsa::tracked(jar = RawTermJar)]
pub(crate) fn raw_term_application_variables(
    db: &dyn RawTermDb,
    term: RawTermExplicitApplication,
) -> Option<RawTermPlaceholders> {
    RawTermPlaceholders::merge(
        term.function(db).variables(db),
        term.argument(db).variables(db),
    )
}
