DiagnosticSheet {
    [salsa id]: 29,
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
                message: "Term Error: expected coersion",
                severity: Error,
                range: [4:44, 4:63),
            },
            Diagnostic {
                message: "Term Error: expected coersion",
                severity: Error,
                range: [4:65, 4:84),
            },
            Diagnostic {
                message: "Term Error: expected coersion",
                severity: Error,
                range: [11:44, 11:66),
            },
            Diagnostic {
                message: "Term Error: expected coersion",
                severity: Error,
                range: [14:44, 14:58),
            },
        ],
    },
}