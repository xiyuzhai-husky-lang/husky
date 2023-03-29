use crate::*;
use either::Either;
use husky_ast::{AstSheet, DecrId, DecrParent};
use husky_decl::HasDecl;
use husky_entity_tree::{
    EntityTreeCrateBundle, EntityTreeResult, EntityTreeSheet, ModuleSymbolContext,
};
use husky_vfs::ModulePath;

pub(crate) struct DecrParserFactory<'a> {
    db: &'a dyn DecrDb,
    module_symbol_context: ModuleSymbolContext<'a>,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    module_entity_tree: &'a EntityTreeSheet,
    entity_tree_crate_bundle: &'a EntityTreeCrateBundle,
}

impl<'a> DecrParserFactory<'a> {
    pub(crate) fn new(db: &'a dyn DecrDb, path: ModulePath) -> EntityTreeResult<Self> {
        let module_symbol_context = db.module_symbol_context(path)?;
        Ok(Self {
            db,
            module_symbol_context,
            token_sheet_data: db.token_sheet_data(path)?,
            ast_sheet: db.ast_sheet(path)?,
            module_entity_tree: db.entity_tree_sheet(path)?,
            entity_tree_crate_bundle: db.entity_tree_bundle(path.crate_path(db))?,
        })
    }

    pub(crate) fn expr_parser(
        &self,
        decr_id: DecrId,
        allow_self_type: AllowSelfType,
        allow_self_value: AllowSelfValue,
    ) -> ExprParser<'a> {
        let parent_expr_region = match decr_id.parent() {
            DecrParent::Defn(path) => match path {
                Either::Left(path) => path
                    .decl(self.db)
                    .ok()
                    .map(|decl| decl.expr_region(self.db)),
                Either::Right(_) => None,
            },
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
