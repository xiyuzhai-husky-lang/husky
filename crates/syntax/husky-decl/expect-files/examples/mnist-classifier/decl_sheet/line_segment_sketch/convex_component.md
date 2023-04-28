Ok(
    DeclSheet {
        decls: [
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Type(
                        TypeDecl::RegularStruct(
                            RegularStructTypeDecl {
                                path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                ast_idx: 3,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
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
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                17,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoRightOperandForBinaryOperator {
                                                            punctuation: Closed(
                                                                RemEuclid,
                                                            ),
                                                            punctuation_token_idx: TokenIdx(
                                                                17,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::List {
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
                                                Expr::EntityPath {
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
                                                    function: 4,
                                                    argument: 5,
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        12,
                                                    ),
                                                    ident: `LineSegmentSketch`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        19,
                                                    ),
                                                    ident: `LineSegmentStroke`,
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
                                lcurl: LeftCurlyBraceToken(
                                    TokenIdx(
                                        8,
                                    ),
                                ),
                                field_comma_list: (
                                    [
                                        FieldDeclPattern {
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
                                            ty: 1,
                                            initialization: None,
                                        },
                                        FieldDeclPattern {
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
                                            ty: 6,
                                            initialization: None,
                                        },
                                    ],
                                    [
                                        CommaToken(
                                            TokenIdx(
                                                13,
                                            ),
                                        ),
                                        CommaToken(
                                            TokenIdx(
                                                20,
                                            ),
                                        ),
                                    ],
                                ),
                                rcurl: RightCurlyBraceToken(
                                    TokenIdx(
                                        21,
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::ImplBlock(
                    ImplBlockId::TraitForType(
                        TraitForTypeImplBlockId {
                            module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Decl::ImplBlock(
                        ImplBlockDecl::TraitForType(
                            TraitForTypeImplBlockDecl {
                                ast_idx: 4,
                                impl_block: TraitForTypeImplBlock {
                                    id: TraitForTypeImplBlockId {
                                        module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                        trai_path: TraitPath(`core::visual::Visualize`),
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 4,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            22,
                                        ),
                                    },
                                    trai_expr: 18,
                                    for_token: TokenIdx(
                                        24,
                                    ),
                                    ty_expr: 19,
                                    body: TraitForType(
                                        TraitForTypeItems {
                                            ast_idx_range: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        22,
                                    ),
                                },
                                implicit_parameter_decl_list: None,
                                trai_expr: TraitExpr {
                                    expr: 0,
                                },
                                for_token: ConnectionForToken {
                                    token_idx: TokenIdx(
                                        24,
                                    ),
                                },
                                ty_expr: TypeExpr {
                                    expr: 1,
                                },
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            26,
                                        ),
                                    },
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId::TraitForType(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::visual::Visualize`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
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
                                                    token_idx: TokenIdx(
                                                        23,
                                                    ),
                                                    ident: `Visualize`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::visual::Visualize`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        25,
                                                    ),
                                                    ident: `ConvexComponent`,
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
                                                expr: 0,
                                            },
                                            ExprRoot {
                                                kind: SelfType,
                                                expr: 1,
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
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::TraitForType(
                            TraitForTypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `visualize`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TraitForTypeItem(
                            TraitForTypeItemDecl::MethodFn(
                                TraitForTypeMethodFnDecl {
                                    path: Some(
                                        TraitForTypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                            trai: TraitPath(`core::visual::Visualize`),
                                            ident: `visualize`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::TraitForType(
                                                TraitForTypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `visualize`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TraitForTypeItem(
                                                TraitForTypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                                    trai: TraitPath(`core::visual::Visualize`),
                                                    ident: `visualize`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::TraitForType(
                                            TraitForTypeImplBlock {
                                                id: TraitForTypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 4,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        22,
                                                    ),
                                                },
                                                trai_expr: 18,
                                                for_token: TokenIdx(
                                                    24,
                                                ),
                                                ty_expr: 19,
                                                body: TraitForType(
                                                    TraitForTypeItems {
                                                        ast_idx_range: ArenaIdxRange(
                                                            1..2,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        ast_idx: 1,
                                        ident: `visualize`,
                                        associated_item_kind: TraitForTypeItem(
                                            MethodFn,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::line_segment_sketch::convex_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 1,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId::TraitForType(
                                                                    TraitForTypeImplBlockId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Trait(
                                                                                TraitPath(`core::visual::Visualize`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                Expr::EntityPath {
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
                                                                    token_idx: TokenIdx(
                                                                        23,
                                                                    ),
                                                                    ident: `Visualize`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Trait(
                                                                            TraitPath(`core::visual::Visualize`),
                                                                        ),
                                                                    ),
                                                                },
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        25,
                                                                    ),
                                                                    ident: `ConvexComponent`,
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
                                                                expr: 0,
                                                            },
                                                            ExprRoot {
                                                                kind: SelfType,
                                                                expr: 1,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::TraitForType(
                                                            TraitForTypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `visualize`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
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
                                                        token_idx: TokenIdx(
                                                            32,
                                                        ),
                                                        ident: `Html`,
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
                                                    expr: 0,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    explicit_parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                29,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        regular_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                30,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                31,
                                            ),
                                        ),
                                    ),
                                    return_ty: Some(
                                        ReturnTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: EolToken::Colon(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                33,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
        ],
    },
)