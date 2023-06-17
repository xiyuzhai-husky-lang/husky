use crate::*;
use husky_ast::*;

use husky_entity_taxonomy::*;
use husky_entity_tree::*;

use husky_print_utils::p;
use husky_token::*;

use husky_word::Ident;
use parsec::*;

pub(crate) struct DeclParseContext<'a> {
    db: &'a dyn DeclDb,
    module_symbol_context: ModuleSymbolContext<'a>,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    entity_tree_sheet: &'a EntityTreeSheet,
    entity_tree_crate_bundle: &'a EntityTreeCrateBundle,
}

impl<'a> DeclParseContext<'a> {
    pub(crate) fn new(db: &'a dyn DeclDb, path: ModulePath) -> EntityTreeResult<Self> {
        let module_symbol_context = db.module_symbol_context(path)?;
        Ok(Self {
            db,
            module_symbol_context,
            token_sheet_data: db.token_sheet_data(path)?,
            ast_sheet: db.ast_sheet(path)?,
            entity_tree_sheet: db.entity_tree_sheet(path)?,
            entity_tree_crate_bundle: db.entity_tree_bundle(path.crate_path(db))?,
        })
    }

    #[inline(always)]
    pub(crate) fn resolve_module_item_ast_idx(&self, id: impl Into<ModuleItemNodePath>) -> AstIdx {
        self.resolve_module_item_symbol(id.into()).ast_idx(self.db)
    }

    #[inline(always)]
    pub(crate) fn resolve_module_item_indexed_ast(
        &self,
        path: ModuleItemPath,
    ) -> (AstIdx, &'a Ast) {
        let path = path.into();
        self.ast_sheet
            .all_ast_indexed_iter()
            .find(|(_, ast)| match ast {
                Ast::Defn { block, .. } => block.entity_path() == Some(path),
                _ => false,
            })
            .expect("should be guaranteed to exists by the construction of path")
    }

    #[inline(always)]
    pub(crate) fn resolve_ty_variant_indexed_ast(
        &self,
        node_path: TypeVariantNodePath,
    ) -> (AstIdx, &'a Ast) {
        todo!()
        // self.ast_sheet
        //     .all_ast_indexed_iter()
        //     .find(|(_, ast)| match ast {
        //         Ast::TypeVariant { path, .. } => node_path == *path,
        //         _ => false,
        //     })
        //     .expect("should be guaranteed to exists by the construction of path")
    }

    #[inline(always)]
    fn resolve_module_item_symbol(&self, id: ModuleItemNodePath) -> ModuleItemNode {
        todo!()
        //         let db = self.db;
        //         let path = id.path(db);
        //         let ident = path.ident(db);
        //         let Some(entity_symbol) = self
        //             .entity_tree_sheet
        //             .module_symbols()
        //             .resolve_ident(ident)
        //             else {
        //                 use salsa::DisplayWithDb;
        //                 panic!(r#"
        //     Path `{}` is invalid!
        //     This is very likely caused by expect item in standard library.
        // "#, path.display(db))
        //             };
        //         entity_symbol.module_item_node().unwrap()
    }

    #[inline(always)]
    pub(crate) fn expr_parser(
        &self,
        node_path: impl Into<EntityNodePath>,
        parent_expr_region: Option<ExprRegion>,
        allow_self_type: AllowSelfType,
        allow_self_value: AllowSelfValue,
    ) -> ExprParser<'a> {
        ExprParser::new(
            self.db,
            RegionPath::Decl(node_path.into()),
            self.token_sheet_data,
            self.module_symbol_context,
            parent_expr_region,
            allow_self_type,
            allow_self_value,
        )
    }

    pub(crate) fn db(&self) -> &'a dyn DeclDb {
        self.db
    }

    pub(crate) fn ast_sheet(&self) -> &'a AstSheet {
        self.ast_sheet
    }
}
