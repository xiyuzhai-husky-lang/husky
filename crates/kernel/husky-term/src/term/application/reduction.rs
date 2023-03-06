use super::*;

impl TermApplication {
    pub(in crate::term) fn reduce(self, db: &dyn TermDb) -> Term {
        reduce_term_application(db, self)
    }
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn reduce_term_application(db: &dyn TermDb, term_application: TermApplication) -> Term {
    let function = term_application.function(db).reduce(db);
    let argument = term_application.argument(db).reduce(db);
    let shift = term_application.shift(db);
    match function {
        Term::EntityPath(TermEntityPath::Form(_)) => todo!(),
        Term::Ritchie(_) => todo!(),
        Term::Abstraction(_) => todo!(),
        Term::Application(_) if shift > 0 => {
            p!(function.debug(db), argument.debug(db), shift);
            todo!()
        }
        _ => TermApplication::new_inner(db, function, argument, shift).into(),
    }
}
