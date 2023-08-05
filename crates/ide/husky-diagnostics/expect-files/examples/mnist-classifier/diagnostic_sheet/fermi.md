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
                message: "Term Error: expected coersion from `Type` to `Type` under contract `const`",
                severity: Error,
                range: [30:19, 30:49),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `List Option Leash ConcaveComponent` to `List Option Leash ConcaveComponent` under contract `move `",
                severity: Error,
                range: [33:45, 33:47),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [37:9, 37:64),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@MutableStackOwned { location: StackLocationIdx(LocalSymbolIdx(3)) } List Option Leash ConcaveComponent` to `List Option Leash ConcaveComponent` under contract `move `",
                severity: Error,
                range: [38:29, 38:36),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `FermiMatchResult` to `FermiMatchResult` under contract `move `",
                severity: Error,
                range: [38:12, 38:45),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [13:13, 13:49),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [19:13, 19:53),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [25:13, 25:63),
            },
        ],
    },
}