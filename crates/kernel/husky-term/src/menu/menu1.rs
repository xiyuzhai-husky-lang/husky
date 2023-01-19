use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu1 {
    parent: TermMenu0,
    ty0: Term,
    eval_ref: Term,
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
            eval_ref: TermApplication::new(db, menu0.ref_ty().into(), menu0.eval_lifetime().into())
                .into(),
            parent: menu0,
        }
    }

    pub fn ty0(&self) -> Term {
        self.ty0
    }

    pub fn eval_ref(&self) -> Term {
        self.eval_ref
    }
}
