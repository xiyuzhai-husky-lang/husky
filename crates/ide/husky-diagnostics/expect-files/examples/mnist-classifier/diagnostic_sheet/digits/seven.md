DiagnosticSheet {
    [salsa id]: 28,
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
                message: "SymbolNotAccessible",
                severity: Error,
                range: [7:33, 7:50),
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
                range: [11:26, 11:42),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [19:27, 19:43),
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