DiagnosticSheet {
    [salsa id]: 31,
    item_tree_diagnostic_sheet: EntityTreeDiagnosticSheet {
        diagnostics: [],
    },
    token_diagnostic_sheet: TokenDiagnosticSheet {
        diagnostics: [],
    },
    ast_diagnostic_sheet: AstDiagnosticSheet {
        diagnostics: [],
    },
    expr_diagnostic_sheet: ExprDiagnosticSheet {
        diagnostics: [],
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
                message: "Type Error: cannot index into type `CyclicSliceLeashed LineSegmentStroke`",
                severity: Error,
                range: [52:33, 52:77),
            },
            Diagnostic {
                message: "Type Error: cannot index into type `CyclicSliceLeashed LineSegmentStroke`",
                severity: Error,
                range: [75:28, 75:72),
            },
            Diagnostic {
                message: "Type Error: cannot index into type `CyclicSliceLeashed LineSegmentStroke`",
                severity: Error,
                range: [77:29, 77:77),
            },
        ],
    },
}