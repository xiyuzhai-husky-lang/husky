use super::*;

impl EtherealTerm {
    pub fn application_expansion(self, db: &dyn EtherealTermDb) -> ApplicationExpansion {
        match self {
            EtherealTerm::Application(term) => term.application_expansion(db),
            EtherealTerm::EntityPath(path) => match path {
                TermEntityPath::Fugitive(_) => todo!(),
                TermEntityPath::Trait(path) => ApplicationExpansion {
                    function: TermFunctionReduced::Trait(path),
                    arguments: None,
                },
                TermEntityPath::TypeOntology(path) => ApplicationExpansion {
                    function: TermFunctionReduced::TypeOntology(path),
                    arguments: None,
                },
                TermEntityPath::TypeInstance(_) => todo!(),
            },
            _ => ApplicationExpansion {
                function: TermFunctionReduced::Other(self),
                arguments: None,
            },
        }
    }
}

impl EtherealTermApplication {
    pub fn application_expansion(self, db: &dyn EtherealTermDb) -> ApplicationExpansion {
        application_expansion_salsa(db, self)
    }
}

#[salsa::tracked(jar= EtherealTermJar)]
pub(crate) fn application_expansion_salsa(
    db: &dyn EtherealTermDb,
    term: EtherealTermApplication,
) -> ApplicationExpansion {
    let function = term.function(db);
    let argument = term.argument(db);
    function.application_expansion(db).apply(db, argument)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EtherealTermDb)]
pub struct ApplicationExpansion {
    function: TermFunctionReduced,
    arguments: Option<EtherealApplicationArguments>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EtherealTermDb)]
pub enum TermFunctionReduced {
    TypeOntology(TypePath),
    Trait(TraitPath),
    Other(EtherealTerm),
}

#[salsa::tracked(db = EtherealTermDb, jar = EtherealTermJar)]
pub(crate) struct EtherealApplicationArguments {
    #[return_ref]
    data: Vec<EtherealTerm>,
}

impl ApplicationExpansion {
    pub fn function(&self) -> TermFunctionReduced {
        self.function
    }

    pub fn opt_arguments<'a>(&self, db: &'a dyn EtherealTermDb) -> Option<&'a [EtherealTerm]> {
        self.arguments.map(|arguments| arguments.data(db) as &[_])
    }

    pub fn arguments<'a>(&self, db: &'a dyn EtherealTermDb) -> &'a [EtherealTerm] {
        self.opt_arguments(db).unwrap_or_default()
    }

    fn apply(&self, db: &dyn EtherealTermDb, argument: EtherealTerm) -> Self {
        let arguments = self.arguments(db);
        let mut arguments = arguments.to_vec();
        arguments.push(argument);
        Self {
            function: self.function,
            arguments: Some(EtherealApplicationArguments::new(db, arguments)),
        }
    }
}
