use super::*;
// deprecated
fn application_expansion_aux(
    db: &dyn DeclarativeTypeDb,
    declarative_term: DeclarativeTerm,
) -> ApplicationExpansion {
    match declarative_term {
        DeclarativeTerm::ExplicitApplication(declarative_term) => {
            application_expansion_salsa(db, declarative_term)
        }
        _ => ApplicationExpansion {
            f: declarative_term,
            arguments: None,
        },
    }
}

#[salsa::tracked(jar=DeclarativeTypeJar)]
pub(crate) fn application_expansion_salsa(
    db: &dyn DeclarativeTypeDb,
    declarative_term: DeclarativeTermExplicitApplication,
) -> ApplicationExpansion {
    let function = declarative_term.function(db);
    let argument = declarative_term.argument(db);
    application_expansion_aux(db, function).apply(db, argument)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = DeclarativeTypeDb, jar = DeclarativeTypeJar)]
pub struct ApplicationExpansion {
    f: DeclarativeTerm,
    arguments: Option<EtherealApplicationArguments>,
}

#[salsa::tracked(db = DeclarativeTypeDb, jar = DeclarativeTypeJar)]
pub(crate) struct EtherealApplicationArguments {
    #[return_ref]
    data: Vec<DeclarativeTerm>,
}

impl ApplicationExpansion {
    pub fn f(&self) -> DeclarativeTerm {
        self.f
    }

    pub fn opt_arguments<'a>(
        &self,
        db: &'a dyn DeclarativeTypeDb,
    ) -> Option<&'a [DeclarativeTerm]> {
        self.arguments.map(|arguments| arguments.data(db) as &[_])
    }

    pub fn arguments<'a>(&self, db: &'a dyn DeclarativeTypeDb) -> &'a [DeclarativeTerm] {
        self.opt_arguments(db).unwrap_or_default()
    }

    fn apply(&self, db: &dyn DeclarativeTypeDb, argument: DeclarativeTerm) -> Self {
        let arguments = self.arguments(db);
        let mut arguments = arguments.to_vec();
        arguments.push(argument);
        Self {
            f: self.f,
            arguments: Some(EtherealApplicationArguments::new(db, arguments)),
        }
    }
}
