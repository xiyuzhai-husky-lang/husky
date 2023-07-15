DiagnosticSheet {
    [salsa id]: 45,
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
                message: "type path mismatch: expect mnist::BinaryImage28, but got core::mem::Leash instead",
                severity: Error,
                range: [4:31, 4:36),
            },
            Diagnostic {
                message: "type path mismatch: expect core::mem::Leash, but got core::list::List instead",
                severity: Error,
                range: [23:5, 23:43),
            },
            Diagnostic {
                message: "type path mismatch: expect core::mem::Leash, but got core::list::List instead",
                severity: Error,
                range: [23:5, 23:43),
            },
            Diagnostic {
                message: "type path mismatch: expect core::mem::Leash, but got mnist_classifier::raw_contour::RawContour instead",
                severity: Error,
                range: [27:5, 27:46),
            },
            Diagnostic {
                message: "type path mismatch: expect core::mem::Leash, but got mnist_classifier::raw_contour::RawContour instead",
                severity: Error,
                range: [27:5, 27:46),
            },
            Diagnostic {
                message: "type path mismatch: expect core::mem::Leash, but got core::list::List instead",
                severity: Error,
                range: [33:5, 33:49),
            },
            Diagnostic {
                message: "type path mismatch: expect core::mem::Leash, but got core::list::List instead",
                severity: Error,
                range: [33:5, 33:49),
            },
        ],
    },
}