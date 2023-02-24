use super::*;
use husky_decl::*;
use husky_print_utils::p;

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
        let token_sheet_data = ranged_token_sheet.token_sheet_data(db);
        for (path, decl) in decl_sheet.decls().iter().copied() {
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

    fn message(&self, db: &Self::Context<'_>) -> String {
        // chatgpt wrote this
        match self {
            OriginalDeclError::ExpectLCurlOrLParOrSemicolon(_) => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        todo!()
    }

    fn range(&self, ctx: &Self::Context<'_>) -> TextRange {
        todo!()
    }
}

impl Diagnose for OriginalDeclExprError {
    type Context<'a> = RegionDiagnosticsContext<'a>;

    fn message(&self, ctx: &Self::Context<'_>) -> String {
        // chatgpt wrote this
        match self {
            OriginalDeclExprError::Expr(e) => {
                // TODO: Handle the error by displaying the error message.
                e.message(ctx)
            }
            OriginalDeclExprError::ExpectOutputType(_) => {
                // TODO: Handle the error by displaying the error message.
                format!("Syntax Error: expect output type")
            }
            OriginalDeclExprError::ExpectCurry(_) => {
                // TODO: Handle the error by displaying the error message.
                format!("Syntax Error: expect `->`",)
            }
            OriginalDeclExprError::ExpectEolColon(e) => {
                // TODO: Handle the error by displaying the error message.
                format!("Syntax Error: expect end-of-line colon",)
            }
            OriginalDeclExprError::ExpectRightCurlyBrace(_) => todo!(),
            OriginalDeclExprError::ExpectRightAngleBracketForImplicitParameterDeclList {
                langle_token_idx,
                current_token_idx,
            } => todo!(),
            OriginalDeclExprError::ExpectParameterDeclList(_) => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        todo!()
    }

    fn range(&self, ctx: &Self::Context<'_>) -> TextRange {
        todo!()
    }
}

impl<'a, 'b> RegionDiagnosticsCollector<'a, 'b> {
    fn visit_decl(&mut self, decl: Decl) {
        match decl {
            Decl::Type(decl) => match decl {
                TypeDecl::Enum(decl) => (),
                TypeDecl::RegularStruct(decl) => (),
                TypeDecl::UnitStruct(decl) => (),
                TypeDecl::TupleStruct(decl) => (),
                TypeDecl::Record(decl) => (),
                TypeDecl::Inductive(decl) => (),
                TypeDecl::Structure(decl) => (),
                TypeDecl::Alien(decl) => (),
                TypeDecl::Union(decl) => (),
            },
            Decl::Form(decl) => (),
            Decl::Trait(decl) => (),
            Decl::ImplBlock(decl) => (),
            Decl::AssociatedItem(decl) => (),
            Decl::Variant(decl) => (),
        }
    }
}
