use lean_coword::ident::LnIdent;
use lean_mir_expr::{
    builder::LeanHirExprBuilder,
    expr::LnMirExprArena,
    item_defn::{LnItemDefnArena, LnItemDefnData, LnItemDefnIdxRange},
    stmt::LnMirStmtArena,
    tactic::LnMirTacticArena,
};
use salsa::Db;
use std::ops::{Deref, DerefMut};
use visored_mir_expr::{
    expr::VdMirExprArenaRef,
    region::VdMirExprRegionData,
    stmt::VdMirStmtArenaRef,
    symbol::local_defn::{storage::VdMirSymbolLocalDefnStorage, VdMirSymbolLocalDefnIdx},
};

use crate::{dictionary::VdLeanTranspilationDictionary, mangle::VdLeanTranspilationMangler};

pub struct VdLeanTranspilationBuilder<'a> {
    lean_hir_expr_builder: LeanHirExprBuilder<'a>,
    expr_arena: VdMirExprArenaRef<'a>,
    stmt_arena: VdMirStmtArenaRef<'a>,
    dictionary: &'a VdLeanTranspilationDictionary,
    mangler: VdLeanTranspilationMangler,
}

impl<'a> VdLeanTranspilationBuilder<'a> {
    pub fn new0(
        db: &'a ::salsa::Db,
        vd_hir_expr_region_data: &'a VdMirExprRegionData,
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
        expr_arena: VdMirExprArenaRef<'a>,
        stmt_arena: VdMirStmtArenaRef<'a>,
        symbol_local_defn_storage: &'a VdMirSymbolLocalDefnStorage,
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

    pub(crate) fn mangled_symbol(&mut self, symbol_local_defn: VdMirSymbolLocalDefnIdx) -> LnIdent {
        self.mangler.mangled_symbol(symbol_local_defn)
    }
}

impl<'db> VdLeanTranspilationBuilder<'db> {
    pub fn expr_arena(&self) -> VdMirExprArenaRef<'db> {
        self.expr_arena
    }

    pub fn stmt_arena(&self) -> VdMirStmtArenaRef<'db> {
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
        LnMirExprArena,
        LnMirStmtArena,
        LnMirTacticArena,
        LnItemDefnArena,
    ) {
        self.lean_hir_expr_builder.finish()
    }
}
