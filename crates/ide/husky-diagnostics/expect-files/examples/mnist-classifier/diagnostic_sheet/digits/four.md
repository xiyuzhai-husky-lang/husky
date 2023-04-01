DiagnosticSheet {
    [salsa id]: 25,
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
                range: [6:29, 6:45),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [100:32, 100:48),
            },
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [105:24, 105:40),
            },
        ],
    },
    decl_diagnostic_sheet: DeclDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: ExpectParameterDeclList",
                severity: Error,
                range: [3:20, 3:22),
            },
            Diagnostic {
                message: "Syntax Error: ExpectParameterDeclList",
                severity: Error,
                range: [10:29, 10:31),
            },
            Diagnostic {
                message: "Syntax Error: ExpectParameterDeclList",
                severity: Error,
                range: [13:27, 13:29),
            },
            Diagnostic {
                message: "Syntax Error: ExpectParameterDeclList",
                severity: Error,
                range: [16:12, 16:14),
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