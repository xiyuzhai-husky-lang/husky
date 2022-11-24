use salsa::{storage::HasJar, DbWithJar};

use crate::*;

pub trait SymbolDb: DbWithJar<SymbolJar> {
    fn symbol_jar(&self) -> &SymbolJar;
    fn new_symbol_ctx<'a>(&'a self) -> SymbolContext<'a>;
    fn preludes(&self) -> &[Symbol];
    fn collect_preludes(&self) -> Vec<Symbol>;
}

impl<T> SymbolDb for T
where
    T: DbWithJar<SymbolJar>,
{
    fn symbol_jar(&self) -> &SymbolJar {
        &<Self as HasJar<SymbolJar>>::jar(&self).0
    }
    fn new_symbol_ctx<'a>(&'a self) -> SymbolContext<'a> {
        SymbolContext::new(self.preludes())
    }

    fn preludes(&self) -> &[Symbol] {
        self.symbol_jar()
            .preludes_place()
            .get_or_init(|| self.collect_preludes())
    }

    fn collect_preludes(&self) -> Vec<Symbol> {
        // ad hoc
        vec![]
    }
}
