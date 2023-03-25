use super::*;
use husky_decl::*;
use husky_expr::ExprError;

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct DeclDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn decl_diagnostic_sheet(
    db: &dyn DiagnosticsDb,
    module_path: ModulePath,
) -> DeclDiagnosticSheet {
    let mut sheet_collector = SheetDiagnosticsCollector::new(db, module_path);
    if let (Ok(ranged_token_sheet), Ok(decl_sheet)) = (
        db.ranged_token_sheet(module_path),
        db.decl_sheet(module_path),
    ) {
        let _token_sheet_data = ranged_token_sheet.token_sheet_data(db);
        for (_path, decl) in decl_sheet.decls().iter().copied() {
            match decl {
                Ok(decl) => {
                    let mut collector = RegionDiagnosticsCollector::new(
                        db,
                        decl.expr_region(db),
                        &mut sheet_collector,
                    );
                    collector.visit_decl(decl)
                }
                Err(DeclError::Original(error)) => sheet_collector.visit_atom(error),
                Err(DeclError::Derived(_)) => (),
            }
        }
    }
    // todo
    DeclDiagnosticSheet::new(db, sheet_collector.finish())
}

impl Diagnose for OriginalDeclError {
    type Context<'a> = SheetDiagnosticsContext<'a>;

    fn message(&self, _db: &Self::Context<'_>) -> String {
        // chatgpt wrote this
        match self {
            OriginalDeclError::ExpectLCurlOrLParOrSemicolon(_) => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        todo!()
    }

    fn range(&self, _ctx: &Self::Context<'_>) -> TextRange {
        todo!()
    }
}

impl Diagnose for OriginalDeclExprError {
    type Context<'a> = RegionDiagnosticsContext<'a>;

    fn message(&self, ctx: &Self::Context<'_>) -> String {
        match self {
            OriginalDeclExprError::Expr(e) => e.message(ctx),
            OriginalDeclExprError::ExpectOutputType(_) => {
                format!("Syntax Error: expect output type")
            }
            OriginalDeclExprError::ExpectCurry(_) => {
                format!("Syntax Error: expect `->`",)
            }
            OriginalDeclExprError::ExpectEolColon(_e) => {
                format!("Syntax Error: expect end-of-line colon",)
            }
            OriginalDeclExprError::ExpectRightCurlyBrace(_) => {
                format!("Syntax Error: expect `}}`",)
            }
            OriginalDeclExprError::ExpectRightAngleBracketForImplicitParameterDeclList {
                ..
            } => {
                format!("Syntax Error: expect `>` for implicit parameter declaration list",)
            }
            OriginalDeclExprError::ExpectParameterDeclList(_) => {
                format!("Syntax Error: ExpectParameterDeclList",)
            }
            OriginalDeclExprError::ExpectImplicitParameterDecl(_) => {
                format!("Syntax Error: expect implicit parameter declaration",)
            }
            OriginalDeclExprError::ExpectRightParenthesisInParameterList(_) => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &Self::Context<'_>) -> TextRange {
        match self {
            OriginalDeclExprError::Expr(error) => error.range(ctx),
            OriginalDeclExprError::ExpectOutputType(token_idx)
            | OriginalDeclExprError::ExpectCurry(token_idx)
            | OriginalDeclExprError::ExpectEolColon(token_idx)
            | OriginalDeclExprError::ExpectRightCurlyBrace(token_idx)
            | OriginalDeclExprError::ExpectRightAngleBracketForImplicitParameterDeclList {
                current_token_idx: token_idx,
                ..
            }
            | OriginalDeclExprError::ExpectParameterDeclList(token_idx)
            | OriginalDeclExprError::ExpectImplicitParameterDecl(token_idx)
            | OriginalDeclExprError::ExpectRightParenthesisInParameterList(token_idx) => {
                ctx.ranged_token_sheet().token_text_range(*token_idx)
            }
        }
    }
}

impl<'a, 'b> RegionDiagnosticsCollector<'a, 'b> {
    fn visit_decl(&mut self, decl: Decl) {
        match decl {
            Decl::Type(decl) => match decl {
                TypeDecl::Enum(decl) => self.visit_enum_decl(decl),
                TypeDecl::RegularStruct(decl) => self.visit_regular_struct_decl(decl),
                TypeDecl::UnitStruct(decl) => self.visit_unit_struct_decl(decl),
                TypeDecl::TupleStruct(decl) => self.visit_tuple_struct_decl(decl),
                TypeDecl::Record(decl) => self.visit_record_decl(decl),
                TypeDecl::Inductive(decl) => self.visit_inductive_decl(decl),
                TypeDecl::Structure(decl) => self.visit_structure_decl(decl),
                TypeDecl::Extern(decl) => self.visit_alien_decl(decl),
                TypeDecl::Union(decl) => self.visit_union_decl(decl),
            },
            Decl::Form(decl) => self.visit_form_decl(decl),
            Decl::Trait(decl) => self.visit_trait_decl(decl),
            Decl::Impl(decl) => self.visit_impl_decl(decl),
            Decl::AssociatedItem(decl) => self.visit_associated_item_decl(decl),
            Decl::Variant(decl) => self.visit_variant_decl(decl),
        }
    }

