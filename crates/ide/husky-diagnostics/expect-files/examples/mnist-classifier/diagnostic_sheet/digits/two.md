DiagnosticSheet {
    [salsa id]: 33,
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
                message: "Type Error: no field named eff_holes in TypeOntology(core::mem::Leash) TypeOntology(mnist_classifier::connected_component::ConnectedComponent)",
                severity: Error,
                range: [32:21, 32:56),
            },
            Diagnostic {
                message: "Type Error: no field named lower_mass in TypeOntology(core::mem::Leash) TypeOntology(mnist_classifier::connected_component::ConnectedComponent)",
                severity: Error,
                range: [41:24, 41:60),
            },
            Diagnostic {
                message: "Type Error: no field named upper_mass in TypeOntology(core::mem::Leash) TypeOntology(mnist_classifier::connected_component::ConnectedComponent)",
                severity: Error,
                range: [41:63, 41:99),
            },
        ],
    },
}