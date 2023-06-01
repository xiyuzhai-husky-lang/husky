DiagnosticSheet {
    [salsa id]: 31,
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
                message: "Syntax Error: unterminated list",
                severity: Error,
                range: [16:20, 16:21),
            },
            Diagnostic {
                message: "Syntax Error: unterminated list",
                severity: Error,
                range: [31:24, 31:25),
            },
            Diagnostic {
                message: "Syntax Error: unterminated list",
                severity: Error,
                range: [39:20, 39:21),
            },
            Diagnostic {
                message: "Syntax Error: unterminated list",
                severity: Error,
                range: [50:16, 50:17),
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
        diagnostics: [],
    },
}