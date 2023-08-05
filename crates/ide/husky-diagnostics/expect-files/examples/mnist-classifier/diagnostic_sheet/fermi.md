DiagnosticSheet {
    [salsa id]: 38,
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
                message: "Type Error: type path mismatch in coersing `never` into `FermiMatchResult` of contract `move `, expected `mnist_classifier::fermi::FermiMatchResult`, but got `core::basic::never` instead",
                severity: Error,
                range: [32:5, 38:45),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `never` into `f32` of contract `move `, expected `core::num::f32`, but got `core::basic::never` instead",
                severity: Error,
                range: [11:9, 14:20),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `never` into `f32` of contract `move `, expected `core::num::f32`, but got `core::basic::never` instead",
                severity: Error,
                range: [17:9, 20:20),
            },
            Diagnostic {
                message: "Type Error: type path mismatch in coersing `never` into `f32` of contract `move `, expected `core::num::f32`, but got `core::basic::never` instead",
                severity: Error,
                range: [23:9, 26:20),
            },
        ],
    },
}