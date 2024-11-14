use lean_coword::ident::LnIdent;
use lean_mir_expr::{
    builder::LeanHirExprBuilder,
    expr::LnHirExprArena,
    item_defn::{LnItemDefnArena, LnItemDefnData, LnItemDefnIdxRange},
    stmt::LnHirStmtArena,
    tactic::LnHirTacticArena,
};
use salsa::Db;
use std::ops::{Deref, DerefMut};
use visored_mir_expr::{
    expr::VdHirExprArenaRef,
    region::VdHirExprRegionData,
    stmt::VdHirStmtArenaRef,
    symbol::local_defn::{storage::VdHirSymbolLocalDefnStorage, VdHirSymbolLocalDefnIdx},
};

use crate::{dictionary::VdLeanTranspilationDictionary, mangle::VdLeanTranspilationMangler};

pub struct VdLeanTranspilationBuilder<'a> {
    lean_hir_expr_builder: LeanHirExprBuilder<'a>,
    expr_arena: VdHirExprArenaRef<'a>,
    stmt_arena: VdHirStmtArenaRef<'a>,
    dictionary: &'a VdLeanTranspilationDictionary,
    mangler: VdLeanTranspilationMangler,
}

impl<'a> VdLeanTranspilationBuilder<'a> {
    pub fn new0(
        db: &'a ::salsa::Db,
        vd_hir_expr_region_data: &'a VdHirExprRegionData,
        dictionary: &'a VdLeanTranspilationDictionary,
    ) -> Self {
        Self::new(
            db,
            vd_hir_expr_region_data.expr_arena(),
            vd_hir_expr_region_data.stmt_arena(),
            vd_hir_expr_region_data.symbol_local_defn_storage(),
            dictionary,
        )
    }

    pub fn new(
        db: &'a ::salsa::Db,
        expr_arena: VdHirExprArenaRef<'a>,
        stmt_arena: VdHirStmtArenaRef<'a>,
        symbol_local_defn_storage: &'a VdHirSymbolLocalDefnStorage,
        dictionary: &'a VdLeanTranspilationDictionary,
    ) -> Self {
        Self {
            lean_hir_expr_builder: LeanHirExprBuilder::new(db),
            expr_arena,
            stmt_arena,
            dictionary,
            mangler: VdLeanTranspilationMangler::new(symbol_local_defn_storage, db),
        }
    }

    pub(crate) fn mangled_symbol(&mut self, symbol_local_defn: VdHirSymbolLocalDefnIdx) -> LnIdent {
        self.mangler.mangled_symbol(symbol_local_defn)
    }
}

impl<'db> VdLeanTranspilationBuilder<'db> {
    pub fn expr_arena(&self) -> VdHirExprArenaRef<'db> {
        self.expr_arena
    }

    pub fn stmt_arena(&self) -> VdHirStmtArenaRef<'db> {
        self.stmt_arena
    }

    pub fn dictionary(&self) -> &VdLeanTranspilationDictionary {
        self.dictionary
    }
}

impl<'db> Deref for VdLeanTranspilationBuilder<'db> {
    type Target = LeanHirExprBuilder<'db>;

    fn deref(&self) -> &Self::Target {
        &self.lean_hir_expr_builder
    }
}

impl<'db> DerefMut for VdLeanTranspilationBuilder<'db> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lean_hir_expr_builder
    }
}

impl<'db> VdLeanTranspilationBuilder<'db> {
    pub fn finish(
        self,
    ) -> (
        LnHirExprArena,
        LnHirStmtArena,
        LnHirTacticArena,
        LnItemDefnArena,
    ) {
        self.lean_hir_expr_builder.finish()
    }
}
