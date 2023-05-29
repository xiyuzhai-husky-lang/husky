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
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [21:5, 21:16),
            },
            Diagnostic {
                message: "Syntax Error: unterminated list",
                severity: Error,
                range: [21:16, 21:17),
            },
        ],
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
                message: "Type Error: TodoIndexOrComposeWithList",
                severity: Error,
                range: [28:13, 28:59),
            },
            Diagnostic {
                message: "Type Error: TodoIndexOrComposeWithList",
                severity: Error,
                range: [30:13, 30:59),
            },
            Diagnostic {
                message: "Type Error: TodoIndexOrComposeWithList",
                severity: Error,
                range: [31:22, 31:68),
            },
        ],
    },
}