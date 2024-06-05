mod narrative;

use super::*;
use crate::region::SynExprDecTermRegion;
use husky_syn_decl::decl::crate_::lib::{LibCrateSynDecl, LibCrateSynDeclItem};
use husky_syn_expr::expr::SynExprIdx;

#[salsa::tracked]
pub struct LibCrateDecSignature {
    #[id]
    crate_path: CratePath,
    default_const_excludes: Option<SmallVec<[DecTerm; 2]>>,
}

impl LibCrateDecSignature {
    pub(super) fn from_decl(
        crate_path: CratePath,
        syn_decl: impl Into<Option<LibCrateSynDecl>>,
        db: &::salsa::Db,
    ) -> DecSignatureResult<Self> {
        LibCrateDecSignatureBuilder::new(crate_path, syn_decl.into(), db).build()
    }
}

struct LibCrateDecSignatureBuilder<'db> {
    db: &'db ::salsa::Db,
    crate_path: CratePath,
    syn_decl: Option<LibCrateSynDecl>,
    dec_term_region: Option<&'db SynExprDecTermRegion>,
    default_const_excludes: Option<SmallVec<[DecTerm; 2]>>,
}

impl<'db> LibCrateDecSignatureBuilder<'db> {
    fn new(crate_path: CratePath, syn_decl: Option<LibCrateSynDecl>, db: &'db ::salsa::Db) -> Self {
        Self {
            db,
            crate_path,
            syn_decl,
            dec_term_region: syn_decl
                .map(|syn_decl| syn_expr_dec_term_region(db, syn_decl.syn_expr_region(db))),
            default_const_excludes: None,
        }
    }

    fn expr_term(&self, expr: SynExprIdx) -> SynExprDecTermResultRef<'db, DecTerm> {
        self.dec_term_region.unwrap().expr_term(expr)
    }

    fn build(mut self) -> DecSignatureResult<LibCrateDecSignature> {
        let db = self.db;
        let Some(syn_decl) = self.syn_decl else {
            return Ok(self.finish());
        };
        for decl_item in syn_decl.items(db) {
            self.build_item(decl_item)?
        }
        Ok(self.finish())
    }

    fn build_item(&mut self, decl_item: &LibCrateSynDeclItem) -> DecSignatureResult<()> {
        match decl_item {
            LibCrateSynDeclItem::Narrative { narrative, .. } => self.build_narrative(narrative),
        }
    }

    fn finish(self) -> LibCrateDecSignature {
        LibCrateDecSignature::new(self.db, self.crate_path, self.default_const_excludes)
    }
}
