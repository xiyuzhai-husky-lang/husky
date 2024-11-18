use crate::{
    division::{VdSynDivisionArenaRef, VdSynDivisionIdxRange, VdSynDivisionMap},
    stmt::{VdSynStmtArenaRef, VdSynStmtMap},
};
use visored_item_path::module::{VdModulePath, VdModulePathRegistry};

pub struct VdSynExprEntityTreeBuilder<'a> {
    db: &'a ::salsa::Db,
    stmt_arena: VdSynStmtArenaRef<'a>,
    division_arena: VdSynDivisionArenaRef<'a>,
    stmt_module_path_map: VdSynStmtMap<VdModulePath>,
    division_module_path_map: VdSynDivisionMap<VdModulePath>,
}

impl<'a> VdSynExprEntityTreeBuilder<'a> {
    pub fn new(
        db: &'a ::salsa::Db,
        stmt_arena: VdSynStmtArenaRef<'a>,
        division_arena: VdSynDivisionArenaRef<'a>,
    ) -> Self {
        Self {
            db,
            stmt_arena,
            division_arena,
            stmt_module_path_map: VdSynStmtMap::new2(stmt_arena),
            division_module_path_map: VdSynDivisionMap::new2(division_arena),
        }
    }
}

impl<'a> VdSynExprEntityTreeBuilder<'a> {
    pub fn build_divisions(
        &mut self,
        divisions: VdSynDivisionIdxRange,
        registry: &mut VdModulePathRegistry,
    ) {
        todo!()
    }
}
