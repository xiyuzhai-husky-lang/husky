DiagnosticSheet {
    [salsa id]: 40,
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
                message: "Type Error: no method named `cyclic_slice` for type `List LineSegmentStroke`",
                severity: Error,
                range: [103:45, 103:57),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@StackPure { location: StackLocationIdx(LocalSymbolIdx(0)) } Leash LineSegmentSketch` to `LineSegmentSketch` under contract ``",
                severity: Error,
                range: [94:37, 94:56),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@StackPure { location: StackLocationIdx(LocalSymbolIdx(0)) } Leash LineSegmentSketch` to `LineSegmentSketch` under contract ``",
                severity: Error,
                range: [98:45, 98:64),
            },
        ],
    },
}