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
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: no right operand for binary operator",
                severity: Error,
                range: [3:39, 3:40),
            },
            Diagnostic {
                message: "Syntax Error: no right operand for binary operator",
                severity: Error,
                range: [10:48, 10:49),
            },
            Diagnostic {
                message: "Syntax Error: no right operand for binary operator",
                severity: Error,
                range: [13:46, 13:47),
            },
            Diagnostic {
                message: "Syntax Error: no right operand for binary operator",
                severity: Error,
                range: [16:26, 16:27),
            },
        ],
    },
    decl_diagnostic_sheet: DeclDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: ExpectEqTokenForVariable",
                severity: Error,
                range: [3:40, 3:41),
            },
            Diagnostic {
                message: "Syntax Error: ExpectEqTokenForVariable",
                severity: Error,
                range: [10:49, 10:50),
            },
            Diagnostic {
                message: "Syntax Error: ExpectEqTokenForVariable",
                severity: Error,
                range: [13:47, 13:48),
            },
            Diagnostic {
                message: "Syntax Error: ExpectEqTokenForVariable",
                severity: Error,
                range: [16:27, 16:28),
            },
        ],
    },
    defn_diagnostic_sheet: DefnDiagnosticSheet {
        diagnostics: [],
    },
    expr_ty_diagnostic_sheet: ExprTypeDiagnosticSheet {
        diagnostics: [],
    },
}