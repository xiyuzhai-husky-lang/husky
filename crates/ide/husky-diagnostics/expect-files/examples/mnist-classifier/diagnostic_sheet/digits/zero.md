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
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [12:8, 12:46),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [29:13, 29:48),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [31:13, 31:48),
            },
            Diagnostic {
                message: "Type Error: NoSuchField",
                severity: Error,
                range: [32:22, 32:57),
            },
        ],
    },
}