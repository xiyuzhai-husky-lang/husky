DiagnosticSheet {
    [salsa id]: 40,
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
                range: [8:31, 8:36),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [11:16, 11:17),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [12:30, 12:31),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [14:26, 14:27),
            },
            Diagnostic {
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [21:17, 21:18),
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