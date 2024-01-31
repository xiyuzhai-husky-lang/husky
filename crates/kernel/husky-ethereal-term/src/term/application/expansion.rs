use super::*;

impl EtherealTerm {
    pub fn application_expansion(self, db: &::salsa::Db) -> ApplicationExpansion {
        match self {
            EtherealTerm::Application(term) => term.application_expansion(db),
            EtherealTerm::EntityPath(path) => match path {
                ItemPathTerm::Fugitive(_) => todo!(),
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

impl ApplicationEtherealTerm {
    pub fn application_expansion(self, db: &::salsa::Db) -> ApplicationExpansion {
        application_expansion_salsa(db, self)
    }
}

#[salsa::tracked(jar= EtherealTermJar)]
pub(crate) fn application_expansion_salsa(
    db: &::salsa::Db,
    term: ApplicationEtherealTerm,
) -> ApplicationExpansion {
    let function = term.function(db);
    let argument = term.argument(db);
    function.application_expansion(db).apply(db, argument)
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ApplicationExpansion {
    function: TermFunctionReduced,
    arguments: Option<EtherealApplicationArguments>,
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

    pub fn opt_arguments<'a>(&self, db: &'a ::salsa::Db) -> Option<&'a [EtherealTerm]> {
        self.arguments.map(|arguments| arguments.data(db) as &[_])
    }

    pub fn arguments<'a>(&self, db: &'a ::salsa::Db) -> &'a [EtherealTerm] {
        self.opt_arguments(db).unwrap_or_default()
    }

    fn apply(&self, db: &::salsa::Db, argument: EtherealTerm) -> Self {
        let arguments = self.arguments(db);
        let mut arguments = arguments.to_vec();
        arguments.push(argument);
        Self {
            function: self.function,
            arguments: Some(EtherealApplicationArguments::new(db, arguments)),
        }
    }
}
