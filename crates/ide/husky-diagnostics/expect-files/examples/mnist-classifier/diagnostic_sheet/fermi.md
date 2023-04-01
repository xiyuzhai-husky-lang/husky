DiagnosticSheet {
    [salsa id]: 34,
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
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: expect end-of-line colon",
                severity: Error,
                range: [10:13, 10:14),
            },
            Diagnostic {
                message: "Syntax Error: expect end-of-line colon",
                severity: Error,
                range: [16:17, 16:18),
            },
            Diagnostic {
                message: "Syntax Error: expect end-of-line colon",
                severity: Error,
                range: [22:26, 22:27),
            },
        ],
    },
    defn_diagnostic_sheet: DefnDiagnosticSheet {
        diagnostics: [],
    },
    expr_ty_diagnostic_sheet: ExprTypeDiagnosticSheet {
        diagnostics: [],
    },
}