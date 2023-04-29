DiagnosticSheet {
    [salsa id]: 41,
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
                message: "Syntax Error: no left operand for binary operator",
                severity: Error,
                range: [15:14, 15:15),
            },
            Diagnostic {
                message: "Syntax Error: no right operand for binary operator",
                severity: Error,
                range: [15:14, 15:15),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [16:44, 16:49),
            },
            Diagnostic {
                message: "Syntax Error: ExpectedExprBeforeDot",
                severity: Error,
                range: [16:43, 16:44),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [17:41, 17:46),
            },
            Diagnostic {
                message: "Syntax Error: ExpectedExprBeforeDot",
                severity: Error,
                range: [17:40, 17:41),
            },
            Diagnostic {
                message: "Syntax Error: HtmlTodo",
                severity: Error,
                range: [22:27, 22:28),
            },
            Diagnostic {
                message: "Syntax Error: UnexpectedLeftCurlyBrace",
                severity: Error,
                range: [22:27, 22:28),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [22:28, 22:33),
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
                message: "OriginalFluffyTermExpectationError::Todo",
                severity: Error,
                range: [157:29, 157:31),
            },
        ],
    },
}