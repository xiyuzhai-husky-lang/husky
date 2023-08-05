DiagnosticSheet {
    [salsa id]: 26,
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
                message: "Type Error: type path mismatch in coersing `never` into `r32` of contract `move `, expected `core::raw_bits::r32`, but got `core::basic::never` instead",
                severity: Error,
                range: [119:5, 124:13),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `never` into `List ConnectedComponent` of contract `move `, expected `core::list::List`, but got `core::basic::never` instead",
                severity: Error,
                range: [127:5, 156:18),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `never` into `EffHoles` of contract `move `, expected `mnist_classifier::connected_component::EffHoles`, but got `core::basic::never` instead",
                severity: Error,
                range: [34:9, 40:33),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `never` into `f32` of contract `move `, expected `core::num::f32`, but got `core::basic::never` instead",
                severity: Error,
                range: [44:9, 50:36),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `never` into `f32` of contract `move `, expected `core::num::f32`, but got `core::basic::never` instead",
                severity: Error,
                range: [54:9, 57:30),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `never` into `f32` of contract `move `, expected `core::num::f32`, but got `core::basic::never` instead",
                severity: Error,
                range: [61:9, 64:35),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `never` into `ConnectedComponentDistribution` of contract `move `, expected `mnist_classifier::connected_component::ConnectedComponentDistribution`, but got `core::basic::never` instead",
                severity: Error,
                range: [67:9, 88:10),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `never` into `f32` of contract `move `, expected `core::num::f32`, but got `core::basic::never` instead",
                severity: Error,
                range: [97:9, 105:41),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `never` into `f32` of contract `move `, expected `core::num::f32`, but got `core::basic::never` instead",
                severity: Error,
                range: [108:9, 116:41),
            },
        ],
    },
}