DiagnosticSheet {
    [salsa id]: 25,
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
                message: "Type Error: no method named `clone` for type `FluffyTerm::Solid(SolidTerm(0))`",
                severity: Error,
                range: [128:30, 128:35),
            },
            Diagnostic {
                message: "type path mismatch: expect core::option::Option, but got core::mem::Leash instead",
                severity: Error,
                range: [40:25, 40:32),
            },
        ],
    },
}