    fn visit_regular_struct_decl(&mut self, decl: RegularStructTypeDecl) {
        if let Err(DeclExprError::Original(e)) = decl.implicit_parameters(self.db()) {
            self.visit_atom(e)
        } else if let Err(DeclExprError::Original(e)) = decl.fields(self.db()) {
            self.visit_atom(e)
        } else if let Err(DeclExprError::Original(e)) = decl.rcurl(self.db()) {
            self.visit_atom(e)
        }
    }

    fn visit_unit_struct_decl(&mut self, decl: UnitStructTypeDecl) {
        if let Err(DeclExprError::Original(e)) = decl.implicit_parameters(self.db()) {
            self.visit_atom(e)
        }
        // todo!()
    }

    fn visit_tuple_struct_decl(&mut self, decl: TupleStructTypeDecl) {
        if let Err(DeclExprError::Original(e)) = decl.implicit_parameters(self.db()) {
            self.visit_atom(e)
        }
        // todo!()
    }

    fn visit_record_decl(&mut self, decl: RecordTypeDecl) {
        if let Err(DeclExprError::Original(e)) = decl.implicit_parameters(self.db()) {
            self.visit_atom(e)
        }
        // todo!()
    }

    fn visit_inductive_decl(&mut self, decl: InductiveTypeDecl) {
        if let Err(DeclExprError::Original(e)) = decl.implicit_parameters(self.db()) {
            self.visit_atom(e)
        }
        // todo!()
    }

    fn visit_structure_decl(&mut self, decl: StructureTypeDecl) {
        if let Err(DeclExprError::Original(e)) = decl.implicit_parameters(self.db()) {
            self.visit_atom(e)
        }
        // todo!()
    }

    fn visit_alien_decl(&mut self, decl: ExternTypeDecl) {
        if let Err(DeclExprError::Original(e)) = decl.implicit_parameters(self.db()) {
            self.visit_atom(e)
        }
    }

    fn visit_union_decl(&mut self, decl: UnionTypeDecl) {
        if let Err(DeclExprError::Original(e)) = decl.implicit_parameters(self.db()) {
            self.visit_atom(e)
        }
    }

    fn visit_enum_decl(&mut self, decl: EnumTypeDecl) {
        if let Err(DeclExprError::Original(e)) = decl.implicit_parameters(self.db()) {
            self.visit_atom(e)
        }
    }

    fn visit_form_decl(&mut self, decl: FormDecl) {
        match decl {
            FormDecl::Fn(decl) => self.visit_function_decl(decl),
            FormDecl::Feature(decl) => self.visit_feature_decl(decl),
            FormDecl::Gn(decl) => self.visit_morphism_decl(decl),
            FormDecl::Value(decl) => self.visit_value_decl(decl),
        }
    }

    fn visit_function_decl(&mut self, decl: FnDecl) {
        if let Err(DeclExprError::Original(e)) = decl.implicit_parameters(self.db()) {
            self.visit_atom(e)
        } else if let Err(DeclExprError::Original(e)) = decl.parameters(self.db()) {
            self.visit_atom(e)
        } else if let Err(DeclExprError::Original(e)) = decl.curry_token(self.db()) {
            self.visit_atom(e)
        } else if let Err(DeclExprError::Original(e)) = decl.return_ty(self.db()) {
            self.visit_atom(e)
        } else if let Err(DeclExprError::Original(e)) = decl.eol_colon(self.db()) {
            self.visit_atom(e)
        }
    }

    fn visit_feature_decl(&mut self, decl: FeatureDecl) {
        if let Err(DeclExprError::Original(e)) = decl.curry_token(self.db()) {
            self.visit_atom(e)
        } else if let Err(DeclExprError::Original(e)) = decl.return_ty(self.db()) {
            self.visit_atom(e)
        } else if let Err(DeclExprError::Original(e)) = decl.eol_colon(self.db()) {
            self.visit_atom(e)
        }
    }

    fn visit_morphism_decl(&mut self, decl: GnDecl) {
        if let Err(DeclExprError::Original(e)) = decl.implicit_parameters(self.db()) {
            self.visit_atom(e)
        }
        // todo!()
    }

    fn visit_value_decl(&mut self, decl: ValueDecl) {
        if let Err(DeclExprError::Original(e)) = decl.implicit_parameters(self.db()) {
            self.visit_atom(e)
        }
        // todo!()
    }

    fn visit_trait_decl(&mut self, decl: TraitDecl) {
        if let Err(DeclExprError::Original(e)) = decl.implicit_parameters(self.db()) {
            self.visit_atom(e)
        }
        // todo!()
    }

    fn visit_impl_decl(&mut self, decl: ImplDecl) {
        match decl {
            ImplDecl::Type(decl) => self.visit_ty_impl_block_decl(decl),
            ImplDecl::TypeAsTrait(decl) => self.visit_trai_for_ty_impl_block_decl(decl),
        }
    }

    fn visit_ty_impl_block_decl(&mut self, decl: TypeImplBlockDecl) {
        if let Err(DeclExprError::Original(e)) = decl.implicit_parameters(self.db()) {
            self.visit_atom(e)
        } else if let Err(DeclExprError::Original(e)) = decl.eol_colon(self.db()) {
            self.visit_atom(e)
        }
    }

    fn visit_trai_for_ty_impl_block_decl(&mut self, _decl: TraitForTypeImplBlockDecl) {
        // todo!()
    }

    fn visit_associated_item_decl(&mut self, _decl: AssociatedItemDecl) {
        // todo!()
    }

    fn visit_variant_decl(&mut self, _decl: VariantDecl) {
        // todo!()
    }
}
