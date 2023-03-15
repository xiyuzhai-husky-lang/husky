DiagnosticSheet {
    [salsa id]: 23,
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "unresolved identifier",
                severity: Error,
                range: [1:5, 1:12),
            },
            Diagnostic {
                message: "unresolved identifier",
                severity: Error,
                range: [4:5, 4:12),
            },
            Diagnostic {
                message: "unresolved identifier",
                severity: Error,
                range: [5:12, 5:15),
            },
            Diagnostic {
                message: "unresolved identifier",
                severity: Error,
                range: [6:12, 6:15),
            },
            Diagnostic {
                message: "unresolved identifier",
                severity: Error,
                range: [7:12, 7:16),
            },
            Diagnostic {
                message: "unresolved identifier",
                severity: Error,
                range: [8:12, 8:17),
            },
            Diagnostic {
                message: "SymbolNotAccessible",
                severity: Error,
                range: [9:33, 9:50),
            },
        ],
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
                range: [13:25, 13:41),
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