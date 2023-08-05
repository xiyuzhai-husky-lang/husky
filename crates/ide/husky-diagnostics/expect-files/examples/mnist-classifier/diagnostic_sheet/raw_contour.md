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
                message: "Term Error: expected coersion from `i32` to `i32` under contract ``",
                severity: Error,
                range: [63:18, 63:19),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `r32` to `r32` under contract ``",
                severity: Error,
                range: [63:24, 63:28),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `r32` to `r32` under contract `move `",
                severity: Error,
                range: [63:5, 63:28),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `r32` to `r32` under contract `move `",
                severity: Error,
                range: [63:5, 63:28),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `r32` to `r32` under contract ``",
                severity: Error,
                range: [66:18, 66:22),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `r32` to `r32` under contract `move `",
                severity: Error,
                range: [66:5, 66:22),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `r32` to `r32` under contract `move `",
                severity: Error,
                range: [66:5, 66:22),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `i32` to `i32` under contract ``",
                severity: Error,
                range: [69:18, 69:19),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `r32` to `r32` under contract ``",
                severity: Error,
                range: [69:24, 69:28),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `r32` to `r32` under contract `move `",
                severity: Error,
                range: [69:5, 69:28),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `r32` to `r32` under contract `move `",
                severity: Error,
                range: [69:5, 69:28),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@StackPure { location: StackLocationIdx(LocalSymbolIdx(0)) } r32` to `r32` under contract ``",
                severity: Error,
                range: [76:43, 76:52),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@StackPure { location: StackLocationIdx(LocalSymbolIdx(2)) } i32` to `i32` under contract ``",
                severity: Error,
                range: [76:54, 76:55),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@StackPure { location: StackLocationIdx(LocalSymbolIdx(1)) } r32` to `r32` under contract ``",
                severity: Error,
                range: [77:43, 77:52),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@StackPure { location: StackLocationIdx(LocalSymbolIdx(2)) } i32` to `i32` under contract ``",
                severity: Error,
                range: [77:54, 77:55),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `i32` to `i32` under contract ``",
                severity: Error,
                range: [102:50, 102:63),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `i32` to `i32` under contract ``",
                severity: Error,
                range: [102:49, 102:64),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `i32` to `i32` under contract ``",
                severity: Error,
                range: [102:84, 102:85),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@StackPure { location: StackLocationIdx(LocalSymbolIdx(0)) } r32` to `r32` under contract ``",
                severity: Error,
                range: [116:43, 116:52),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@StackPure { location: StackLocationIdx(LocalSymbolIdx(2)) } i32` to `i32` under contract ``",
                severity: Error,
                range: [116:54, 116:55),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@StackPure { location: StackLocationIdx(LocalSymbolIdx(1)) } r32` to `r32` under contract ``",
                severity: Error,
                range: [117:43, 117:52),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@StackPure { location: StackLocationIdx(LocalSymbolIdx(2)) } i32` to `i32` under contract ``",
                severity: Error,
                range: [117:54, 117:55),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `Point2d` to `Point2d` under contract `move `",
                severity: Error,
                range: [165:5, 168:6),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `Point2d` to `Point2d` under contract `move `",
                severity: Error,
                range: [162:5, 168:6),
            },
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
                message: "Term Error: expected coersion from `List RawContour` to `List RawContour` under contract `move `",
                severity: Error,
                range: [171:36, 171:38),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `_i` to `@StackPure { location: StackLocationIdx(LocalSymbolIdx(3)) } _i` under contract ``",
                severity: Error,
                range: [174:30, 174:31),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [178:9, 178:82),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `List Point2d` to `List Point2d` under contract `move `",
                severity: Error,
                range: [181:42, 181:44),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [202:24, 202:31),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [202:36, 202:43),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [202:24, 202:43),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [202:48, 202:72),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [202:24, 202:72),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [202:23, 202:73),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [202:22, 202:73),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [211:17, 211:81),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [214:24, 214:48),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [215:33, 215:57),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [214:24, 215:57),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [216:33, 216:52),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [214:24, 216:52),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [217:33, 217:51),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [214:24, 217:51),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [218:33, 218:50),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [214:24, 218:50),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@MutableStackOwned { location: StackLocationIdx(LocalSymbolIdx(9)) } List Point2d` to `List Point2d` under contract ``",
                severity: Error,
                range: [219:68, 219:75),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [219:25, 219:76),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `Point2d` to `Point2d` under contract ``",
                severity: Error,
                range: [220:38, 220:67),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [220:25, 220:68),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [221:25, 221:42),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [222:25, 222:42),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [223:26, 223:50),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [224:33, 224:49),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [223:26, 224:49),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [225:33, 225:50),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [223:26, 225:50),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [226:25, 226:72),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [227:25, 227:52),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [228:25, 228:54),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [229:26, 229:50),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [230:33, 230:49),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [229:26, 230:49),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [231:33, 231:52),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [229:26, 231:52),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [232:33, 232:49),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [229:26, 232:49),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [233:25, 233:72),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [234:25, 234:42),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [235:25, 235:42),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `Point2d` to `Point2d` under contract ``",
                severity: Error,
                range: [237:38, 237:67),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [237:25, 237:68),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [238:25, 238:52),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [239:25, 239:54),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [241:21, 241:39),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [243:21, 243:60),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [244:21, 244:54),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [259:17, 259:53),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [261:20, 261:40),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [263:16, 263:40),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [263:45, 263:64),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [263:16, 263:64),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [263:69, 263:85),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `bool` to `bool` under contract ``",
                severity: Error,
                range: [263:16, 263:85),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `Option Point2d` to `unit` under contract ``",
                severity: Error,
                range: [264:17, 264:30),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@StackPure { location: StackLocationIdx(LocalSymbolIdx(0)) } Leash ConnectedComponent` to `Leash ConnectedComponent` under contract `move `",
                severity: Error,
                range: [265:36, 265:38),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@MutableStackOwned { location: StackLocationIdx(LocalSymbolIdx(9)) } List Point2d` to `List Point2d` under contract `move `",
                severity: Error,
                range: [265:40, 265:47),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `RawContour` to `RawContour` under contract ``",
                severity: Error,
                range: [265:25, 265:48),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [265:13, 265:49),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `@MutableStackOwned { location: StackLocationIdx(LocalSymbolIdx(1)) } List RawContour` to `List RawContour` under contract `move `",
                severity: Error,
                range: [266:12, 266:18),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `Html` to `Html` under contract `move `",
                severity: Error,
                range: [13:9, 13:41),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `Html` to `Html` under contract `move `",
                severity: Error,
                range: [13:9, 13:41),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `f32` to `f32` under contract ``",
                severity: Error,
                range: [17:38, 17:41),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `LineSegmentSketch` to `LineSegmentSketch` under contract `move `",
                severity: Error,
                range: [17:9, 17:42),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `LineSegmentSketch` to `LineSegmentSketch` under contract `move `",
                severity: Error,
                range: [17:9, 17:42),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [27:13, 27:37),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [28:13, 28:37),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [29:13, 29:37),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [30:13, 30:37),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `ClosedRange` to `ClosedRange` under contract `move `",
                severity: Error,
                range: [32:13, 32:36),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `ClosedRange` to `ClosedRange` under contract `move `",
                severity: Error,
                range: [33:13, 33:36),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `BoundingBox` to `BoundingBox` under contract `move `",
                severity: Error,
                range: [31:16, 34:10),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [44:13, 44:65),
            },
            Diagnostic {
                message: "Term Error: expected coersion from `unit` to `unit` under contract ``",
                severity: Error,
                range: [47:9, 47:61),
            },
        ],
    },
}