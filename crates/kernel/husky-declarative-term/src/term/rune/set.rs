use super::*;
use vec_like::VecSet;

/// unlike DeclarativeTermSymbols
/// Some(DeclarativeTermRunes { unaccounted_variables: Default::default() })
/// means different from None
///
/// the former implies that variables exists, but all accounted
#[salsa::tracked(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct DeclarativeTermRunes {
    /// unaccounted means the variable is not declared within this term
    #[return_ref]
    pub unaccounted_variables: VecSet<DeclarativeTermRune>,
}

impl DeclarativeTermRunes {
    #[inline(always)]
    pub(crate) fn contains(
        self,
        db: &dyn DeclarativeTermDb,
        variable: DeclarativeTermRune,
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
        _variable: impl Into<Option<DeclarativeTermRune>>,
    ) -> Option<Self> {
        let _variables = variables.into()?;
        todo!()
    }
}
impl DeclarativeTerm {
    pub fn contains_variable(
        self,
        db: &dyn DeclarativeTermDb,
        variable: DeclarativeTermRune,
    ) -> bool {
        self.variables(db)
            .map(|declarative_term_variables| declarative_term_variables.contains(db, variable))
            .unwrap_or_default()
    }

    pub(crate) fn variables(self, db: &dyn DeclarativeTermDb) -> Option<DeclarativeTermRunes> {
        match self {
            DeclarativeTerm::Literal(_) => todo!(),
            DeclarativeTerm::Variable(variable) => Some(DeclarativeTermRunes::new(
                db,
                VecSet::new_one_elem_set(variable),
            )),
            DeclarativeTerm::Symbol(symbol) => None,
            DeclarativeTerm::EntityPath(path) => match path {
                DeclarativeTermEntityPath::Fugitive(_) => todo!(),
                DeclarativeTermEntityPath::Trait(_) | DeclarativeTermEntityPath::Type(_) => None,
                DeclarativeTermEntityPath::TypeVariant(_) => todo!(),
            },
            DeclarativeTerm::Category(_) => None,
            DeclarativeTerm::Universe(_) => None,
            DeclarativeTerm::Curry(declarative_term) => {
                declarative_term_curry_placeholders(db, declarative_term)
            }
            DeclarativeTerm::Ritchie(declarative_term) => {
                declarative_term_ritchie_variables(db, declarative_term)
            }
            DeclarativeTerm::Abstraction(_) => todo!(),
            DeclarativeTerm::ExplicitApplication(declarative_term) => {
                declarative_term_application_variables(db, declarative_term)
            }
            DeclarativeTerm::ExplicitApplicationOrRitchieCall(_declarative_ty) => todo!(),
            DeclarativeTerm::Subitem(_) => todo!(),
            DeclarativeTerm::AsTraitSubitem(_) => todo!(),
            DeclarativeTerm::TraitConstraint(_) => todo!(),
            DeclarativeTerm::LeashOrBitNot(_) => todo!(),
            DeclarativeTerm::List(_) => todo!(),
            DeclarativeTerm::Wrapper(_) => todo!(),
        }
    }
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn declarative_term_curry_placeholders(
    db: &dyn DeclarativeTermDb,
    term: DeclarativeTermCurry,
) -> Option<DeclarativeTermRunes> {
    let parameter_ty_variables = term.parameter_ty(db).variables(db);
    let return_ty_variables = term.return_ty(db).variables(db);
    DeclarativeTermRunes::merge(
        parameter_ty_variables,
        DeclarativeTermRunes::remove(return_ty_variables, term.parameter_variable(db)),
    )
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn declarative_term_ritchie_variables(
    db: &dyn DeclarativeTermDb,
    term: DeclarativeTermRitchie,
) -> Option<DeclarativeTermRunes> {
    let mut variables: Option<DeclarativeTermRunes> = None;
    for param in term.params(db) {
        variables = DeclarativeTermRunes::merge(variables, param.ty().variables(db))
    }
    DeclarativeTermRunes::merge(variables, term.return_ty(db).variables(db))
}

#[salsa::tracked(jar = DeclarativeTermJar)]
pub(crate) fn declarative_term_application_variables(
    db: &dyn DeclarativeTermDb,
    term: DeclarativeTermExplicitApplication,
) -> Option<DeclarativeTermRunes> {
    DeclarativeTermRunes::merge(
        term.function(db).variables(db),
        term.argument(db).variables(db),
    )
}
