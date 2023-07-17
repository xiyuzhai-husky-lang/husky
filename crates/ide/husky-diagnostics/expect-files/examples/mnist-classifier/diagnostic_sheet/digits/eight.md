DiagnosticSheet {
    [salsa id]: 27,
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
                message: "Type Error: no method named `first` for type `FluffyTerm::Application(EtherealTermApplication { function: EtherealTerm(`CyclicSliceLeashed`), argument: EtherealTerm(`LineSegmentStroke`), shift: 0 })`",
                severity: Error,
                range: [29:28, 29:33),
            },
            Diagnostic {
                message: "Type Error: no method named `first` for type `FluffyTerm::Application(EtherealTermApplication { function: EtherealTerm(`CyclicSliceLeashed`), argument: EtherealTerm(`LineSegmentStroke`), shift: 0 })`",
                severity: Error,
                range: [29:58, 29:63),
            },
        ],
    },
}