use super::*;

#[salsa::interned]
pub struct DecTypeAsTrait {
    pub parent: DecTerm,
    pub trai: DecTerm,
}

impl DecTypeAsTrait {
    #[inline(never)]
    pub(crate) fn display_fmt_with_db_and_ctx(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
        _ctx: &DecSymbolicVariableNameMap,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl DecTermRewriteCopy for DecTypeAsTrait {
    fn substitute_copy(self, db: &::salsa::Db, substitution: &DecTermSubstitution) -> Self
    where
        Self: Copy,
    {
        let old_parent = self.parent(db);
        let parent = old_parent.substitute_copy(db, substitution);
        let old_trai = self.trai(db);
        let trai = old_trai.substitute_copy(db, substitution);
        if old_parent == parent && old_trai == trai {
            return self;
        }
        Self::new(db, parent, trai)
    }
}
