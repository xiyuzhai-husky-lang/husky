use super::*;
// deprecated
fn application_expansion_aux(db: &dyn RawTypeDb, raw_term: RawTerm) -> ApplicationExpansion {
    match raw_term {
        RawTerm::ExplicitApplication(raw_term) => application_expansion_salsa(db, raw_term),
        _ => ApplicationExpansion {
            f: raw_term,
            arguments: None,
        },
    }
}

#[salsa::tracked(jar=RawTypeJar)]
pub(crate) fn application_expansion_salsa(
    db: &dyn RawTypeDb,
    raw_term: RawTermExplicitApplication,
) -> ApplicationExpansion {
    let function = raw_term.function(db);
    let argument = raw_term.argument(db);
    application_expansion_aux(db, function).apply(db, argument)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = RawTypeDb, jar = RawTypeJar)]
pub struct ApplicationExpansion {
    f: RawTerm,
    arguments: Option<ApplicationArguments>,
}

#[salsa::tracked(db = RawTypeDb, jar = RawTypeJar)]
pub(crate) struct ApplicationArguments {
    #[return_ref]
    data: Vec<RawTerm>,
}

impl ApplicationExpansion {
    pub fn f(&self) -> RawTerm {
        self.f
    }

    pub fn opt_arguments<'a>(&self, db: &'a dyn RawTypeDb) -> Option<&'a [RawTerm]> {
        self.arguments.map(|arguments| arguments.data(db) as &[_])
    }

    pub fn arguments<'a>(&self, db: &'a dyn RawTypeDb) -> &'a [RawTerm] {
        self.opt_arguments(db).unwrap_or_default()
    }

    fn apply(&self, db: &dyn RawTypeDb, argument: RawTerm) -> Self {
        let arguments = self.arguments(db);
        let mut arguments = arguments.to_vec();
        arguments.push(argument);
        Self {
            f: self.f,
            arguments: Some(ApplicationArguments::new(db, arguments)),
        }
    }
}
