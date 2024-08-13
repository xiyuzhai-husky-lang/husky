use super::{source_map::HirLazyExprSourceMapData, HirLazyExprIdx, HirLazyStmtIdx};
use crate::helpers::hir_lazy_expr_source_map;
use husky_entity_path::{path::ItemPath, region::RegionPath};
use husky_regional_token::RegionalTokenIdxBase;
use husky_sem_expr::{helpers::range::sem_expr_range_region, SemExprIdx, SemStmtIdx};
use husky_sem_expr::{helpers::range::SemExprRangeRegionData, SemExprDb, SemExprRegionData};
use husky_syn_defn::{item_syn_defn, ItemSynDefn};
use husky_text::Text;
use husky_token::RangedTokenSheet;
use salsa::DebugWithDb;

pub struct HirLazyExprDebugger<'db> {
    db: &'db ::salsa::Db,
    text: Text<'db>,
    ranged_token_sheet: &'db RangedTokenSheet,
    sem_expr_region_data: &'db SemExprRegionData,
    sem_expr_range_region_data: &'db SemExprRangeRegionData,
    regional_token_idx_base: RegionalTokenIdxBase,
    source_map_data: &'db HirLazyExprSourceMapData,
}

impl<'db> HirLazyExprDebugger<'db> {
    pub fn new_body(item_path: ItemPath, db: &'db ::salsa::Db) -> Option<Self> {
        use husky_entity_tree::helpers::tokra_region::HasRegionalTokenIdxBase;
        use husky_text::HasText;
        use husky_token::jar::TokenDb;

        let ItemSynDefn {
            body,
            syn_expr_region,
        } = item_syn_defn(db, item_path)?;
        let region_path = RegionPath::ItemDefn(item_path);
        let sem_expr_region = db.sem_expr_region(syn_expr_region);
        let source_map_data = hir_lazy_expr_source_map(item_path, db)?.data(db);
        let regional_token_idx_base = region_path.regional_token_idx_base(db)?;
        let sem_expr_region_data = sem_expr_region.data(db);
        let module_path = region_path.module_path(db);
        let text = module_path.text(db);
        let ranged_token_sheet = db.ranged_token_sheet(module_path);
        let sem_expr_range_region_data = sem_expr_range_region(db, sem_expr_region).data(db);
        Some(Self {
            db,
            ranged_token_sheet,
            source_map_data,
            sem_expr_region_data,
            sem_expr_range_region_data,
            text,
            regional_token_idx_base,
        })
    }
}

impl<'db> HirLazyExprDebugger<'db> {
    pub fn expr_text(&self, expr: HirLazyExprIdx) -> &'db str {
        let expr = self.source_map_data.hir_lazy_to_sem_expr_idx(expr).unwrap(); // should None be handled?
        let token_idx_range =
            self.sem_expr_range_region_data[expr].token_idx_range(self.regional_token_idx_base);
        let text_range = self.ranged_token_sheet.tokens_text_range(token_idx_range);
        self.text.text_within(text_range)
    }

    pub fn stmt_text(&self, stmt: HirLazyStmtIdx) -> &'db str {
        let stmt = self.source_map_data.hir_lazy_to_sem_stmt_idx(stmt).unwrap(); // should None be handled?
        let token_idx_range =
            self.sem_expr_range_region_data[stmt].token_idx_range(self.regional_token_idx_base);
        let text_range = self.ranged_token_sheet.tokens_text_range(token_idx_range);
        self.text.text_within(text_range)
    }
}
