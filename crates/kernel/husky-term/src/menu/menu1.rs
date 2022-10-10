use husky_print_utils::p;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu1 {
    parent: TermMenu0,
    ty0: Ty,
}

impl TermMenu1 {
    pub fn new(db: &dyn TermQuery, menu0: TermMenu0) -> Self {
        let term = db.it_term(Term::Application(
            TermApplication::new(menu0.sort(), menu0.universe1()).unwrap(),
        ));
        let ty0 = Ty::new(term).unwrap();
        Self { parent: menu0, ty0 }
    }

    pub fn ty0(&self) -> Ty {
        self.ty0
    }
}
