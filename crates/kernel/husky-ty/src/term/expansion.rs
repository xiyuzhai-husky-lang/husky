use super::*;
pub(crate) fn application_expansion(db: &dyn TypeDb, term: ReducedTerm) -> ApplicationExpansion {
    application_expansion_aux(db, term.term())
}

fn application_expansion_aux(db: &dyn TypeDb, term: Term) -> ApplicationExpansion {
    match term {
        Term::Application(term) => application_expansion_salsa(db, term),
        _ => ApplicationExpansion {
            f: term,
            arguments: None,
        },
    }
}

#[salsa::tracked(jar=TypeJar)]
pub(crate) fn application_expansion_salsa(
    db: &dyn TypeDb,
    term: TermApplication,
) -> ApplicationExpansion {
    let function = term.function(db);
    let argument = term.argument(db);
    application_expansion_aux(db, function).apply(db, argument)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct ApplicationExpansion {
    f: Term,
    arguments: Option<ApplicationArguments>,
}

#[salsa::tracked(db = TypeDb, jar = TypeJar)]
pub(crate) struct ApplicationArguments {
    #[return_ref]
    data: Vec<Term>,
}

impl ApplicationExpansion {
    pub(crate) fn f(&self) -> Term {
        self.f
    }

    pub(crate) fn arguments<'a>(&self, db: &'a dyn TypeDb) -> Option<&'a [Term]> {
        self.arguments.map(|arguments| arguments.data(db) as &[_])
    }

    fn apply(&self, db: &dyn TypeDb, argument: Term) -> Self {
        let arguments = self.arguments(db).unwrap_or_default();
        let mut arguments = arguments.to_vec();
        arguments.push(argument);
        Self {
            f: self.f,
            arguments: Some(ApplicationArguments::new(db, arguments)),
        }
    }
}
