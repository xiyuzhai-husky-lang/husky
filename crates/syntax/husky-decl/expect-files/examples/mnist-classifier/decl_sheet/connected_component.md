Ok(
    DeclSheet {
        [salsa id]: 26,
        decls: [
            (
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                    ),
                ),
                Decl::ModuleItem(
                    ModuleItemDecl::Type(
                        TypeDecl::PropsStruct(
                            PropsStructTypeDecl {
                                path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                implicit_parameters: [],
                                fields: [
                                    PropsFieldDeclPattern {
                                        decorators: [],
                                        visibility: None,
                                        ident_token: IdentToken {
                                            ident: `row_start`,
                                            token_idx: TokenIdx(
                                                14,
                                            ),
                                        },
                                        colon: ColonToken(
                                            TokenIdx(
                                                15,
                                            ),
                                        ),
                                        ty_expr_idx: 0,
                                        initialization: None,
                                    },
                                    PropsFieldDeclPattern {
                                        decorators: [],
                                        visibility: None,
                                        ident_token: IdentToken {
                                            ident: `row_end`,
                                            token_idx: TokenIdx(
                                                18,
                                            ),
                                        },
                                        colon: ColonToken(
                                            TokenIdx(
                                                19,
                                            ),
                                        ),
                                        ty_expr_idx: 1,
                                        initialization: None,
                                    },
                                    PropsFieldDeclPattern {
                                        decorators: [],
                                        visibility: None,
                                        ident_token: IdentToken {
                                            ident: `upper_mass`,
                                            token_idx: TokenIdx(
                                                22,
                                            ),
                                        },
                                        colon: ColonToken(
                                            TokenIdx(
                                                23,
                                            ),
                                        ),
                                        ty_expr_idx: 2,
                                        initialization: None,
                                    },
                                    PropsFieldDeclPattern {
                                        decorators: [],
                                        visibility: None,
                                        ident_token: IdentToken {
                                            ident: `lower_mass`,
                                            token_idx: TokenIdx(
                                                26,
                                            ),
                                        },
                                        colon: ColonToken(
                                            TokenIdx(
                                                27,
                                            ),
                                        ),
                                        ty_expr_idx: 3,
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
                                                            path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
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
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 2,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 3,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `i32`,
                                                            token_idx: TokenIdx(
                                                                16,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `i32`,
                                                            token_idx: TokenIdx(
                                                                20,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `i32`,
                                                            token_idx: TokenIdx(
                                                                24,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `i32`,
                                                            token_idx: TokenIdx(
                                                                28,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
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
                                                kind: PropsStructFieldType {
                                                    ident_token: IdentToken {
                                                        ident: Ident(
                                                            Word(
                                                                Id {
                                                                    value: 217,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                },
                                                expr_idx: 0,
                                            },
                                            ExprRoot {
                                                kind: PropsStructFieldType {
                                                    ident_token: IdentToken {
                                                        ident: Ident(
                                                            Word(
                                                                Id {
                                                                    value: 218,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                },
                                                expr_idx: 1,
                                            },
                                            ExprRoot {
                                                kind: PropsStructFieldType {
                                                    ident_token: IdentToken {
                                                        ident: Ident(
                                                            Word(
                                                                Id {
                                                                    value: 219,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            22,
                                                        ),
                                                    },
                                                },
                                                expr_idx: 2,
                                            },
                                            ExprRoot {
                                                kind: PropsStructFieldType {
                                                    ident_token: IdentToken {
                                                        ident: Ident(
                                                            Word(
                                                                Id {
                                                                    value: 220,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            26,
                                                        ),
                                                    },
                                                },
                                                expr_idx: 3,
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
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                    ),
                ),
                Decl::ModuleItem(
                    ModuleItemDecl::Type(
                        TypeDecl::PropsStruct(
                            PropsStructTypeDecl {
                                path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                implicit_parameters: [],
                                fields: [
                                    PropsFieldDeclPattern {
                                        decorators: [],
                                        visibility: None,
                                        ident_token: IdentToken {
                                            ident: `matches`,
                                            token_idx: TokenIdx(
                                                35,
                                            ),
                                        },
                                        colon: ColonToken(
                                            TokenIdx(
                                                36,
                                            ),
                                        ),
                                        ty_expr_idx: 4,
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
                                                            path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Prefix {
                                                    opr: Tilde,
                                                    opr_token_idx: TokenIdx(
                                                        40,
                                                    ),
                                                    opd: 0,
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        37,
                                                    ),
                                                    items: [],
                                                    rbox_token_idx: TokenIdx(
                                                        38,
                                                    ),
                                                },
                                                Expr::Prefix {
                                                    opr: Option,
                                                    opr_token_idx: TokenIdx(
                                                        39,
                                                    ),
                                                    opd: 1,
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 2,
                                                    argument: 3,
                                                },
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `RawContour`,
                                                            token_idx: TokenIdx(
                                                                41,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                kind: PropsStructFieldType {
                                                    ident_token: IdentToken {
                                                        ident: Ident(
                                                            Word(
                                                                Id {
                                                                    value: 222,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            35,
                                                        ),
                                                    },
                                                },
                                                expr_idx: 4,
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
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                    ),
                ),
                Decl::ModuleItem(
                    ModuleItemDecl::Type(
                        TypeDecl::PropsStruct(
                            PropsStructTypeDecl {
                                path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                implicit_parameters: [],
                                fields: [
                                    PropsFieldDeclPattern {
                                        decorators: [],
                                        visibility: None,
                                        ident_token: IdentToken {
                                            ident: `mask`,
                                            token_idx: TokenIdx(
                                                73,
                                            ),
                                        },
                                        colon: ColonToken(
                                            TokenIdx(
                                                74,
                                            ),
                                        ),
                                        ty_expr_idx: 0,
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
                                                            path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist::BinaryImage28`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `BinaryImage28`,
                                                            token_idx: TokenIdx(
                                                                75,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::BinaryImage28`, `Struct`),
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
                                                kind: PropsStructFieldType {
                                                    ident_token: IdentToken {
                                                        ident: Ident(
                                                            Word(
                                                                Id {
                                                                    value: 228,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            73,
                                                        ),
                                                    },
                                                },
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
                EntityPath::ImplBlock(
                    TraitForTypeImplBlock(
                        TraitForTypeImplBlockPath(
                            Id {
                                value: 33,
                            },
                        ),
                    ),
                ),
                Decl::ImplBlock(
                    ImplBlockDecl::TraitForType(
                        TraitForTypeImplBlockDecl {
                            path: TraitForTypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                        module_path: `mnist_classifier::connected_component`,
                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::visual::Visualize`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Visualize`,
                                                        token_idx: TokenIdx(
                                                            78,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Trait(
                                                        TraitPath(`core::visual::Visualize`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `ConnectedComponent`,
                                                        token_idx: TokenIdx(
                                                            80,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                module_path: `mnist_classifier::connected_component`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                        module_path: `mnist_classifier::connected_component`,
                                        trai_path: TraitPath(`core::visual::Visualize`),
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `visualize`,
                                    item_kind: MethodFn,
                                },
                                implicit_parameters: [],
                                self_parameter: None,
                                explicit_parameters: [],
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
                                                                        module_path: `mnist_classifier::connected_component`,
                                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Trait(
                                                                            TraitPath(`core::visual::Visualize`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            Expr::PrincipalEntityPath {
                                                                entity_path_expr: 1,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_entity_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `Visualize`,
                                                                        token_idx: TokenIdx(
                                                                            78,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Trait(
                                                                        TraitPath(`core::visual::Visualize`),
                                                                    ),
                                                                ),
                                                            },
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `ConnectedComponent`,
                                                                        token_idx: TokenIdx(
                                                                            80,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::visual::Html`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `Html`,
                                                            token_idx: TokenIdx(
                                                                87,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
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
            (
                EntityPath::ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockPath(
                            Id {
                                value: 31,
                            },
                        ),
                    ),
                ),
                Decl::ImplBlock(
                    ImplBlockDecl::Type(
                        TypeImplBlockDecl {
                            path: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            implicit_parameters: [],
                            ty_expr: TypeExpr {
                                expr: 0,
                            },
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ImplBlock(
                                            ImplBlockNodePath::TypeImplBlock(
                                                TypeImplBlockNodePath {
                                                    path: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::connected_component`,
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `ConnectedComponent`,
                                                        token_idx: TokenIdx(
                                                            97,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
            (
                EntityPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `raw_contours`,
                            item_kind: MemoizedField,
                        },
                    ),
                ),
                Decl::AssociatedItem(
                    AssociatedItemDecl::TypeItem(
                        TypeItemDecl::MemoizedField(
                            TypeMemoizedFieldDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `raw_contours`,
                                    item_kind: MemoizedField,
                                },
                                memo_ty: Some(
                                    FormTypeExpr {
                                        expr: 2,
                                    },
                                ),
                                expr: None,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        EntityNodePath::ImplBlock(
                                                            ImplBlockNodePath::TypeImplBlock(
                                                                TypeImplBlockNodePath {
                                                                    path: TypeImplBlockPath {
                                                                        module_path: `mnist_classifier::connected_component`,
                                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_entity_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `ConnectedComponent`,
                                                                        token_idx: TokenIdx(
                                                                            97,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                            EntityNodePath::AssociatedItem(
                                                AssociatedItemNodePath::TypeItem(
                                                    TypeItemNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `raw_contours`,
                                                                item_kind: MemoizedField,
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        102,
                                                    ),
                                                    items: [],
                                                    rbox_token_idx: TokenIdx(
                                                        103,
                                                    ),
                                                },
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 0,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 0,
                                                    argument: 1,
                                                },
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `RawContour`,
                                                            token_idx: TokenIdx(
                                                                104,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                expr_idx: 2,
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
                EntityPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `eff_holes`,
                            item_kind: MemoizedField,
                        },
                    ),
                ),
                Decl::AssociatedItem(
                    AssociatedItemDecl::TypeItem(
                        TypeItemDecl::MemoizedField(
                            TypeMemoizedFieldDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `eff_holes`,
                                    item_kind: MemoizedField,
                                },
                                memo_ty: Some(
                                    FormTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                expr: None,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        EntityNodePath::ImplBlock(
                                                            ImplBlockNodePath::TypeImplBlock(
                                                                TypeImplBlockNodePath {
                                                                    path: TypeImplBlockPath {
                                                                        module_path: `mnist_classifier::connected_component`,
                                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_entity_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `ConnectedComponent`,
                                                                        token_idx: TokenIdx(
                                                                            97,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                            EntityNodePath::AssociatedItem(
                                                AssociatedItemNodePath::TypeItem(
                                                    TypeItemNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `eff_holes`,
                                                                item_kind: MemoizedField,
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
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `EffHoles`,
                                                            token_idx: TokenIdx(
                                                                113,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
            (
                EntityPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `max_hole_ilen`,
                            item_kind: MemoizedField,
                        },
                    ),
                ),
                Decl::AssociatedItem(
                    AssociatedItemDecl::TypeItem(
                        TypeItemDecl::MemoizedField(
                            TypeMemoizedFieldDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `max_hole_ilen`,
                                    item_kind: MemoizedField,
                                },
                                memo_ty: Some(
                                    FormTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                expr: None,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        EntityNodePath::ImplBlock(
                                                            ImplBlockNodePath::TypeImplBlock(
                                                                TypeImplBlockNodePath {
                                                                    path: TypeImplBlockPath {
                                                                        module_path: `mnist_classifier::connected_component`,
                                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_entity_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `ConnectedComponent`,
                                                                        token_idx: TokenIdx(
                                                                            97,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                            EntityNodePath::AssociatedItem(
                                                AssociatedItemNodePath::TypeItem(
                                                    TypeItemNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `max_hole_ilen`,
                                                                item_kind: MemoizedField,
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
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `f32`,
                                                            token_idx: TokenIdx(
                                                                178,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
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
            (
                EntityPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `max_row_span`,
                            item_kind: MemoizedField,
                        },
                    ),
                ),
                Decl::AssociatedItem(
                    AssociatedItemDecl::TypeItem(
                        TypeItemDecl::MemoizedField(
                            TypeMemoizedFieldDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `max_row_span`,
                                    item_kind: MemoizedField,
                                },
                                memo_ty: Some(
                                    FormTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                expr: None,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        EntityNodePath::ImplBlock(
                                                            ImplBlockNodePath::TypeImplBlock(
                                                                TypeImplBlockNodePath {
                                                                    path: TypeImplBlockPath {
                                                                        module_path: `mnist_classifier::connected_component`,
                                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_entity_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `ConnectedComponent`,
                                                                        token_idx: TokenIdx(
                                                                            97,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                            EntityNodePath::AssociatedItem(
                                                AssociatedItemNodePath::TypeItem(
                                                    TypeItemNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `max_row_span`,
                                                                item_kind: MemoizedField,
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
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `f32`,
                                                            token_idx: TokenIdx(
                                                                230,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
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
            (
                EntityPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `row_span_sum`,
                            item_kind: MemoizedField,
                        },
                    ),
                ),
                Decl::AssociatedItem(
                    AssociatedItemDecl::TypeItem(
                        TypeItemDecl::MemoizedField(
                            TypeMemoizedFieldDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `row_span_sum`,
                                    item_kind: MemoizedField,
                                },
                                memo_ty: Some(
                                    FormTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                expr: None,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        EntityNodePath::ImplBlock(
                                                            ImplBlockNodePath::TypeImplBlock(
                                                                TypeImplBlockNodePath {
                                                                    path: TypeImplBlockPath {
                                                                        module_path: `mnist_classifier::connected_component`,
                                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_entity_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `ConnectedComponent`,
                                                                        token_idx: TokenIdx(
                                                                            97,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                            EntityNodePath::AssociatedItem(
                                                AssociatedItemNodePath::TypeItem(
                                                    TypeItemNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `row_span_sum`,
                                                                item_kind: MemoizedField,
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
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `f32`,
                                                            token_idx: TokenIdx(
                                                                268,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
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
            (
                EntityPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `distribution`,
                            item_kind: MemoizedField,
                        },
                    ),
                ),
                Decl::AssociatedItem(
                    AssociatedItemDecl::TypeItem(
                        TypeItemDecl::MemoizedField(
                            TypeMemoizedFieldDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `distribution`,
                                    item_kind: MemoizedField,
                                },
                                memo_ty: Some(
                                    FormTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                expr: None,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        EntityNodePath::ImplBlock(
                                                            ImplBlockNodePath::TypeImplBlock(
                                                                TypeImplBlockNodePath {
                                                                    path: TypeImplBlockPath {
                                                                        module_path: `mnist_classifier::connected_component`,
                                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_entity_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `ConnectedComponent`,
                                                                        token_idx: TokenIdx(
                                                                            97,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                            EntityNodePath::AssociatedItem(
                                                AssociatedItemNodePath::TypeItem(
                                                    TypeItemNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `distribution`,
                                                                item_kind: MemoizedField,
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
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `ConnectedComponentDistribution`,
                                                            token_idx: TokenIdx(
                                                                301,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
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
            (
                EntityPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `upper_mass`,
                            item_kind: MemoizedField,
                        },
                    ),
                ),
                Decl::AssociatedItem(
                    AssociatedItemDecl::TypeItem(
                        TypeItemDecl::MemoizedField(
                            TypeMemoizedFieldDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `upper_mass`,
                                    item_kind: MemoizedField,
                                },
                                memo_ty: Some(
                                    FormTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                expr: None,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        EntityNodePath::ImplBlock(
                                                            ImplBlockNodePath::TypeImplBlock(
                                                                TypeImplBlockNodePath {
                                                                    path: TypeImplBlockPath {
                                                                        module_path: `mnist_classifier::connected_component`,
                                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_entity_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `ConnectedComponent`,
                                                                        token_idx: TokenIdx(
                                                                            97,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                            EntityNodePath::AssociatedItem(
                                                AssociatedItemNodePath::TypeItem(
                                                    TypeItemNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `upper_mass`,
                                                                item_kind: MemoizedField,
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
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `f32`,
                                                            token_idx: TokenIdx(
                                                                423,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
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
            (
                EntityPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `lower_mass`,
                            item_kind: MemoizedField,
                        },
                    ),
                ),
                Decl::AssociatedItem(
                    AssociatedItemDecl::TypeItem(
                        TypeItemDecl::MemoizedField(
                            TypeMemoizedFieldDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `lower_mass`,
                                    item_kind: MemoizedField,
                                },
                                memo_ty: Some(
                                    FormTypeExpr {
                                        expr: 0,
                                    },
                                ),
                                expr: None,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        EntityNodePath::ImplBlock(
                                                            ImplBlockNodePath::TypeImplBlock(
                                                                TypeImplBlockNodePath {
                                                                    path: TypeImplBlockPath {
                                                                        module_path: `mnist_classifier::connected_component`,
                                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    principal_entity_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `ConnectedComponent`,
                                                                        token_idx: TokenIdx(
                                                                            97,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                            EntityNodePath::AssociatedItem(
                                                AssociatedItemNodePath::TypeItem(
                                                    TypeItemNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `lower_mass`,
                                                                item_kind: MemoizedField,
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
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `f32`,
                                                            token_idx: TokenIdx(
                                                                435,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
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
        ],
    },
)