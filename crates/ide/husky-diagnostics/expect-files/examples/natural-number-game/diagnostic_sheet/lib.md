DiagnosticSheet {
    [salsa id]: 46,
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
    token_diagnostic_sheet: TokenDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: unrecognized char",
                severity: Error,
                range: [10:22, 10:23),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized char",
                severity: Error,
                range: [10:46, 10:47),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized char",
                severity: Error,
                range: [15:17, 15:18),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized char",
                severity: Error,
                range: [20:17, 20:18),
            },
        ],
    },
    ast_diagnostic_sheet: AstDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: unexpected stmt inside impl",
                severity: Error,
                range: [6:5, 6:29),
            },
            Diagnostic {
                message: "Syntax Error: unexpected stmt inside impl",
                severity: Error,
                range: [7:5, 7:24),
            },
            Diagnostic {
                message: "Syntax Error: unexpected stmt inside impl",
                severity: Error,
                range: [8:5, 8:45),
            },
            Diagnostic {
                message: "Syntax Error: expected identifier",
                severity: Error,
                range: [10:8, 10:9),
            },
            Diagnostic {
                message: "Syntax Error: excessive indent",
                severity: Error,
                range: [11:5, 11:7),
            },
        ],
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
        diagnostics: [],
    },
}