use super::*;

impl EthTerm {
    pub fn application_expansion(self, db: &::salsa::Db) -> ApplicationExpansion {
        match self {
            EthTerm::Application(term) => term.application_expansion(db),
            EthTerm::EntityPath(path) => match path {
                ItemPathTerm::Form(_) => todo!(),
                ItemPathTerm::Trait(path) => ApplicationExpansion {
                    function: TermFunctionReduced::Trait(path),
                    arguments: None,
                },
                ItemPathTerm::TypeOntology(path) => ApplicationExpansion {
                    function: TermFunctionReduced::TypeOntology(path),
                    arguments: None,
                },
                ItemPathTerm::TypeInstance(_) => todo!(),
                ItemPathTerm::TypeVariant(_) => todo!(),
            },
            _ => ApplicationExpansion {
                function: TermFunctionReduced::Other(self),
                arguments: None,
            },
        }
    }
}

impl EthApplication {
    pub fn application_expansion(self, db: &::salsa::Db) -> ApplicationExpansion {
        application_expansion_salsa(db, self)
    }
}

#[salsa::tracked(jar= EthTermJar)]
pub(crate) fn application_expansion_salsa(
    db: &::salsa::Db,
    term: EthApplication,
) -> ApplicationExpansion {
    let function = term.function(db);
    let argument = term.argument(db);
    function.application_expansion(db).apply(db, argument)
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ApplicationExpansion {
    function: TermFunctionReduced,
    arguments: Option<EtherealApplicationArguments>,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TermFunctionReduced {
    TypeOntology(TypePath),
    Trait(TraitPath),
    Other(EthTerm),
}

#[salsa::tracked(db = EthTermDb, jar = EthTermJar)]
pub(crate) struct EtherealApplicationArguments {
    #[return_ref]
    data: Vec<EthTerm>,
}

impl ApplicationExpansion {
    pub fn function(&self) -> TermFunctionReduced {
        self.function
    }

    pub fn opt_arguments<'a>(&self, db: &'a ::salsa::Db) -> Option<&'a [EthTerm]> {
        self.arguments.map(|arguments| arguments.data(db) as &[_])
    }

    pub fn arguments<'a>(&self, db: &'a ::salsa::Db) -> &'a [EthTerm] {
        self.opt_arguments(db).unwrap_or_default()
    }

    fn apply(&self, db: &::salsa::Db, argument: EthTerm) -> Self {
        let arguments = self.arguments(db);
        let mut arguments = arguments.to_vec();
        arguments.push(argument);
        Self {
            function: self.function,
            arguments: Some(EtherealApplicationArguments::new(db, arguments)),
        }
    }
}
