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
                message: "Term Error: expected coersion from `Leash LineSegmentSketch at StackPure { location: StackLocationIdx(LocalSymbolIdx(0)) }` to `LineSegmentSketch` under contract ``",
                severity: Error,
                range: [94:37, 94:56),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `Leash LineSegmentSketch at StackPure { location: StackLocationIdx(LocalSymbolIdx(0)) }` to `LineSegmentSketch` under contract ``",
                severity: Error,
                range: [98:45, 98:64),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `never` into `List ConcaveComponent` of contract `move `, expected `core::list::List`, but got `core::basic::never` instead",
                severity: Error,
                range: [90:5, 107:30),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `never` into `f32` of contract `move `, expected `core::num::f32`, but got `core::basic::never` instead",
                severity: Error,
                range: [25:9, 35:30),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `never` into `f32` of contract `move `, expected `core::num::f32`, but got `core::basic::never` instead",
                severity: Error,
                range: [38:9, 45:28),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `never` into `BoundingBox` of contract `move `, expected `mnist_classifier::geom2d::BoundingBox`, but got `core::basic::never` instead",
                severity: Error,
                range: [48:9, 62:10),
            },
        ],
    },
}