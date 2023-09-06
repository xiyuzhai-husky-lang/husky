use crate::*;
use either::Either;
use husky_ast::AstSheet;
use husky_entity_syn_tree::{
    EntitySynTreeCrateBundle, EntitySynTreeResult, EntitySynTreeSheet, ModuleSymbolContext,
};
use husky_syn_decl::HasSynDecl;
use husky_vfs::ModulePath;

pub(crate) struct DecrParserFactory<'a> {
    db: &'a dyn DecrDb,
    module_symbol_context: ModuleSymbolContext<'a>,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
}

impl<'a> DecrParserFactory<'a> {
    pub(crate) fn new(db: &'a dyn DecrDb, path: ModulePath) -> EntitySynTreeResult<Self> {
        let module_symbol_context = db.module_symbol_context(path)?;
        Ok(Self {
            db,
            module_symbol_context,
            token_sheet_data: db.token_sheet_data(path)?,
            ast_sheet: db.ast_sheet(path)?,
        })
    }

    pub(crate) fn expr_parser(
        &self,
        decr_id: DecrPath,
        allow_self_type: AllowSelfType,
        allow_self_value: AllowSelfValue,
    ) -> ExprParser<'a> {
        let parent_expr_region = match decr_id.parent(self.db) {
            DecrParent::Defn(path) => path
                .syn_decl(self.db)
                .ok()
                .map(|decl| decl.syn_expr_region(self.db))
                .flatten(),
        };
        ExprParser::new(
            self.db,
            decr_id.into(),
            self.token_sheet_data,
            self.module_symbol_context,
            parent_expr_region,
            allow_self_type,
            allow_self_value,
        )
    }
}
