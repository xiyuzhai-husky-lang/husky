use crate::*;
use husky_ast::*;

use husky_entity_taxonomy::*;
use husky_entity_tree::*;

use husky_print_utils::p;
use husky_token::*;

use husky_coword::Ident;
use parsec::*;

pub(crate) struct DeclParser<'a> {
    db: &'a dyn DeclDb,
    module_symbol_context: ModuleSymbolContext<'a>,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
}

impl<'a> DeclParser<'a> {
    pub(crate) fn new(db: &'a dyn DeclDb, path: ModulePath) -> Self {
        let Ok(module_symbol_context) = db.module_symbol_context(path) else {
            use salsa::DebugWithDb;
            p!(path.debug(db));
            unreachable!("valid module")
        };
        Self {
            db,
            module_symbol_context,
            token_sheet_data: db.token_sheet_data(path).expect("valid module"),
            ast_sheet: db.ast_sheet(path).expect("valid module"),
        }
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
