use crate::*;
use husky_ast::{Ast, AstIdx, AstIdxRange, AstSheet};
use husky_entity_path::EntityPath;
use husky_entity_taxonomy::{EntityKind, FormKind, ModuleItemKind, TypeKind};
use husky_entity_tree::{CratePrelude, EntitySymbol, EntityTreeSheet, ModulePrelude};
use husky_opn_syntax::BinaryOpr;
use husky_print_utils::p;
use husky_token::{
    IdentifierToken, LeftAngleBracketToken, LeftBoxBracketToken, LeftCurlyBraceToken, Punctuation,
    RangedTokenSheet, RightCurlyBraceToken, TokenGroupIdx, TokenIdx, TokenSheetData,
};
use parsec::{parse_separated_list, ParseContext, ParseFrom};
use salsa::DebugWithDb;
use vec_like::VecPairMap;

pub(crate) struct DeclCollector<'a> {
    db: &'a dyn DeclDb,
    module_prelude: ModulePrelude<'a>,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    entity_tree_sheet: &'a EntityTreeSheet,
}

impl<'a> DeclCollector<'a> {
    pub(crate) fn new(db: &'a dyn DeclDb, module_path: ModulePath) -> EntityTreeResult<Self> {
        let module_prelude = db.module_prelude(module_path)?;
        Ok(Self {
            db,
            module_prelude,
            token_sheet_data: db.token_sheet_data(module_path)?,
            ast_sheet: db.ast_sheet(module_path)?,
            entity_tree_sheet: db.entity_tree_sheet(module_path)?,
        })
    }

    pub(crate) fn collect_all(mut self) -> DeclSheet {
        let mut decls: VecPairMap<EntityPath, DeclResult<Decl>> = Default::default();
        for entity_symbol in self.entity_tree_sheet.module_symbols().iter() {
            match entity_symbol {
                EntitySymbol::CrateRoot { .. } => unreachable!(),
                EntitySymbol::Submodule { .. } | EntitySymbol::EntityUse { .. } => (),
                EntitySymbol::ModuleItem {
                    ident,
                    accessibility,
                    path,
                    ast_idx,
                } => decls.insert(((*path).into(), self.parse_decl(*ast_idx, (*path).into()))),
            }
        }
        for associated_item in self.entity_tree_sheet.associated_items().iter() {
            todo!()
        }
        DeclSheet::new(decls)
    }

    fn parse_decl(&mut self, ast_idx: AstIdx, entity_path: EntityPath) -> DeclResult<Decl> {
        match self.ast_sheet[ast_idx] {
            Ast::Defn {
                token_group_idx,
                ref body,
                accessibility,
                entity_kind,
                entity_path: _,
                is_generic,
                body_kind,
                saved_stream_state,
                ..
            } => match entity_path {
                EntityPath::Module(_) => todo!(),
                EntityPath::ModuleItem(path) => match path {
                    ModuleItemPath::Type(path) => self.parse_ty_decl(
                        ast_idx,
                        path.type_kind(self.db),
                        path,
                        entity_kind,
                        token_group_idx,
                        body,
                        saved_stream_state,
                    ),
                    ModuleItemPath::Trait(path) => self.parse_trai_decl(
                        ast_idx,
                        path,
                        token_group_idx,
                        body,
                        saved_stream_state,
                    ),
                    ModuleItemPath::Form(path) => self.parse_form_decl(
                        ast_idx,
                        path,
                        entity_kind,
                        token_group_idx,
                        body,
                        saved_stream_state,
                    ),
                },
                EntityPath::GenericParameter(_) => todo!(),
                EntityPath::AssociatedItem(_) => todo!(),
                EntityPath::Variant(_) => todo!(),
            },
            Ast::Impl { .. }
            | Ast::Err { .. }
            | Ast::Use { .. }
            | Ast::Decor { .. }
            | Ast::BasicStmtOrBranch { .. }
            | Ast::IfElseStmts { .. }
            | Ast::MatchStmts { .. }
            | Ast::ModuleItemVariant { .. }
            | Ast::Main { .. }
            | Ast::Config { .. } => unreachable!(),
        }
    }

