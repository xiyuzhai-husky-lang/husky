use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu1 {
    ty0: Term,
    parent: TermMenu0,
}

impl std::ops::Deref for TermMenu1 {
    type Target = TermMenu0;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl TermMenu1 {
    pub fn new(db: &dyn TermDb, _toolchain: Toolchain, menu0: TermMenu0) -> Self {
        let ty0 = db.it_term(TermData::Application(
            TermApplication::new(menu0.sort(), menu0.universe1()).unwrap(),
        ));
        Self { parent: menu0, ty0 }
    }

    pub fn ty0(&self) -> Term {
        self.ty0
    }
}
