Ok(
    DeclSheet {
        [salsa id]: 41,
        decls: [
            (
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                    ),
                ),
                Decl::ModuleItem(
                    ModuleItemDecl::Type(
                        TypeDecl::RegularStruct(
                            RegularStructTypeDecl {
                                path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                implicit_parameters: [],
                                fields: [
                                    RegularStructFieldDeclPattern {
                                        decorators: [],
                                        visibility: None,
                                        ident_token: IdentToken {
                                            ident: `line_segment_sketch`,
                                            token_idx: TokenIdx(
                                                9,
                                            ),
                                        },
                                        colon: ColonToken(
                                            TokenIdx(
                                                10,
                                            ),
                                        ),
                                        ty_expr_idx: 1,
                                        initialization: None,
                                    },
                                    RegularStructFieldDeclPattern {
                                        decorators: [],
                                        visibility: None,
                                        ident_token: IdentToken {
                                            ident: `line_segments`,
                                            token_idx: TokenIdx(
                                                14,
                                            ),
                                        },
                                        colon: ColonToken(
                                            TokenIdx(
                                                15,
                                            ),
                                        ),
                                        ty_expr_idx: 7,
                                        initialization: None,
                                    },
                                ],
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            EntityNodePath::ModuleItem(
                                                ModuleItemNodePath::Type(
                                                    TypeNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 0,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Prefix {
                                                    opr: Ref,
                                                    opr_token_idx: TokenIdx(
                                                        11,
                                                    ),
                                                    opd: 0,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                18,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        NoRightOperandForBinaryOperator {
                                                            punctuation: Closed(
                                                                RemEuclid,
                                                            ),
                                                            punctuation_token_idx: TokenIdx(
                                                                18,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        17,
                                                    ),
                                                    items: [
                                                        CommaListItem {
                                                            expr_idx: 3,
                                                            comma_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_token_idx: TokenIdx(
                                                        19,
                                                    ),
                                                },
                                                Expr::Prefix {
                                                    opr: Tilde,
                                                    opr_token_idx: TokenIdx(
                                                        16,
                                                    ),
                                                    opd: 4,
                                                },
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 5,
                                                    argument: 6,
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `LineSegmentSketch`,
                                                            token_idx: TokenIdx(
                                                                12,
                                                            ),
                                                        },
                                                    ),
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `LineSegmentStroke`,
                                                            token_idx: TokenIdx(
                                                                20,
                                                            ),
                                                        },
                                                    ),
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        ),
                                                    ),
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
                                            pattern_expr_contracts: ArenaMap {
                                                data: [],
                                            },
                                            pattern_infos: [],
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                            pattern_symbol_maps: [],
                                            pattern_symbol_modifiers: ArenaMap {
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
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: RegularStructFieldType {
                                                    ident_token: IdentToken {
                                                        ident: Ident(
                                                            Word(
                                                                Id {
                                                                    value: 195,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                },
                                                expr_idx: 1,
                                            },
                                            ExprRoot {
                                                kind: RegularStructFieldType {
                                                    ident_token: IdentToken {
                                                        ident: Ident(
                                                            Word(
                                                                Id {
                                                                    value: 392,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                },
                                                expr_idx: 7,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                EntityPath::ImplBlock(
                    TraitForTypeImplBlock(
                        TraitForTypeImplBlockPath(
                            Id {
                                value: 38,
                            },
                        ),
                    ),
                ),
                Decl::ImplBlock(
                    ImplBlockDecl::TraitForType(
                        TraitForTypeImplBlockDecl {
                            path: TraitForTypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            implicit_parameters: [],
                            trai_expr: TraitExpr {
                                expr: 0,
                            },
                            ty_expr: TypeExpr {
                                expr: 1,
                            },
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ImplBlock(
                                            ImplBlockNodePath::TraitForTypeImplBlock(
                                                TraitForTypeImplBlockNodePath {
                                                    path: TraitForTypeImplBlockPath {
                                                        module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::visual::Visualize`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 1,
                                                path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Visualize`,
                                                        token_idx: TokenIdx(
                                                            24,
                                                        ),
                                                    },
                                                ),
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Trait(
                                                        TraitPath(`core::visual::Visualize`),
                                                    ),
                                                ),
                                            },
                                            EntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `ConvexComponent`,
                                                        token_idx: TokenIdx(
                                                            26,
                                                        ),
                                                    },
                                                ),
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                                    ),
                                                ),
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
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                        pattern_symbol_maps: [],
                                        pattern_symbol_modifiers: ArenaMap {
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
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: Trait,
                                            expr_idx: 0,
                                        },
                                        ExprRoot {
                                            kind: SelfType,
                                            expr_idx: 1,
                                        },
                                    ],
                                },
                            },
                        },
                    ),
                ),
            ),
            (
                EntityPath::AssociatedItem(
                    AssociatedItemPath::TraitForTypeItem(
                        TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `visualize`,
                            item_kind: MethodFn,
                        },
                    ),
                ),
                Decl::AssociatedItem(
                    AssociatedItemDecl::TraitForTypeItem(
                        TraitForTypeItemDecl::MethodFn(
                            TraitForTypeMethodFnDecl {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                        trai_path: TraitPath(`core::visual::Visualize`),
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `visualize`,
                                    item_kind: MethodFn,
                                },
                                implicit_parameters: [],
                                self_parameter: None,
                                regular_parameters: [],
                                return_ty: Some(
                                    ReturnTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        EntityNodePath::ImplBlock(
                                                            ImplBlockNodePath::TraitForTypeImplBlock(
                                                                TraitForTypeImplBlockNodePath {
                                                                    path: TraitForTypeImplBlockPath {
                                                                        module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            Expr::PrincipalEntityPath {
                                                                entity_path_expr: 0,
                                                                path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Trait(
                                                                            TraitPath(`core::visual::Visualize`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            Expr::PrincipalEntityPath {
                                                                entity_path_expr: 1,
                                                                path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `Visualize`,
                                                                        token_idx: TokenIdx(
                                                                            24,
                                                                        ),
                                                                    },
                                                                ),
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Trait(
                                                                        TraitPath(`core::visual::Visualize`),
                                                                    ),
                                                                ),
                                                            },
                                                            EntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `ConvexComponent`,
                                                                        token_idx: TokenIdx(
                                                                            26,
                                                                        ),
                                                                    },
                                                                ),
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                                                    ),
                                                                ),
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
                                                        pattern_expr_contracts: ArenaMap {
                                                            data: [],
                                                        },
                                                        pattern_infos: [],
                                                        pattern_symbol_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_symbol_maps: [],
                                                        pattern_symbol_modifiers: ArenaMap {
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
                                                        pattern_ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Trait,
                                                            expr_idx: 0,
                                                        },
                                                        ExprRoot {
                                                            kind: SelfType,
                                                            expr_idx: 1,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: RegionPath::Decl(
                                            EntityNodePath::AssociatedItem(
                                                AssociatedItemNodePath::TraitForTypeItem(
                                                    TraitForTypeItemNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TraitForTypeItemPath {
                                                                impl_block: TraitForTypeImplBlockPath {
                                                                    module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `visualize`,
                                                                item_kind: MethodFn,
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 0,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::visual::Html`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `Html`,
                                                            token_idx: TokenIdx(
                                                                33,
                                                            ),
                                                        },
                                                    ),
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::visual::Html`, `Extern`),
                                                        ),
                                                    ),
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
                                            pattern_expr_contracts: ArenaMap {
                                                data: [],
                                            },
                                            pattern_infos: [],
                                            pattern_symbol_arena: Arena {
                                                data: [],
                                            },
                                            pattern_symbol_maps: [],
                                            pattern_symbol_modifiers: ArenaMap {
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
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: ReturnType,
                                                expr_idx: 0,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)