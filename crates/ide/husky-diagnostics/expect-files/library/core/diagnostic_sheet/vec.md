DiagnosticSheet {
    [salsa id]: 18,
    item_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
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
                range: [12:35, 12:36),
            },
            Diagnostic {
                message: "Syntax Error: expect end-of-line colon",
                severity: Error,
                range: [14:34, 14:35),
            },
        ],
    },
    defn_diagnostic_sheet: DefnDiagnosticSheet {
        diagnostics: [],
    },
    expr_ty_diagnostic_sheet: ExprTypeDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Term Error: expected category",
                severity: Error,
                range: [12:28, 12:34),
            },
            Diagnostic {
                message: "Term Error: expected category",
                severity: Error,
                range: [14:27, 14:33),
            },
        ],
    },
}