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
    let mut collector = ModuleDiagnosticsCollector::new(db, module_path);
    if let (Ok(ranged_token_sheet), Ok(node_decl_sheet)) = (
        db.ranged_token_sheet(module_path),
        db.node_decl_sheet(module_path),
    ) {
        for (_, node_decl) in node_decl_sheet.decls(db).iter().copied() {
            collector.visit_node_decl(node_decl)
        }
    }
    DeclDiagnosticSheet::new(db, collector.finish())
}

impl<'a> ModuleDiagnosticsCollector<'a> {
    fn visit_node_decl(&mut self, node_decl: NodeDecl) {
        match node_decl {
            NodeDecl::Submodule(node_decl) => (),
            NodeDecl::ModuleItem(node_decl) => self.visit_module_item_decl(node_decl),
            NodeDecl::ImplBlock(node_decl) => (),
            NodeDecl::AssociatedItem(node_decl) => (),
            NodeDecl::TypeVariant(node_decl) => (),
        }
    }

    fn visit_module_item_decl(&mut self, node_decl: ModuleItemNodeDecl) {
        match node_decl {
            ModuleItemNodeDecl::Type(node_decl) => self.visit_ty_node_decl(node_decl),
            ModuleItemNodeDecl::Fugitive(node_decl) => todo!(),
            ModuleItemNodeDecl::Trait(node_decl) => todo!(),
        }
    }

    fn visit_ty_node_decl(&mut self, node_decl: TypeNodeDecl) {
        match node_decl {
            TypeNodeDecl::Enum(node_decl) => {
                todo!()
            }
            TypeNodeDecl::PropsStruct(_) => todo!(),
            TypeNodeDecl::UnitStruct(_) => todo!(),
            TypeNodeDecl::TupleStruct(_) => todo!(),
            TypeNodeDecl::Record(_) => todo!(),
            TypeNodeDecl::Inductive(_) => todo!(),
            TypeNodeDecl::Structure(_) => todo!(),
            TypeNodeDecl::Extern(_) => todo!(),
            TypeNodeDecl::Union(_) => todo!(),
        }
    }
}

// impl Diagnose for OriginalDeclError {
//     type Context<'a> = SheetDiagnosticsContext<'a>;

//     fn message(&self, ctx: &Self::Context<'_>) -> String {
//         match self {
//             OriginalDeclError::ExpectedLCurlOrLParOrSemicolon(_) => {
//                 format!("expected `{{` or `(` or `;`")
//             }
//             OriginalDeclError::NoSuchItem => format!("no such item"),
//             OriginalDeclError::Expr(e) => e.message(ctx),
//             OriginalDeclError::Deprecated => "deprecated".to_string(),
//         }
//     }

//     fn severity(&self) -> DiagnosticSeverity {
//         DiagnosticSeverity::Error
//     }

//     fn range(&self, ctx: &Self::Context<'_>) -> TextRange {
//         match self {
//             OriginalDeclError::ExpectedLCurlOrLParOrSemicolon(token_stream_state) => {
//                 ctx.token_stream_state_text_range(*token_stream_state)
//             }
//             OriginalDeclError::NoSuchItem => todo!(),
//             OriginalDeclError::Expr(e) => e.range(ctx),
//             OriginalDeclError::Deprecated => todo!(),
//         }
//     }
// }

impl Diagnose for OriginalDeclExprError {
    type Context<'a> = SheetDiagnosticsContext<'a>;

    fn message(&self, ctx: &Self::Context<'_>) -> String {
        match self {
            OriginalDeclExprError::Expr(e) => e.message(ctx),
            OriginalDeclExprError::ExpectedOutputType(_) => {
                format!("Syntax Error: expect output type")
            }
            OriginalDeclExprError::ExpectedCurry(_) => {
                format!("Syntax Error: expect `->`",)
            }
            OriginalDeclExprError::ExpectedEolColon(_e) => {
                format!("Syntax Error: expect end-of-line colon",)
            }
            OriginalDeclExprError::ExpectedRightCurlyBrace(_) => {
                format!("Syntax Error: expect `}}`",)
            }
            OriginalDeclExprError::ExpectedRightAngleBracketForImplicitParameterDeclList {
                ..
            } => {
                format!("Syntax Error: expect `>` for implicit parameter declaration list",)
            }
            OriginalDeclExprError::ExpectedParameterDeclList(_) => {
                format!("Syntax Error: ExpectParameterDeclList",)
            }
            OriginalDeclExprError::ExpectedImplicitParameterDecl(_) => {
                format!("Syntax Error: expect implicit parameter declaration",)
            }
            OriginalDeclExprError::ExpectedRightParenthesisInParameterList(_) => {
                format!("Syntax Error: expected `)` in parameter list",)
            }
            OriginalDeclExprError::ExpectedRightParenthesisInTupleStructFieldTypeList(_) => {
                format!("Syntax Error: expected `)` in tuple struct field type list",)
            }
            OriginalDeclExprError::ExpectedVariableType(_) => {
                format!("Syntax Error: ExpectVariableType",)
            }
            OriginalDeclExprError::ExpectEqTokenForVariable(_) => {
                format!("Syntax Error: ExpectEqTokenForVariable",)
            }
            OriginalDeclExprError::ExpectedLeftCurlyBraceOrLeftParenthesisOrSemicolonForStruct(
                _,
            ) => {
                format!("Syntax Error: expected `{{` `(` or `;` for struct",)
            }
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &Self::Context<'_>) -> TextRange {
        match self {
            OriginalDeclExprError::Expr(error) => error.range(ctx),
            OriginalDeclExprError::ExpectedOutputType(token_stream_state)
            | OriginalDeclExprError::ExpectedCurry(token_stream_state)
            | OriginalDeclExprError::ExpectedEolColon(token_stream_state)
            | OriginalDeclExprError::ExpectedRightCurlyBrace(token_stream_state)
            | OriginalDeclExprError::ExpectedRightAngleBracketForImplicitParameterDeclList {
                token_stream_state,
                ..
            }
            | OriginalDeclExprError::ExpectedParameterDeclList(token_stream_state)
            | OriginalDeclExprError::ExpectedImplicitParameterDecl(token_stream_state)
            | OriginalDeclExprError::ExpectedRightParenthesisInParameterList(token_stream_state)
            | OriginalDeclExprError::ExpectedRightParenthesisInTupleStructFieldTypeList(
                token_stream_state,
            )
            | OriginalDeclExprError::ExpectedVariableType(token_stream_state)
            | OriginalDeclExprError::ExpectEqTokenForVariable(token_stream_state)
            | OriginalDeclExprError::ExpectedLeftCurlyBraceOrLeftParenthesisOrSemicolonForStruct(
                token_stream_state,
            ) => ctx.token_stream_state_text_range(*token_stream_state),
        }
    }
}
