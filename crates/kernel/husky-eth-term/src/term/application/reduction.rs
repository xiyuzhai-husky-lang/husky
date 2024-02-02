use super::*;

impl EthApplication {
    pub(in crate::term) fn reduce(self, db: &::salsa::Db) -> EthTerm {
        reduce_term_application(db, self)
    }
}

#[salsa::tracked(jar = EthTermJar)]
pub(crate) fn reduce_term_application(
    db: &::salsa::Db,
    term_application: EthApplication,
) -> EthTerm {
    let function = term_application.function(db).reduce(db);
    let argument = term_application.argument(db).reduce(db);
    let shift = term_application.shift(db);
    match function {
        EthTerm::EntityPath(ItemPathTerm::Fugitive(_)) => todo!(),
        EthTerm::Ritchie(_) => todo!(),
        EthTerm::Abstraction(_) => todo!(),
        EthTerm::Application(function_term_application)
            if let function_shift = function_term_application.shift(db)
                && function_shift > 0 =>
        {
            EthApplication::new_reduced(
                db,
                function_term_application.function(db),
                EthApplication::new_reduced(
                    db,
                    function_term_application.argument(db),
                    argument,
                    shift,
                ),
                function_shift + shift - 1,
            )
        }
        _ => EthApplication::new_inner(db, function, argument, shift).into(),
    }
}
