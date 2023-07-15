DiagnosticSheet {
    [salsa id]: 44,
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
    token_diagnostic_sheet: TokenDiagnosticSheet {
        diagnostics: [],
    },
    ast_diagnostic_sheet: AstDiagnosticSheet {
        diagnostics: [],
    },
    expr_diagnostic_sheet: ExprDiagnosticSheet {
        diagnostics: [],
    },
    decl_diagnostic_sheet: DeclDiagnosticSheet {
        diagnostics: [],
    },
    defn_diagnostic_sheet: DefnDiagnosticSheet {
        diagnostics: [],
    },
    expr_ty_diagnostic_sheet: ExprTypeDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Type Error: no method named `popx` for type `FluffyTerm::Solid(SolidTerm(2))`",
                severity: Error,
                range: [185:49, 185:53),
            },
            Diagnostic {
                message: "Type Error: no method named `popx` for type `FluffyTerm::Solid(SolidTerm(2))`",
                severity: Error,
                range: [196:47, 196:51),
            },
        ],
    },
}