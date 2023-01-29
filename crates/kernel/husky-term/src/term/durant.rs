pub use context::*;

use crate::*;

/// representing term `x -> y`
#[salsa::interned(db = TermDb, jar = TermJar)]
pub struct TermDurant {
    pub kind: TermDurantKind,
    #[return_ref]
    pub params: Vec<TermDurantParameter>,
    pub y: Term,
    // ty: Term,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TermDurantParameter {
    ty: Term,
}

impl TermDurantParameter {
    pub fn new(ty: Term) -> Self {
        Self { ty }
    }

    pub fn ty(&self) -> Term {
        self.ty
    }
}

impl<Db: TermDb + ?Sized> salsa::DebugWithDb<Db> for TermDurantParameter {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<TermJar>>::as_jar_db(db);
        f.debug_struct("TermDurantParameter")
            .field("ty", &self.ty.debug_with(db, include_all_fields))
            .finish()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TermDurantKind {
    Fp,
    Fn,
    FnMut,
}

impl TermRewriteCopy for TermDurant {
    fn substitute_copy(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Self {
        todo!()
    }
}
