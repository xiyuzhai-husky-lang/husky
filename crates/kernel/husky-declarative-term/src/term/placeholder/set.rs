use super::*;
use vec_like::VecSet;

/// unlike DeclarativeTermSymbols
/// Some(DeclarativeTermPlaceholders { unaccounted_variables: Default::default() })
/// means different from None
///
/// the former implies that variables exists, but all accounted
#[salsa::tracked(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct DeclarativeTermPlaceholders {
    /// unaccounted means the variable is not declared within this term
    #[return_ref]
    pub unaccounted_variables: VecSet<DeclarativeTermPlaceholder>,
}

impl DeclarativeTermPlaceholders {
    #[inline(always)]
    pub(crate) fn contains(
        self,
        db: &dyn DeclarativeTermDb,
        variable: DeclarativeTermPlaceholder,
    ) -> bool {
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
        _variable: impl Into<Option<DeclarativeTermPlaceholder>>,
    ) -> Option<Self> {
        let _variables = variables.into()?;
        todo!()
    }
}
impl DeclarativeTerm {
    pub fn contains_variable(
        self,
        db: &dyn DeclarativeTermDb,
        variable: DeclarativeTermPlaceholder,
    ) -> bool {
        self.variables(db)
            .map(|raw_term_variables| raw_term_variables.contains(db, variable))
            .unwrap_or_default()
    }

    pub(crate) fn variables(
        self,
        db: &dyn DeclarativeTermDb,
    ) -> Option<DeclarativeTermPlaceholders> {
        match self {
            DeclarativeTerm::Literal(_) => todo!(),
            DeclarativeTerm::Hole(variable) => Some(DeclarativeTermPlaceholders::new(
                db,
                VecSet::new_one_elem_set(variable),
            )),
            DeclarativeTerm::Symbol(symbol) => None,
            DeclarativeTerm::EntityPath(path) => match path {
                DeclarativeTermEntityPath::Form(_) => todo!(),
                DeclarativeTermEntityPath::Trait(_) | DeclarativeTermEntityPath::Type(_) => None,
            },
            DeclarativeTerm::Category(_) => None,
            DeclarativeTerm::Universe(_) => None,
            DeclarativeTerm::Curry(raw_term) => raw_term_curry_placeholders(db, raw_term),
            DeclarativeTerm::Ritchie(raw_term) => raw_term_ritchie_variables(db, raw_term),
            DeclarativeTerm::Abstraction(_) => todo!(),
            DeclarativeTerm::ExplicitApplication(raw_term) => {
                raw_term_application_variables(db, raw_term)
            }
            DeclarativeTerm::ExplicitApplicationOrRitchieCall(_raw_ty) => todo!(),
            DeclarativeTerm::Subentity(_) => todo!(),
            DeclarativeTerm::AsTraitSubentity(_) => todo!(),
            DeclarativeTerm::TraitConstraint(_) => todo!(),
            DeclarativeTerm::LeashOrBitNot(_) => todo!(),
            DeclarativeTerm::List(_) => todo!(),
        }
    }
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn raw_term_curry_placeholders(
    db: &dyn DeclarativeTermDb,
    term: DeclarativeTermCurry,
) -> Option<DeclarativeTermPlaceholders> {
    let parameter_ty_variables = term.parameter_ty(db).variables(db);
    let return_ty_variables = term.return_ty(db).variables(db);
    DeclarativeTermPlaceholders::merge(
        parameter_ty_variables,
        DeclarativeTermPlaceholders::remove(return_ty_variables, term.parameter_variable(db)),
    )
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn raw_term_ritchie_variables(
    db: &dyn DeclarativeTermDb,
    term: DeclarativeTermRitchie,
) -> Option<DeclarativeTermPlaceholders> {
    let mut variables: Option<DeclarativeTermPlaceholders> = None;
    for parameter_ty in term.parameter_tys(db) {
        variables = DeclarativeTermPlaceholders::merge(variables, parameter_ty.ty().variables(db))
    }
    DeclarativeTermPlaceholders::merge(variables, term.return_ty(db).variables(db))
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn raw_term_application_variables(
    db: &dyn DeclarativeTermDb,
    term: DeclarativeTermExplicitApplication,
) -> Option<DeclarativeTermPlaceholders> {
    DeclarativeTermPlaceholders::merge(
        term.function(db).variables(db),
        term.argument(db).variables(db),
    )
}
