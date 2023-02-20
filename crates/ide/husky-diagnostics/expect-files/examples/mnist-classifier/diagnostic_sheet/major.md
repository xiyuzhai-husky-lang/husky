DiagnosticSheet {
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
        ],
    },
    expr_ty_diagnostic_sheet: ExprTypeDiagnosticSheet {
        diagnostics: [
            Diagnostic {
                message: "Type Error: original `todo` in term List ConnectedComponent",
                severity: Error,
                range: [13:13, 13:33),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term never",
                severity: Error,
                range: [11:5, 18:36),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term List ConnectedComponent",
                severity: Error,
                range: [22:13, 22:33),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term ConnectedComponent",
                severity: Error,
                range: [24:18, 24:43),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term never",
                severity: Error,
                range: [21:5, 24:56),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term ConnectedComponent",
                severity: Error,
                range: [27:5, 27:30),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term RawContour",
                severity: Error,
                range: [34:5, 34:22),
            },
            Diagnostic {
                message: "Type Error: original `todo` in term LineSegmentSketch",
                severity: Error,
                range: [37:5, 37:30),
            },
        ],
    },
    entity_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
}