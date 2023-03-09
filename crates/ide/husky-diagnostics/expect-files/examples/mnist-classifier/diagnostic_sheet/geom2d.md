DiagnosticSheet {
    [salsa id]: 34,
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
    token_diagnostic_sheet: TokenDiagnosticSheet {
        diagnostics: [],
    },
    ast_diagnostic_sheet: AstDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: expected decorator or entity keyword",
                severity: Error,
                range: [101:5, 101:29),
            },
            Diagnostic {
                message: "Syntax Error: expected decorator or entity keyword",
                severity: Error,
                range: [102:5, 102:29),
            },
            Diagnostic {
                message: "Syntax Error: unexpected stmt inside module",
                severity: Error,
                range: [103:1, 103:2),
            },
        ],
    },
    expr_diagnostic_sheet: ExprDiagnosticSheet {
        diagnostics: [],
    },
    decl_diagnostic_sheet: DeclDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: expect `}`",
                severity: Error,
                range: [101:5, 101:8),
            },
        ],
    },
    defn_diagnostic_sheet: DefnDiagnosticSheet {
        diagnostics: [],
    },
    expr_ty_diagnostic_sheet: ExprTypeDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [8:18, 8:20),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [8:39, 8:41),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [50:24, 50:27),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [56:62, 56:67),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [56:70, 56:79),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [71:24, 71:27),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [78:25, 78:30),
            },
            Diagnostic {
                message: "unresolved term",
                severity: Error,
                range: [78:33, 78:42),
            },
        ],
    },
}