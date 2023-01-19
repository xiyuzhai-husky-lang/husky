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
        // todo!()
        Self {
            ty0: TermApplication::new(db, menu0.sort().into(), menu0.universe1().into()).into(),
            parent: menu0,
        }
    }

    pub fn ty0(&self) -> Term {
        self.ty0
    }
}
