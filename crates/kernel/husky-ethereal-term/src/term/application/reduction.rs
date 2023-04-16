use super::*;

impl EtherealTermApplication {
    pub(in crate::term) fn reduce(self, db: &dyn EtherealTermDb) -> EtherealTerm {
        reduce_term_application(db, self)
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn reduce_term_application(
    db: &dyn EtherealTermDb,
    term_application: EtherealTermApplication,
) -> EtherealTerm {
    let function = term_application.function(db).reduce(db);
    let argument = term_application.argument(db).reduce(db);
    let shift = term_application.shift(db);
    match function {
        EtherealTerm::EntityPath(TermEntityPath::Form(_)) => todo!(),
        EtherealTerm::Ritchie(_) => todo!(),
        EtherealTerm::Abstraction(_) => todo!(),
        EtherealTerm::Application(_) if shift > 0 => {
            p!(function.debug(db), argument.debug(db), shift);
            todo!()
        }
        _ => EtherealTermApplication::new_inner(db, function, argument, shift).into(),
    }
}
