Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    RegularStruct(
                        RegularStructTypeDecl {
                            path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                            ast_idx: 3,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                ),
                                            },
                                            PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    11,
                                                ),
                                                opd: 0,
                                            },
                                            Err(
                                                NoLeftOperandForBinaryOperator {
                                                    binary_token_idx: TokenIdx(
                                                        17,
                                                    ),
                                                },
                                            ),
                                            Err(
                                                NoRightOperandForBinaryOperator {
                                                    lopd: 2,
                                                    punctuation: PureClosed(
                                                        RemEuclid,
                                                    ),
                                                    punctuation_token_idx: TokenIdx(
                                                        17,
                                                    ),
                                                },
                                            ),
                                            NewBoxList {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    16,
                                                ),
                                                items: ArenaIdxRange(
                                                    3..4,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    18,
                                                ),
                                            },
                                            EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                ),
                                            },
                                            Application {
                                                function: 4,
                                                argument: 5,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    12,
                                                ),
                                                ident: `LineSegmentSketch`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                            },
                                            Root {
                                                token_idx: TokenIdx(
                                                    19,
                                                ),
                                                ident: `LineSegmentStroke`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_maps: [],
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                    },
                                    symbol_region: SymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: FieldType,
                                            expr: 1,
                                        },
                                        ExprRoot {
                                            kind: FieldType,
                                            expr: 6,
                                        },
                                    ],
                                },
                            },
                            implicit_parameter_decl_list: None,
                            lcurl: LeftCurlyBraceToken {
                                token_idx: TokenIdx(
                                    8,
                                ),
                            },
                            fields: [
                                RegularStructFieldPattern {
                                    ident_token: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 96,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            9,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            10,
                                        ),
                                    },
                                    ty: 1,
                                },
                                RegularStructFieldPattern {
                                    ident_token: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 311,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            14,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            15,
                                        ),
                                    },
                                    ty: 6,
                                },
                            ],
                            separators: [
                                CommaToken {
                                    token_idx: TokenIdx(
                                        13,
                                    ),
                                },
                                CommaToken {
                                    token_idx: TokenIdx(
                                        20,
                                    ),
                                },
                            ],
                            rcurl: RightCurlyBraceToken {
                                token_idx: TokenIdx(
                                    21,
                                ),
                            },
                        },
                    ),
                ),
            ),
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 4,
                            impl_block: ImplBlock {
                                id: ImplBlockId {
                                    module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                    impl_block_kind: Type {
                                        ty: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                    },
                                },
                                ast_idx: 4,
                                body: ArenaIdxRange(
                                    0..2,
                                ),
                                variant: Type {
                                    ty: TypePath(
                                        Id {
                                            value: 32,
                                        },
                                    ),
                                },
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    22,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        24,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        ImplBlock(
                                            ImplBlock {
                                                id: ImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                                    impl_block_kind: Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                                    },
                                                },
                                                ast_idx: 4,
                                                body: ArenaIdxRange(
                                                    0..2,
                                                ),
                                                variant: Type {
                                                    ty: TypePath(
                                                        Id {
                                                            value: 32,
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    23,
                                                ),
                                                ident: `ConvexComponent`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_maps: [],
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                    },
                                    symbol_region: SymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: Type,
                                            expr: 0,
                                        },
                                    ],
                                },
                            },
                        },
                    ),
                ),
            ),
        ],
    },
)