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
                Err(error) => sheet_collector.visit_atom(error),
            }
        }
    }
    // todo
    DeclDiagnosticSheet::new(db, sheet_collector.finish())
}

impl Diagnose for DeclError {
    type Context<'a> = SheetDiagnosticsContext<'a>;

    fn message(&self, db: &Self::Context<'_>) -> String {
        // chatgpt wrote this
        match self {
            OriginalDeclExprError::ExpectLCurlOrLParOrSemicolon(e) => {
                // TODO: Handle the error by displaying the error message.
                format!("SyntaxError: expected a left curly brace, left parenthesis or semicolon")
            }
            OriginalDeclExprError::Token(e) => {
                // TODO: Handle the error by displaying the error message.
                format!("SyntaxError: {}", e)
            }
            OriginalDeclExprError::EntityTree(e) => {
                // TODO: Handle the error by displaying the error message.
                format!("Error while parsing entity tree: {}", e)
            }
            OriginalDeclExprError::Vfs(e) => {
                // TODO: Handle the error by displaying the error message.
                format!("Error while accessing virtual file system: {}", e)
            }
            OriginalDeclExprError::Expr(e) => {
                // TODO: Handle the error by displaying the error message.
                format!("Error while parsing expression: {}", e)
            }
            OriginalDeclExprError::ImplBlockErr => {
                // TODO: Handle the error by displaying a generic error message.
                format!("Error while parsing impl block")
            }
            OriginalDeclExprError::ExpectOutputType(e) => {
                // TODO: Handle the error by displaying the error message.
                format!("Expect output type: {}", e)
            }
            OriginalDeclExprError::ExpectCurry(e) => {
                // TODO: Handle the error by displaying the error message.
                format!("Expect curry: {}", e)
            }
            OriginalDeclExprError::ExpectEolColon(e) => {
                // TODO: Handle the error by displaying the error message.
                format!("Expect end-of-line colon: {}", e)
            }
            OriginalDeclExprError::UnableToParseImplBlockDeclForTyAsTraitMethodDecl => {
                // TODO: Handle the error by displaying a generic error message.
                format!("Unable to parse impl block declaration as trait method declaration")
            }
            OriginalDeclExprError::UnableToParseImplBlockDeclForTyMethodDecl => {
                // TODO: Handle the error by displaying a generic error message.
                format!("Unable to parse impl block declaration as method declaration")
            }
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
