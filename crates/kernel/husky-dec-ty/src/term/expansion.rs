use super::*;
// deprecated
fn application_expansion_aux(db: &::salsa::Db, declarative_term: DecTerm) -> ApplicationExpansion {
    match declarative_term {
        DecTerm::Application(declarative_term) => application_expansion_salsa(db, declarative_term),
        _ => ApplicationExpansion {
            f: declarative_term,
            arguments: None,
        },
    }
}

#[salsa::tracked(jar=DeclarativeTypeJar)]
pub(crate) fn application_expansion_salsa(
    db: &::salsa::Db,
    declarative_term: ApplicationDecTerm,
) -> ApplicationExpansion {
    let function = declarative_term.function(db);
    let argument = declarative_term.argument(db);
    application_expansion_aux(db, function).apply(db, argument)
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ApplicationExpansion {
    f: DecTerm,
    arguments: Option<EtherealApplicationArguments>,
}

#[salsa::tracked(db = DeclarativeTypeDb, jar = DeclarativeTypeJar)]
pub(crate) struct EtherealApplicationArguments {
    #[return_ref]
    data: Vec<DecTerm>,
}

impl ApplicationExpansion {
    pub fn f(&self) -> DecTerm {
        self.f
    }

    pub fn opt_arguments<'a>(&self, db: &'a ::salsa::Db) -> Option<&'a [DecTerm]> {
        self.arguments.map(|arguments| arguments.data(db) as &[_])
    }

    pub fn arguments<'a>(&self, db: &'a ::salsa::Db) -> &'a [DecTerm] {
        self.opt_arguments(db).unwrap_or_default()
    }

    fn apply(&self, db: &::salsa::Db, argument: DecTerm) -> Self {
        let arguments = self.arguments(db);
        let mut arguments = arguments.to_vec();
        arguments.push(argument);
        Self {
            f: self.f,
            arguments: Some(EtherealApplicationArguments::new(db, arguments)),
        }
    }
}
