DiagnosticSheet {
    [salsa id]: 30,
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
                range: [3:34, 3:35),
            },
            Diagnostic {
                message: "Syntax Error: no right operand for binary operator",
                severity: Error,
                range: [6:41, 6:42),
            },
            Diagnostic {
                message: "Syntax Error: no right operand for binary operator",
                severity: Error,
                range: [9:37, 9:38),
            },
        ],
    },
    decl_diagnostic_sheet: DeclDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: ExpectEqTokenForVariable",
                severity: Error,
                range: [3:35, 3:36),
            },
            Diagnostic {
                message: "Syntax Error: ExpectEqTokenForVariable",
                severity: Error,
                range: [6:42, 6:43),
            },
            Diagnostic {
                message: "Syntax Error: ExpectEqTokenForVariable",
                severity: Error,
                range: [9:38, 9:39),
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