DiagnosticSheet {
    [salsa id]: 24,
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
                message: "Type Error: no method named `len` for type `@MutableStackOwned { location: StackLocationIdx(LocalSymbolIdx(1)) } Slice t`",
                severity: Error,
                range: [2:19, 2:22),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@MutableStackOwned { location: StackLocationIdx(LocalSymbolIdx(1)) } Slice t` to `Slice _t` under contract `move `",
                severity: Error,
                range: [3:20, 3:23),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `isize` to `isize` under contract ``",
                severity: Error,
                range: [3:25, 3:26),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `isize` to `isize` under contract ``",
                severity: Error,
                range: [3:28, 3:46),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@StackPure { location: StackLocationIdx(LocalSymbolIdx(3)) } isize` to `isize` under contract ``",
                severity: Error,
                range: [6:14, 6:18),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [6:8, 6:18),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@MutableStackOwned { location: StackLocationIdx(LocalSymbolIdx(1)) } Slice t` to `Slice _t` under contract `move `",
                severity: Error,
                range: [7:27, 7:30),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@StackPure { location: StackLocationIdx(LocalSymbolIdx(2)) } isize` to `isize` under contract ``",
                severity: Error,
                range: [7:32, 7:35),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@StackPure { location: StackLocationIdx(LocalSymbolIdx(3)) } isize` to `isize` under contract ``",
                severity: Error,
                range: [7:37, 7:41),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@MutableStackOwned { location: StackLocationIdx(LocalSymbolIdx(1)) } Slice t` to `Slice _t` under contract `move `",
                severity: Error,
                range: [8:24, 8:27),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@StackPure { location: StackLocationIdx(LocalSymbolIdx(2)) } isize` to `isize` under contract ``",
                severity: Error,
                range: [8:29, 8:32),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [8:9, 8:40),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@MutableStackOwned { location: StackLocationIdx(LocalSymbolIdx(1)) } Slice t` to `Slice _t` under contract `move `",
                severity: Error,
                range: [9:24, 9:27),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@StackPure { location: StackLocationIdx(LocalSymbolIdx(3)) } isize` to `isize` under contract ``",
                severity: Error,
                range: [9:36, 9:40),
            },
            Diagnostic {
                message: "Type Error: no method named `swap` for type `@MutableStackOwned { location: StackLocationIdx(LocalSymbolIdx(1)) } Slice t`",
                severity: Error,
                range: [26:17, 26:21),
            },
            Diagnostic {
                message: "Type Error: no method named `swap` for type `@MutableStackOwned { location: StackLocationIdx(LocalSymbolIdx(1)) } Slice t`",
                severity: Error,
                range: [27:9, 27:13),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `isize` to `isize` under contract ``",
                severity: Error,
                range: [13:33, 13:34),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [16:11, 16:15),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [17:9, 17:25),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [18:15, 18:53),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [19:13, 19:29),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [20:9, 20:24),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [21:15, 21:30),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [21:34, 21:71),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [21:15, 21:71),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [22:13, 22:28),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [23:12, 23:37),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `_i` to `_a` under contract `move `",
                severity: Error,
                range: [32:18, 32:19),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `_i` to `_a` under contract `move `",
                severity: Error,
                range: [32:21, 32:23),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `_i` to `_a` under contract `move `",
                severity: Error,
                range: [32:25, 32:26),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `_i` to `_a` under contract `move `",
                severity: Error,
                range: [32:28, 32:31),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `_i` to `_a` under contract `move `",
                severity: Error,
                range: [32:33, 32:34),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `_i` to `_a` under contract `move `",
                severity: Error,
                range: [32:36, 32:38),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `_i` to `_a` under contract `move `",
                severity: Error,
                range: [32:40, 32:41),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `_i` to `_a` under contract `move `",
                severity: Error,
                range: [32:43, 32:45),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `_i` to `_a` under contract `move `",
                severity: Error,
                range: [32:47, 32:50),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `_i` to `_a` under contract `move `",
                severity: Error,
                range: [32:52, 32:53),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [33:5, 33:18),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `_i` to `_a` under contract `move `",
                severity: Error,
                range: [34:18, 34:21),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `_i` to `_a` under contract `move `",
                severity: Error,
                range: [34:23, 34:24),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `_i` to `_a` under contract `move `",
                severity: Error,
                range: [34:26, 34:27),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `_i` to `_a` under contract `move `",
                severity: Error,
                range: [34:29, 34:30),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `_i` to `_a` under contract `move `",
                severity: Error,
                range: [34:32, 34:33),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `_i` to `_a` under contract `move `",
                severity: Error,
                range: [34:35, 34:36),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `_i` to `_a` under contract `move `",
                severity: Error,
                range: [34:38, 34:40),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `_i` to `_a` under contract `move `",
                severity: Error,
                range: [34:42, 34:44),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `_i` to `_a` under contract `move `",
                severity: Error,
                range: [34:46, 34:48),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `_i` to `_a` under contract `move `",
                severity: Error,
                range: [34:50, 34:53),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [34:12, 34:54),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `Ref 'static str` to `_a` under contract `move `",
                severity: Error,
                range: [38:21, 38:28),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `Ref 'static str` to `_a` under contract `move `",
                severity: Error,
                range: [38:30, 38:37),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `Ref 'static str` to `_a` under contract `move `",
                severity: Error,
                range: [38:39, 38:49),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `Ref 'static str` to `_a` under contract `move `",
                severity: Error,
                range: [38:51, 38:56),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `Ref 'static str` to `_a` under contract `move `",
                severity: Error,
                range: [38:58, 38:65),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `Ref 'static str` to `_a` under contract `move `",
                severity: Error,
                range: [38:67, 38:72),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [39:5, 39:21),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `Ref 'static str` to `_a` under contract `move `",
                severity: Error,
                range: [40:21, 40:31),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `Ref 'static str` to `_a` under contract `move `",
                severity: Error,
                range: [40:33, 40:38),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `Ref 'static str` to `_a` under contract `move `",
                severity: Error,
                range: [40:40, 40:47),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `Ref 'static str` to `_a` under contract `move `",
                severity: Error,
                range: [40:49, 40:54),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `Ref 'static str` to `_a` under contract `move `",
                severity: Error,
                range: [40:56, 40:63),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `Ref 'static str` to `_a` under contract `move `",
                severity: Error,
                range: [40:65, 40:72),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [40:12, 40:73),
            },
        ],
    },
}