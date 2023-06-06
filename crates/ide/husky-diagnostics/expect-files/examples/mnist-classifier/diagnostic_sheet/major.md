DiagnosticSheet {
    [salsa id]: 44,
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
                range: [4:31, 4:36),
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
                message: "expected subtype",
                severity: Error,
                range: [3:32, 3:34),
            },
            Diagnostic {
                message: "expected function type",
                severity: Error,
                range: [3:31, 3:34),
            },
            Diagnostic {
                message: "expected subtype",
                severity: Error,
                range: [22:30, 22:32),
            },
            Diagnostic {
                message: "expected function type",
                severity: Error,
                range: [22:29, 22:32),
            },
            Diagnostic {
                message: "expected subtype",
                severity: Error,
                range: [32:36, 32:38),
            },
            Diagnostic {
                message: "expected function type",
                severity: Error,
                range: [32:35, 32:38),
            },
        ],
    },
}