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
    pub(crate) fn new(db: &'a dyn DeclDb, path: ModulePath) -> Self {
        let module_symbol_context = db.module_symbol_context(path).expect("valid module");
        Self {
            db,
            module_symbol_context,
            token_sheet_data: db.token_sheet_data(path).expect("valid module"),
            ast_sheet: db.ast_sheet(path).expect("valid module"),
            entity_tree_sheet: db.entity_tree_sheet(path).expect("valid module"),
            entity_tree_crate_bundle: db
                .entity_tree_bundle(path.crate_path(db))
                .expect("valid module"),
        }
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

    #[inline(always)]
    pub(crate) fn db(&self) -> &'a dyn DeclDb {
        self.db
    }

    #[inline(always)]
    pub(crate) fn ast_sheet(&self) -> &'a AstSheet {
        self.ast_sheet
    }
}
