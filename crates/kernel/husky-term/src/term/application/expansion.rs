use super::*;

impl Term {
    pub fn application_expansion(self, db: &dyn TermDb) -> ApplicationExpansion {
        match self {
            Term::Application(term) => term.application_expansion(db),
            Term::EntityPath(path) => match path {
                TermEntityPath::Form(_) => todo!(),
                TermEntityPath::Trait(path) => ApplicationExpansion {
                    function: TermFunctionReduced::Trait(path),
                    arguments: None,
                },
                TermEntityPath::TypeOntology(path) => ApplicationExpansion {
                    function: TermFunctionReduced::TypeOntology(path),
                    arguments: None,
                },
                TermEntityPath::TypeConstructor(_) => todo!(),
            },
            _ => ApplicationExpansion {
                function: TermFunctionReduced::Other(self),
                arguments: None,
            },
        }
    }
}

impl TermApplication {
    pub fn application_expansion(self, db: &dyn TermDb) -> ApplicationExpansion {
        application_expansion_salsa(db, self)
    }
}

#[salsa::tracked(jar= TermJar)]
pub(crate) fn application_expansion_salsa(
    db: &dyn TermDb,
    term: TermApplication,
) -> ApplicationExpansion {
    let function = term.function(db);
    let argument = term.argument(db);
    function.application_expansion(db).apply(db, argument)
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
    Trait(TraitPath),
    Other(Term),
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
