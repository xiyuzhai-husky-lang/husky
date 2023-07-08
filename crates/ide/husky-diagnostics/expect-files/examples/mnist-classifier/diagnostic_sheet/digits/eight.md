DiagnosticSheet {
    [salsa id]: 27,
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
                range: [3:41, 3:42),
            },
            Diagnostic {
                message: "Syntax Error: no right operand for binary operator",
                severity: Error,
                range: [6:38, 6:39),
            },
        ],
    },
    decl_diagnostic_sheet: DeclDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: ExpectEqTokenForVariable",
                severity: Error,
                range: [3:42, 3:43),
            },
            Diagnostic {
                message: "Syntax Error: ExpectEqTokenForVariable",
                severity: Error,
                range: [6:39, 6:40),
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