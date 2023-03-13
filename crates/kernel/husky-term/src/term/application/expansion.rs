use super::*;

pub(crate) fn application_expansion(db: &dyn TermDb, term: Term) -> ApplicationExpansion {
    application_expansion_aux(db, term)
}

fn application_expansion_aux(db: &dyn TermDb, term: Term) -> ApplicationExpansion {
    match term {
        Term::Application(term) => application_expansion_salsa(db, term),
        Term::EntityPath(path) => match path {
            TermEntityPath::Form(_) => todo!(),
            TermEntityPath::Trait(_) => todo!(),
            TermEntityPath::TypeOntology(path) => ApplicationExpansion {
                function: TermFunctionReduced::TypeOntology(path),
                arguments: None,
            },
            TermEntityPath::TypeConstructor(_) => todo!(),
        },
        _ => todo!(),
    }
}

#[salsa::tracked(jar= TermJar)]
pub(crate) fn application_expansion_salsa(
    db: &dyn TermDb,
    term: TermApplication,
) -> ApplicationExpansion {
    let function = term.function(db);
    let argument = term.argument(db);
    application_expansion_aux(db, function).apply(db, argument)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TermDb, jar = RawTypeJar)]
pub struct ApplicationExpansion {
    function: TermFunctionReduced,
    arguments: Option<ApplicationArguments>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TermFunctionReduced {
    TypeOntology(TypePath),
}

#[salsa::tracked(db = TermDb, jar = TermJar)]
pub(crate) struct ApplicationArguments {
    #[return_ref]
    data: Vec<Term>,
}

impl ApplicationExpansion {
    pub fn function(&self) -> TermFunctionReduced {
        self.function
    }

    pub fn opt_arguments<'a>(&self, db: &'a dyn TermDb) -> Option<&'a [Term]> {
        self.arguments.map(|arguments| arguments.data(db) as &[_])
    }

    pub fn arguments<'a>(&self, db: &'a dyn TermDb) -> &'a [Term] {
        self.opt_arguments(db).unwrap_or_default()
    }

    fn apply(&self, db: &dyn TermDb, argument: Term) -> Self {
        let arguments = self.arguments(db);
        let mut arguments = arguments.to_vec();
        arguments.push(argument);
        Self {
            function: self.function,
            arguments: Some(ApplicationArguments::new(db, arguments)),
        }
    }
}
