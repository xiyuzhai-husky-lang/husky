DiagnosticSheet {
    [salsa id]: 35,
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
                range: [3:33, 3:34),
            },
            Diagnostic {
                message: "Syntax Error: no right operand for binary operator",
                severity: Error,
                range: [23:36, 23:37),
            },
        ],
    },
    decl_diagnostic_sheet: DeclDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: ExpectEqTokenForVariable",
                severity: Error,
                range: [3:34, 3:35),
            },
            Diagnostic {
                message: "Syntax Error: ExpectEqTokenForVariable",
                severity: Error,
                range: [23:37, 23:38),
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