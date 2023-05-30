Ok(
    DeclSheet {
        decls: [
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Type(
                        TypeDecl::RegularStruct(
                            RegularStructTypeDecl {
                                path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                ast_idx: 74,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                    opr: Tilde,
                                                    opr_token_idx: TokenIdx(
                                                        38,
                                                    ),
                                                    opd: 0,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                45,
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
                                                                45,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        44,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        46,
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
                                                Expr::Prefix {
                                                    opr: Tilde,
                                                    opr_token_idx: TokenIdx(
                                                        43,
                                                    ),
                                                    opd: 6,
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        39,
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
                                                        47,
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
                                                expr_idx: 1,
                                            },
                                            ExprRoot {
                                                kind: FieldType,
                                                expr_idx: 7,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: None,
                                lcurl: LeftCurlyBraceToken(
                                    TokenIdx(
                                        35,
                                    ),
                                ),
                                field_comma_list: (
                                    [
                                        RegularStructFieldDeclPattern {
                                            decorators: [],
                                            visibility: None,
                                            ident_token: IdentToken {
                                                ident: `line_segment_sketch`,
                                                token_idx: TokenIdx(
                                                    36,
                                                ),
                                            },
                                            colon: ColonToken(
                                                TokenIdx(
                                                    37,
                                                ),
                                            ),
                                            ty_expr_idx: 1,
                                            initialization: None,
                                        },
                                        RegularStructFieldDeclPattern {
                                            decorators: [],
                                            visibility: None,
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    41,
                                                ),
                                            },
                                            colon: ColonToken(
                                                TokenIdx(
                                                    42,
                                                ),
                                            ),
                                            ty_expr_idx: 7,
                                            initialization: None,
                                        },
                                    ],
                                    [
                                        CommaToken(
                                            TokenIdx(
                                                40,
                                            ),
                                        ),
                                        CommaToken(
                                            TokenIdx(
                                                48,
                                            ),
                                        ),
                                    ],
                                ),
                                rcurl: RightCurlyBraceToken(
                                    TokenIdx(
                                        49,
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FugitiveDecl::Fn(
                            FnDecl {
                                path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                ast_idx: 77,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
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
                                                        537,
                                                    ),
                                                    opd: 0,
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        541,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        2..2,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        542,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 2,
                                                    argument: 3,
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        538,
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
                                                        543,
                                                    ),
                                                    ident: `ConcaveComponent`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                data: [
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `line_segment_sketch`,
                                                            token_idx: TokenIdx(
                                                                535,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    Pure,
                                                ],
                                            },
                                            pattern_infos: [
                                                Parameter,
                                            ],
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    PatternSymbol::Atom(
                                                        0,
                                                    ),
                                                ],
                                            },
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `line_segment_sketch`,
                                                        0,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Pure,
                                                ],
                                            },
                                        },
                                        symbol_region: SymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            536,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `line_segment_sketch`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 0,
                                                        ty: 1,
                                                    },
                                                    ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 1,
                                            },
                                            ExprRoot {
                                                kind: ReturnType,
                                                expr_idx: 4,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ExplicitParameterDeclList {
                                    lpar: LeftParenthesisToken(
                                        TokenIdx(
                                            534,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    regular_parameters: [
                                        RegularParameterDeclPattern {
                                            pattern: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    536,
                                                ),
                                            ),
                                            ty: 1,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            539,
                                        ),
                                    ),
                                },
                                curry_token: Some(
                                    CurryToken(
                                        TokenIdx(
                                            540,
                                        ),
                                    ),
                                ),
                                return_ty: Some(
                                    ReturnTypeExpr {
                                        expr: 4,
                                    },
                                ),
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            544,
                                        ),
                                    },
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
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Decl::ImplBlock(
                        ImplBlockDecl::TraitForType(
                            TraitForTypeImplBlockDecl {
                                ast_idx: 75,
                                impl_block: TraitForTypeImplBlock {
                                    id: TraitForTypeImplBlockId {
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        trai_path: TraitPath(`core::visual::Visualize`),
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 75,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            50,
                                        ),
                                    },
                                    trai_expr: 17,
                                    for_token: TokenIdx(
                                        52,
                                    ),
                                    ty_expr: 18,
                                    items: Some(
                                        TraitForType(
                                            TraitForTypeItems {
                                                ast_idx_range: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        50,
                                    ),
                                },
                                implicit_parameter_decl_list: None,
                                trai_expr: TraitExpr {
                                    expr: 0,
                                },
                                for_token: ConnectionForToken {
                                    token_idx: TokenIdx(
                                        52,
                                    ),
                                },
                                ty_expr: TypeExpr {
                                    expr: 1,
                                },
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            54,
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
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                        51,
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
                                                        53,
                                                    ),
                                                    ident: `ConcaveComponent`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::TraitForType(
                            TraitForTypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            trai: TraitPath(`core::visual::Visualize`),
                                            ident: `visualize`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::TraitForType(
                                                TraitForTypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `visualize`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TraitForTypeItem(
                                                TraitForTypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    trai: TraitPath(`core::visual::Visualize`),
                                                    ident: `visualize`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::TraitForType(
                                            TraitForTypeImplBlock {
                                                id: TraitForTypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 75,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        50,
                                                    ),
                                                },
                                                trai_expr: 17,
                                                for_token: TokenIdx(
                                                    52,
                                                ),
                                                ty_expr: 18,
                                                items: Some(
                                                    TraitForType(
                                                        TraitForTypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        ast_idx: 1,
                                        ident: `visualize`,
                                        associated_item_kind: TraitForTypeItem(
                                            MethodFn,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::line_segment_sketch::concave_component`,
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
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        51,
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
                                                                        53,
                                                                    ),
                                                                    ident: `ConcaveComponent`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::TraitForType(
                                                            TraitForTypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                            60,
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
                                                    expr_idx: 0,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    explicit_parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                57,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        regular_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                58,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                59,
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
                                                61,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::ImplBlock(
                    ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Decl::ImplBlock(
                        ImplBlockDecl::Type(
                            TypeImplBlockDecl {
                                ast_idx: 76,
                                impl_block: TypeImplBlock {
                                    id: TypeImplBlockId {
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 76,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            69,
                                        ),
                                    },
                                    ty_expr: 19,
                                    body: Type(
                                        TypeItems {
                                            ast_idx_range: ArenaIdxRange(
                                                41..53,
                                            ),
                                        },
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        69,
                                    ),
                                },
                                implicit_parameter_decl_list: None,
                                ty_expr: TypeExpr {
                                    expr: 0,
                                },
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            71,
                                        ),
                                    },
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                        70,
                                                    ),
                                                    ident: `ConcaveComponent`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                kind: SelfType,
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
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `norm`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::MemoizedField(
                                TypeMemoizedFieldDecl {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `norm`,
                                            item_kind: MemoizedField,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `norm`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ident: `norm`,
                                                    item_kind: MemoizedField,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 76,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        69,
                                                    ),
                                                },
                                                ty_expr: 19,
                                                body: Type(
                                                    TypeItems {
                                                        ast_idx_range: ArenaIdxRange(
                                                            41..53,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        ast_idx: 41,
                                        ident: `norm`,
                                        associated_item_kind: TypeItem(
                                            MemoizedField,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::line_segment_sketch::concave_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 41,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                74,
                                            ),
                                        ),
                                    ),
                                    memo_ty: Some(
                                        FormTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eq_token: EqToken(
                                        TokenIdx(
                                            76,
                                        ),
                                    ),
                                    expr: None,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId::Type(
                                                                    TypeImplBlockId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        70,
                                                                    ),
                                                                    ident: `ConcaveComponent`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                kind: SelfType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `norm`,
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
                                                                    TypePath(`core::num::f32`, `Extern`),
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
                                                            75,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
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
                                                    kind: VarType,
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
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `rel_norm`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::MemoizedField(
                                TypeMemoizedFieldDecl {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `rel_norm`,
                                            item_kind: MemoizedField,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `rel_norm`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ident: `rel_norm`,
                                                    item_kind: MemoizedField,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 76,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        69,
                                                    ),
                                                },
                                                ty_expr: 19,
                                                body: Type(
                                                    TypeItems {
                                                        ast_idx_range: ArenaIdxRange(
                                                            41..53,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        ast_idx: 42,
                                        ident: `rel_norm`,
                                        associated_item_kind: TypeItem(
                                            MemoizedField,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::line_segment_sketch::concave_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 42,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                82,
                                            ),
                                        ),
                                    ),
                                    memo_ty: Some(
                                        FormTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eq_token: EqToken(
                                        TokenIdx(
                                            84,
                                        ),
                                    ),
                                    expr: None,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId::Type(
                                                                    TypeImplBlockId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        70,
                                                                    ),
                                                                    ident: `ConcaveComponent`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                kind: SelfType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `rel_norm`,
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
                                                                    TypePath(`core::num::f32`, `Extern`),
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
                                                            83,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
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
                                                    kind: VarType,
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
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `hausdorff_norm`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::MemoizedField(
                                TypeMemoizedFieldDecl {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `hausdorff_norm`,
                                            item_kind: MemoizedField,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `hausdorff_norm`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ident: `hausdorff_norm`,
                                                    item_kind: MemoizedField,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 76,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        69,
                                                    ),
                                                },
                                                ty_expr: 19,
                                                body: Type(
                                                    TypeItems {
                                                        ast_idx_range: ArenaIdxRange(
                                                            41..53,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        ast_idx: 43,
                                        ident: `hausdorff_norm`,
                                        associated_item_kind: TypeItem(
                                            MemoizedField,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::line_segment_sketch::concave_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 43,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                100,
                                            ),
                                        ),
                                    ),
                                    memo_ty: Some(
                                        FormTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eq_token: EqToken(
                                        TokenIdx(
                                            102,
                                        ),
                                    ),
                                    expr: None,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId::Type(
                                                                    TypeImplBlockId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        70,
                                                                    ),
                                                                    ident: `ConcaveComponent`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                kind: SelfType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `hausdorff_norm`,
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
                                                                    TypePath(`core::num::f32`, `Extern`),
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
                                                            101,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
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
                                                    kind: VarType,
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
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `angle_change`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::MemoizedField(
                                TypeMemoizedFieldDecl {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `angle_change`,
                                            item_kind: MemoizedField,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `angle_change`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ident: `angle_change`,
                                                    item_kind: MemoizedField,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 76,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        69,
                                                    ),
                                                },
                                                ty_expr: 19,
                                                body: Type(
                                                    TypeItems {
                                                        ast_idx_range: ArenaIdxRange(
                                                            41..53,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        ast_idx: 44,
                                        ident: `angle_change`,
                                        associated_item_kind: TypeItem(
                                            MemoizedField,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::line_segment_sketch::concave_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 44,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                187,
                                            ),
                                        ),
                                    ),
                                    memo_ty: Some(
                                        FormTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eq_token: EqToken(
                                        TokenIdx(
                                            189,
                                        ),
                                    ),
                                    expr: None,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId::Type(
                                                                    TypeImplBlockId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        70,
                                                                    ),
                                                                    ident: `ConcaveComponent`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                kind: SelfType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `angle_change`,
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
                                                                    TypePath(`core::num::f32`, `Extern`),
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
                                                            188,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
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
                                                    kind: VarType,
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
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `bounding_box`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::MemoizedField(
                                TypeMemoizedFieldDecl {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `bounding_box`,
                                            item_kind: MemoizedField,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `bounding_box`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ident: `bounding_box`,
                                                    item_kind: MemoizedField,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 76,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        69,
                                                    ),
                                                },
                                                ty_expr: 19,
                                                body: Type(
                                                    TypeItems {
                                                        ast_idx_range: ArenaIdxRange(
                                                            41..53,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        ast_idx: 45,
                                        ident: `bounding_box`,
                                        associated_item_kind: TypeItem(
                                            MemoizedField,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::line_segment_sketch::concave_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 45,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                258,
                                            ),
                                        ),
                                    ),
                                    memo_ty: Some(
                                        FormTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eq_token: EqToken(
                                        TokenIdx(
                                            260,
                                        ),
                                    ),
                                    expr: None,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId::Type(
                                                                    TypeImplBlockId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        70,
                                                                    ),
                                                                    ident: `ConcaveComponent`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                kind: SelfType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `bounding_box`,
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
                                                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                            259,
                                                        ),
                                                        ident: `BoundingBox`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                    kind: VarType,
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
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `relative_bounding_box`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::MemoizedField(
                                TypeMemoizedFieldDecl {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `relative_bounding_box`,
                                            item_kind: MemoizedField,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `relative_bounding_box`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ident: `relative_bounding_box`,
                                                    item_kind: MemoizedField,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 76,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        69,
                                                    ),
                                                },
                                                ty_expr: 19,
                                                body: Type(
                                                    TypeItems {
                                                        ast_idx_range: ArenaIdxRange(
                                                            41..53,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        ast_idx: 46,
                                        ident: `relative_bounding_box`,
                                        associated_item_kind: TypeItem(
                                            MemoizedField,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::line_segment_sketch::concave_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 46,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                387,
                                            ),
                                        ),
                                    ),
                                    memo_ty: Some(
                                        FormTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eq_token: EqToken(
                                        TokenIdx(
                                            389,
                                        ),
                                    ),
                                    expr: None,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId::Type(
                                                                    TypeImplBlockId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        70,
                                                                    ),
                                                                    ident: `ConcaveComponent`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                kind: SelfType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `relative_bounding_box`,
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
                                                                    TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                            388,
                                                        ),
                                                        ident: `RelativeBoundingBox`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                    kind: VarType,
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
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `line_segment`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::MethodFn(
                                TypeMethodFnDecl {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId::Type(
                                            TypeImplBlockId {
                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `line_segment`,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `line_segment`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `line_segment`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ident: `line_segment`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 76,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        69,
                                                    ),
                                                },
                                                ty_expr: 19,
                                                body: Type(
                                                    TypeItems {
                                                        ast_idx_range: ArenaIdxRange(
                                                            41..53,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        ast_idx: 47,
                                        ident: `line_segment`,
                                        associated_item_kind: TypeItem(
                                            MethodFn,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::line_segment_sketch::concave_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 47,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId::Type(
                                                                    TypeImplBlockId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        70,
                                                                    ),
                                                                    ident: `ConcaveComponent`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                kind: SelfType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `line_segment`,
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
                                                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                            407,
                                                        ),
                                                        ident: `LineSegment`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                    implicit_parameter_decl_list: None,
                                    explicit_parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                404,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        regular_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                405,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                406,
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
                                                408,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `start`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::MethodFn(
                                TypeMethodFnDecl {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId::Type(
                                            TypeImplBlockId {
                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `start`,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `start`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `start`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ident: `start`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 76,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        69,
                                                    ),
                                                },
                                                ty_expr: 19,
                                                body: Type(
                                                    TypeItems {
                                                        ast_idx_range: ArenaIdxRange(
                                                            41..53,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        ast_idx: 48,
                                        ident: `start`,
                                        associated_item_kind: TypeItem(
                                            MethodFn,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::line_segment_sketch::concave_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 48,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId::Type(
                                                                    TypeImplBlockId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        70,
                                                                    ),
                                                                    ident: `ConcaveComponent`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                kind: SelfType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `start`,
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
                                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                            444,
                                                        ),
                                                        ident: `Point2d`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                    implicit_parameter_decl_list: None,
                                    explicit_parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                441,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        regular_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                442,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                443,
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
                                                445,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `end`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::MethodFn(
                                TypeMethodFnDecl {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId::Type(
                                            TypeImplBlockId {
                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `end`,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `end`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `end`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ident: `end`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 76,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        69,
                                                    ),
                                                },
                                                ty_expr: 19,
                                                body: Type(
                                                    TypeItems {
                                                        ast_idx_range: ArenaIdxRange(
                                                            41..53,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        ast_idx: 49,
                                        ident: `end`,
                                        associated_item_kind: TypeItem(
                                            MethodFn,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::line_segment_sketch::concave_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 49,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId::Type(
                                                                    TypeImplBlockId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        70,
                                                                    ),
                                                                    ident: `ConcaveComponent`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                kind: SelfType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `end`,
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
                                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                            464,
                                                        ),
                                                        ident: `Point2d`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                    implicit_parameter_decl_list: None,
                                    explicit_parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                461,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        regular_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                462,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                463,
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
                                                465,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `displacement`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::MethodFn(
                                TypeMethodFnDecl {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId::Type(
                                            TypeImplBlockId {
                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `displacement`,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `displacement`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `displacement`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ident: `displacement`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 76,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        69,
                                                    ),
                                                },
                                                ty_expr: 19,
                                                body: Type(
                                                    TypeItems {
                                                        ast_idx_range: ArenaIdxRange(
                                                            41..53,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        ast_idx: 50,
                                        ident: `displacement`,
                                        associated_item_kind: TypeItem(
                                            MethodFn,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::line_segment_sketch::concave_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 50,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId::Type(
                                                                    TypeImplBlockId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        70,
                                                                    ),
                                                                    ident: `ConcaveComponent`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                kind: SelfType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `displacement`,
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
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                            484,
                                                        ),
                                                        ident: `Vector2d`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                    implicit_parameter_decl_list: None,
                                    explicit_parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                481,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        regular_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                482,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                483,
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
                                                485,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `start_tangent`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::MethodFn(
                                TypeMethodFnDecl {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId::Type(
                                            TypeImplBlockId {
                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `start_tangent`,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `start_tangent`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `start_tangent`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ident: `start_tangent`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 76,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        69,
                                                    ),
                                                },
                                                ty_expr: 19,
                                                body: Type(
                                                    TypeItems {
                                                        ast_idx_range: ArenaIdxRange(
                                                            41..53,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        ast_idx: 51,
                                        ident: `start_tangent`,
                                        associated_item_kind: TypeItem(
                                            MethodFn,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::line_segment_sketch::concave_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 51,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId::Type(
                                                                    TypeImplBlockId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        70,
                                                                    ),
                                                                    ident: `ConcaveComponent`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                kind: SelfType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `start_tangent`,
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
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                            500,
                                                        ),
                                                        ident: `Vector2d`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                    implicit_parameter_decl_list: None,
                                    explicit_parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                497,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        regular_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                498,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                499,
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
                                                501,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `end_tangent`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::MethodFn(
                                TypeMethodFnDecl {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId::Type(
                                            TypeImplBlockId {
                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `end_tangent`,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `end_tangent`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `end_tangent`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ident: `end_tangent`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 76,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        69,
                                                    ),
                                                },
                                                ty_expr: 19,
                                                body: Type(
                                                    TypeItems {
                                                        ast_idx_range: ArenaIdxRange(
                                                            41..53,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        ast_idx: 52,
                                        ident: `end_tangent`,
                                        associated_item_kind: TypeItem(
                                            MethodFn,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::line_segment_sketch::concave_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 52,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId::Type(
                                                                    TypeImplBlockId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        70,
                                                                    ),
                                                                    ident: `ConcaveComponent`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                kind: SelfType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `end_tangent`,
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
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                            518,
                                                        ),
                                                        ident: `Vector2d`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                    implicit_parameter_decl_list: None,
                                    explicit_parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                515,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        regular_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                516,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                517,
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
                                                519,
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