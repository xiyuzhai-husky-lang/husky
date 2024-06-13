use super::*;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;

#[salsa::interned]
pub struct DecTypeAsTraitItem {
    pub self_ty: DecTerm,
    pub trai: DecTerm,
    pub ident: Ident,
}

impl DecTypeAsTraitItem {
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

impl DecTermRewriteCopy for DecTypeAsTraitItem {
    fn substitute_copy(self, db: &::salsa::Db, substitution: &DecTermSubstitution) -> Self
    where
        Self: Copy,
    {
        let old_self_ty = self.self_ty(db);
        let self_ty = old_self_ty.substitute_copy(db, substitution);
        let old_trai = self.trai(db);
        let trai = old_trai.substitute_copy(db, substitution);
        if old_self_ty == self_ty && old_trai == trai {
            return self;
        }
        let ident = self.ident(db);
        Self::new(db, self_ty, trai, ident)
    }
}
