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
                range: [10:14, 10:15),
            },
            Diagnostic {
                message: "Syntax Error: expect end-of-line colon",
                severity: Error,
                range: [16:18, 16:19),
            },
            Diagnostic {
                message: "Syntax Error: expect end-of-line colon",
                severity: Error,
                range: [22:27, 22:28),
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