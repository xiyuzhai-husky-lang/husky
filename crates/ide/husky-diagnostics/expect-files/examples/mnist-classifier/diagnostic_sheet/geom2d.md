DiagnosticSheet {
    [salsa id]: 35,
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
                message: "Type Error: NoSuchMethod",
                severity: Error,
                range: [68:26, 68:38),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [108:40, 108:52),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [109:40, 109:52),
            },
        ],
    },
}