    fn parse_ty_decl(
        &mut self,
        ast_idx: AstIdx,
        type_kind: TypeKind,
        path: TypePath,
        entity_kind: EntityKind,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> Result<Decl, DeclError> {
        match type_kind {
            TypeKind::Enum => {
                self.parse_enum_type_decl(ast_idx, path, token_group_idx, body, saved_stream_state)
            }
            TypeKind::Inductive => self.parse_inductive_type_decl(
                ast_idx,
                path,
                token_group_idx,
                body,
                saved_stream_state,
            ),
            TypeKind::Record => todo!(),
            TypeKind::Struct => self.parse_struct_type_decl(
                ast_idx,
                path,
                token_group_idx,
                body,
                saved_stream_state,
            ),
            TypeKind::Structure => self.parse_structure_type_decl(
                ast_idx,
                path,
                token_group_idx,
                body,
                saved_stream_state,
            ),
            TypeKind::Foreign => self.parse_foreign_type_decl(
                ast_idx,
                path,
                token_group_idx,
                body,
                saved_stream_state,
            ),
        }
    }

    fn parse_enum_type_decl(
        &self,
        ast_idx: AstIdx,
        path: TypePath,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<Decl> {
        let mut parser = self.module_item_decl_expr_parser(path.into());
        let mut ctx = parser.ctx(
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let implicit_parameters = ctx.parse()?;
        Ok(Decl::Type(
            EnumTypeDecl::new(self.db, path, ast_idx, parser.finish(), implicit_parameters).into(),
        ))
    }

    fn parse_trai_decl(
        &self,
        ast_idx: AstIdx,
        path: TraitPath,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<Decl> {
        let mut parser = self.module_item_decl_expr_parser(path.into());
        let mut ctx = parser.ctx(
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let implicit_parameters = ctx.parse()?;
        Ok(Decl::Trait(TraitDecl::new(
            self.db,
            path,
            ast_idx,
            parser.finish(),
            implicit_parameters,
        )))
    }

    fn parse_inductive_type_decl(
        &self,
        ast_idx: AstIdx,
        path: TypePath,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<Decl> {
        let mut parser = self.module_item_decl_expr_parser(path.into());
        let mut ctx = parser.ctx(
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let implicit_parameters = ctx.parse()?;
        Ok(Decl::Type(
            InductiveTypeDecl::new(self.db, path, ast_idx, parser.finish(), implicit_parameters)
                .into(),
        ))
    }

    fn parse_struct_type_decl(
        &self,
        ast_idx: AstIdx,
        path: TypePath,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<Decl> {
        let mut parser = self.module_item_decl_expr_parser(path.into());
        let mut ctx = parser.ctx(
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let implicit_parameters = ctx.parse()?;
        if let Some(lcurl) = ctx.parse::<LeftCurlyBraceToken>()? {
            let (fields, separators) = parse_separated_list(&mut ctx)?;
            let rcurl: RightCurlyBraceToken = ctx.parse_expected()?;
            Ok(Decl::Type(
                PropsStructTypeDecl::new(
                    self.db,
                    path,
                    ast_idx,
                    parser.finish(),
                    implicit_parameters,
                    lcurl,
                    fields,
                    separators,
                    rcurl,
                )
                .into(),
            ))
        } else if let Some(lbox) = ctx.parse::<LeftBoxBracketToken>()? {
            todo!()
        } else {
            Err(DeclError::ExpectLCurlOrLParOrSemicolon(ctx.save_state()))
        }
    }

    fn module_item_decl_expr_parser(&self, entity_path: EntityPath) -> ExprParser<'a> {
        ExprParser::new(
            self.db,
            Some(entity_path),
            self.token_sheet_data,
            self.module_prelude,
        )
    }

    fn parse_structure_type_decl(
        &self,
        ast_idx: AstIdx,
        path: TypePath,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<Decl> {
        let mut token_iter = self
            .token_sheet_data
            .token_group_token_stream(token_group_idx, Some(saved_stream_state));

        let mut parser = self.module_item_decl_expr_parser(path.into());
        let mut ctx = parser.ctx(
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let implicit_parameters = ctx.parse()?;
        Ok(Decl::Type(
            StructureTypeDecl::new(self.db, path, ast_idx, parser.finish(), implicit_parameters)
                .into(),
        ))
    }

    // get declaration from tokens
    fn parse_foreign_type_decl(
        &self,
        ast_idx: AstIdx,
        path: TypePath,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<Decl> {
        let mut token_iter = self
            .token_sheet_data
            .token_group_token_stream(token_group_idx, Some(saved_stream_state));

        let mut parser = self.module_item_decl_expr_parser(path.into());
        let mut ctx = parser.ctx(
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let implicit_parameters = ctx.parse()?;
        Ok(Decl::Type(
            AlienTypeDecl::new(self.db, path, ast_idx, parser.finish(), implicit_parameters).into(),
        ))
    }

    fn parse_form_decl(
        &mut self,
        ast_idx: AstIdx,
        path: FormPath,
        entity_kind: EntityKind,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> Result<Decl, DeclError> {
        match path.form_kind(self.db) {
            FormKind::Feature => {
                self.parse_feature_decl(ast_idx, token_group_idx, saved_stream_state, path)
            }
            FormKind::Function => {
                self.parse_function_decl(ast_idx, token_group_idx, saved_stream_state, path)
            }
            FormKind::Value => todo!(),
            FormKind::TypeAlias => todo!(),
        }
    }

    fn parse_feature_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenIdx,
        path: FormPath,
    ) -> Result<Decl, DeclError> {
        let mut parser = self.module_item_decl_expr_parser(path.into());
        let mut ctx = parser.ctx(
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        Ok(Decl::Form(
            FeatureDecl::new(self.db, path, ast_idx, parser.finish()).into(),
        ))
    }

    fn parse_function_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenIdx,
        path: FormPath,
    ) -> Result<Decl, DeclError> {
        let mut parser = self.module_item_decl_expr_parser(path.into());
        let mut ctx = parser.ctx(
            self.token_sheet_data
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
        );
        let implicit_parameter_decl_list = ctx.parse()?;
        let Some(parameter_decl_list) = ctx.parse()? else {
            todo!()
        };
        Ok(Decl::Form(
            FunctionDecl::new(
                self.db,
                path,
                ast_idx,
                parser.finish(),
                implicit_parameter_decl_list,
                parameter_decl_list,
            )
            .into(),
        ))
    }
}
