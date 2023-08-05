DiagnosticSheet {
    [salsa id]: 44,
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
                message: "Type Error: type path mismatch in coersing `never` into `i32` of contract `move `, expected `core::num::i32`, but got `core::basic::never` instead",
                severity: Error,
                range: [82:5, 112:15),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `never` into `i32` of contract `move `, expected `core::num::i32`, but got `core::basic::never` instead",
                severity: Error,
                range: [115:5, 154:22),
            },
            Diagnostic {
                message: "Type Error: no field named start in CyclicSliceLeashed Point2d",
                severity: Error,
                range: [171:68, 171:102),
            },
            Diagnostic {
                message: "Type Error: no field named end in CyclicSliceLeashed Point2d",
                severity: Error,
                range: [193:41, 193:74),
            },
            Diagnostic {
                message: "Type Error: no field named end in CyclicSliceLeashed Point2d",
                severity: Error,
                range: [200:13, 200:46),
            },
            Diagnostic {
                message: "Type Error: no method named `cyclic_slice` for type `List Point2d`",
                severity: Error,
                range: [27:37, 27:49),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `never` into `BoundingBox` of contract `move `, expected `mnist_classifier::geom2d::BoundingBox`, but got `core::basic::never` instead",
                severity: Error,
                range: [46:9, 60:10),
            },
        ],
    },
}