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
    module_entity_tree: &'a EntityTreeSheet,
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
            module_entity_tree: db.entity_tree_sheet(path)?,
            entity_tree_crate_bundle: db.entity_tree_bundle(path.crate_path(db))?,
        })
    }

    pub(crate) fn resolve_module_item_ast_idx(&self, path: impl Into<EntityPath>) -> AstIdx {
        self.resolve_module_item_symbol(path).ast_idx(self.db)
    }

    #[inline(always)]
    fn resolve_module_item_symbol(&self, path: impl Into<EntityPath>) -> ModuleItemSymbol {
        let path = path.into();
        let ident = path.ident(self.db);
        let Some(entity_symbol) = self
            .module_entity_tree
            .module_symbols()
            .resolve_ident(ident)
            else {
                use salsa::DisplayWithDb;
                panic!(r#"
    Path `{}` is invalid!
    This is very likely caused by expect item in standard library.
"#, path.display(self.db()))
            };
        entity_symbol.module_item_symbol().unwrap()
    }

    pub(super) fn parse_trai_decl_aux(
        &self,
        ast_idx: AstIdx,
        path: TraitPath,
        token_group_idx: TokenGroupIdx,
        _body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TraitDecl> {
        let mut parser = self.expr_parser(
            DeclRegionPath::Entity(path.into()),
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx2(None, token_group_idx, Some(saved_stream_state));
        let implicit_parameters = ctx.parse();
        Ok(TraitDecl::new(
            self.db,
            path,
            ast_idx,
            parser.finish(),
            implicit_parameters,
        ))
    }

    pub(crate) fn expr_parser(
        &self,
        expr_path: DeclRegionPath,
        parent_expr_region: Option<ExprRegion>,
        allow_self_type: AllowSelfType,
        allow_self_value: AllowSelfValue,
    ) -> ExprParser<'a> {
        ExprParser::new(
            self.db,
            expr_path.into(),
            self.token_sheet_data,
            self.module_symbol_context,
            parent_expr_region,
            allow_self_type,
            allow_self_value,
        )
    }

    pub(super) fn parse_feature_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenIdx,
        path: FormPath,
    ) -> Result<FormDecl, DeclError> {
        let mut parser = self.expr_parser(
            DeclRegionPath::Entity(path.into()),
            None,
            AllowSelfType::False,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx2(None, token_group_idx, Some(saved_stream_state));
        let curry_token = ctx.parse_expected(OriginalDeclExprError::ExpectCurry);
        let return_ty = ctx.parse_expected(OriginalDeclExprError::ExpectOutputType);
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectEolColon);
        Ok(FeatureDecl::new(
            self.db,
            path,
            ast_idx,
            curry_token,
            return_ty,
            eol_colon,
            parser.finish(),
        )
        .into())
    }

    pub(super) fn parse_fn_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenIdx,
        path: FormPath,
    ) -> Result<FormDecl, DeclError> {
        let mut parser = self.expr_parser(
            DeclRegionPath::Entity(path.into()),
            None,
            AllowSelfType::False,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx2(None, token_group_idx, Some(saved_stream_state));
        let implicit_parameter_decl_list = ctx.parse();
        let parameter_decl_list =
            ctx.parse_expected(OriginalDeclExprError::ExpectParameterDeclList);
        let curry_token = ctx.parse_expected(OriginalDeclExprError::ExpectCurry);
        let return_ty = ctx.parse_expected(OriginalDeclExprError::ExpectOutputType);
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectEolColon);
        Ok(FnDecl::new(
            self.db,
            path,
            ast_idx,
            parser.finish(),
            implicit_parameter_decl_list,
            parameter_decl_list,
            curry_token,
            return_ty,
            eol_colon,
        )
        .into())
    }

    pub(crate) fn db(&self) -> &'a dyn DeclDb {
        self.db
    }

    pub(crate) fn token_sheet_data(&self) -> &'a TokenSheetData {
        self.token_sheet_data
    }

    pub(crate) fn ast_sheet(&self) -> &'a AstSheet {
        self.ast_sheet
    }
}
