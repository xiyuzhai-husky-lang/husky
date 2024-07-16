use super::*;

impl EthApplication {
    pub(in crate::term) fn reduce(self, db: &::salsa::Db) -> EthTerm {
        reduce_eth_application(db, self)
    }
}

#[salsa::tracked]
fn reduce_eth_application(db: &::salsa::Db, eth_application: EthApplication) -> EthTerm {
    let function = eth_application.function(db).reduce(db);
    let argument = eth_application.argument(db).reduce(db);
    let shift = eth_application.shift(db);
    match function {
        EthTerm::ItemPath(ItemPathTerm::MajorForm(_)) => todo!(),
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
