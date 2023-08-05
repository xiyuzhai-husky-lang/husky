DiagnosticSheet {
    [salsa id]: 46,
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
        diagnostics: [
            Diagnostic {
                message: "Syntax Error: unrecognized identifier",
                severity: Error,
                range: [172:35, 172:47),
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
                message: "Type Error: cannot index into type `BinaryImage28`",
                severity: Error,
                range: [174:20, 174:32),
            },
            Diagnostic {
                message: "Type Error: cannot index into type `BinaryImage28`",
                severity: Error,
                range: [175:20, 175:30),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `HollowTermTodo` to `HollowTermTodo` under contract ``",
                severity: Error,
                range: [174:30, 174:31),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `Option Point2d` into `unit` of contract ``, expected `core::basic::unit`, but got `core::option::Option` instead",
                severity: Error,
                range: [264:17, 264:30),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `never` into `List RawContour` of contract `move `, expected `core::list::List`, but got `core::basic::never` instead",
                severity: Error,
                range: [171:5, 266:18),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `never` into `BoundingBox` of contract `move `, expected `mnist_classifier::geom2d::BoundingBox`, but got `core::basic::never` instead",
                severity: Error,
                range: [20:9, 34:10),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `never` into `f32` of contract `move `, expected `core::num::f32`, but got `core::basic::never` instead",
                severity: Error,
                range: [40:9, 48:27),
            },
        ],
    },
}