DiagnosticSheet {
    [salsa id]: 26,
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
                range: [30:36, 30:37),
            },
            Diagnostic {
                message: "Syntax Error: no right operand for binary operator",
                severity: Error,
                range: [33:29, 33:30),
            },
            Diagnostic {
                message: "Syntax Error: no right operand for binary operator",
                severity: Error,
                range: [42:28, 42:29),
            },
            Diagnostic {
                message: "Syntax Error: no right operand for binary operator",
                severity: Error,
                range: [52:27, 52:28),
            },
            Diagnostic {
                message: "Syntax Error: no right operand for binary operator",
                severity: Error,
                range: [59:27, 59:28),
            },
            Diagnostic {
                message: "Syntax Error: no right operand for binary operator",
                severity: Error,
                range: [66:54, 66:55),
            },
            Diagnostic {
                message: "Syntax Error: no right operand for binary operator",
                severity: Error,
                range: [90:25, 90:26),
            },
            Diagnostic {
                message: "Syntax Error: no right operand for binary operator",
                severity: Error,
                range: [93:25, 93:26),
            },
        ],
    },
    decl_diagnostic_sheet: DeclDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: ExpectEqTokenForVariable",
                severity: Error,
                range: [30:37, 30:38),
            },
            Diagnostic {
                message: "Syntax Error: ExpectEqTokenForVariable",
                severity: Error,
                range: [33:30, 33:31),
            },
            Diagnostic {
                message: "Syntax Error: ExpectEqTokenForVariable",
                severity: Error,
                range: [42:29, 42:30),
            },
            Diagnostic {
                message: "Syntax Error: ExpectEqTokenForVariable",
                severity: Error,
                range: [52:28, 52:29),
            },
            Diagnostic {
                message: "Syntax Error: ExpectEqTokenForVariable",
                severity: Error,
                range: [59:28, 59:29),
            },
            Diagnostic {
                message: "Syntax Error: ExpectEqTokenForVariable",
                severity: Error,
                range: [66:55, 66:56),
            },
            Diagnostic {
                message: "Syntax Error: ExpectEqTokenForVariable",
                severity: Error,
                range: [90:26, 90:27),
            },
            Diagnostic {
                message: "Syntax Error: ExpectEqTokenForVariable",
                severity: Error,
                range: [93:26, 93:27),
            },
        ],
    },
    defn_diagnostic_sheet: DefnDiagnosticSheet {
        diagnostics: [],
    },
    expr_ty_diagnostic_sheet: ExprTypeDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Type Error: no method named `clone` for type `FluffyTerm::Solid(SolidTerm(0))`",
                severity: Error,
                range: [128:30, 128:35),
            },
            Diagnostic {
                message: "type path mismatch: expect core::list::List, but got core::option::Option instead",
                severity: Error,
                range: [40:25, 40:38),
            },
        ],
    },
}