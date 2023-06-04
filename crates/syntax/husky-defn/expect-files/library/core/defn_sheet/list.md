Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::list::List`, `Extern`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Type(
                        TypeDefn::Extern(
                            ExternTypeDefn {
                                path: TypePath(`core::list::List`, `Extern`),
                                decl: ExternTypeDecl {
                                    path: TypePath(`core::list::List`, `Extern`),
                                    ast_idx: 2,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::list::List`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
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
                                                    data: [
                                                        CurrentSymbol {
                                                            modifier: Const,
                                                            access_start: TokenIdx(
                                                                6,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `E`,
                                                                        token_idx: TokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    (
                                                        ImplicitTypeParameter,
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [],
                                        },
                                    },
                                    implicit_parameter_decl_list: Some(
                                        ImplicitParameterDeclList {
                                            langle: LeftAngleBracketOrLessThanToken(
                                                TokenIdx(
                                                    3,
                                                ),
                                            ),
                                            implicit_parameters: [
                                                ImplicitParameterDeclPattern {
                                                    annotated_variance_token: Some(
                                                        VarianceToken::Covariant(
                                                            CovariantToken {
                                                                token_idx: TokenIdx(
                                                                    4,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    symbol: 0,
                                                    variant: ImplicitParameterDeclPatternVariant::Type {
                                                        ident_token: IdentToken {
                                                            ident: `E`,
                                                            token_idx: TokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                        traits: None,
                                                    },
                                                },
                                            ],
                                            commas: [],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rangle: RightAngleBracketToken(
                                                TokenIdx(
                                                    6,
                                                ),
                                            ),
                                        },
                                    ),
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Impl(
                    ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `core::list`,
                            ty_path: TypePath(`core::list::List`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Defn::ImplBlock(
                        ImplBlockDecl::Type(
                            TypeImplBlockDecl {
                                ast_idx: 3,
                                impl_block: TypeImplBlock {
                                    id: TypeImplBlockId {
                                        module_path: `core::list`,
                                        ty_path: TypePath(`core::list::List`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 3,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            8,
                                        ),
                                    },
                                    ty_expr: 0,
                                    body: Type(
                                        TypeItems {
                                            ast_idx_range: ArenaIdxRange(
                                                0..2,
                                            ),
                                        },
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        8,
                                    ),
                                },
                                implicit_parameter_decl_list: Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                9,
                                            ),
                                        ),
                                        implicit_parameters: [
                                            ImplicitParameterDeclPattern {
                                                annotated_variance_token: None,
                                                symbol: 0,
                                                variant: ImplicitParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `E`,
                                                        token_idx: TokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RightAngleBracketToken(
                                            TokenIdx(
                                                11,
                                            ),
                                        ),
                                    },
                                ),
                                ty_expr: TypeExpr {
                                    expr: 2,
                                },
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            14,
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
                                                        module_path: `core::list`,
                                                        ty_path: TypePath(`core::list::List`, `Extern`),
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
                                                                TypePath(`core::list::List`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `E`,
                                                    token_idx: TokenIdx(
                                                        13,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                            ident_token: IdentToken {
                                                                ident: `E`,
                                                                token_idx: TokenIdx(
                                                                    10,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 0,
                                                    argument: 1,
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        12,
                                                    ),
                                                    ident: `List`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::list::List`, `Extern`),
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
                                                data: [
                                                    CurrentSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            11,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `E`,
                                                                    token_idx: TokenIdx(
                                                                        10,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                (
                                                    ImplicitTypeParameter,
                                                    ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: SelfType,
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
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `core::list`,
                                ty_path: TypePath(`core::list::List`, `Extern`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `ilen`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::MethodFn(
                                TypeMethodFnDefn {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId::Type(
                                            TypeImplBlockId {
                                                module_path: `core::list`,
                                                ty_path: TypePath(`core::list::List`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `ilen`,
                                    },
                                    decl: TypeMethodFnDecl {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `core::list`,
                                                    ty_path: TypePath(`core::list::List`, `Extern`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `ilen`,
                                        },
                                        path: Some(
                                            TypeItemPath {
                                                parent_ty: TypePath(`core::list::List`, `Extern`),
                                                ident: `ilen`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `core::list`,
                                                        ty_path: TypePath(`core::list::List`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `ilen`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        parent_ty: TypePath(`core::list::List`, `Extern`),
                                                        ident: `ilen`,
                                                        item_kind: MethodFn,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module_path: `core::list`,
                                                        ty_path: TypePath(`core::list::List`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 3,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                    ty_expr: 0,
                                                    body: Type(
                                                        TypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                0..2,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            ast_idx: 0,
                                            ident: `ilen`,
                                            associated_item_kind: TypeItem(
                                                MethodFn,
                                            ),
                                            visibility: Scope::Pub,
                                            is_generic: false,
                                        },
                                        ast_idx: 0,
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
                                                                            module_path: `core::list`,
                                                                            ty_path: TypePath(`core::list::List`, `Extern`),
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
                                                                                    TypePath(`core::list::List`, `Extern`),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    Expr::CurrentSymbol {
                                                                        ident: `E`,
                                                                        token_idx: TokenIdx(
                                                                            13,
                                                                        ),
                                                                        current_symbol_idx: 0,
                                                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                            implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `E`,
                                                                                    token_idx: TokenIdx(
                                                                                        10,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    Expr::ExplicitApplication {
                                                                        function: 0,
                                                                        argument: 1,
                                                                    },
                                                                ],
                                                            },
                                                            entity_path_expr_arena: Arena {
                                                                data: [
                                                                    EntityPathExpr::Root {
                                                                        token_idx: TokenIdx(
                                                                            12,
                                                                        ),
                                                                        ident: `List`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`core::list::List`, `Extern`),
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
                                                                    data: [
                                                                        CurrentSymbol {
                                                                            modifier: Const,
                                                                            access_start: TokenIdx(
                                                                                11,
                                                                            ),
                                                                            access_end: None,
                                                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                    ident_token: IdentToken {
                                                                                        ident: `E`,
                                                                                        token_idx: TokenIdx(
                                                                                            10,
                                                                                        ),
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                                allow_self_type: True,
                                                                allow_self_value: False,
                                                                pattern_ty_constraints: [
                                                                    (
                                                                        ImplicitTypeParameter,
                                                                        ArenaIdxRange(
                                                                            0..1,
                                                                        ),
                                                                    ),
                                                                ],
                                                            },
                                                            roots: [
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr_idx: 2,
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
                                                                    module_path: `core::list`,
                                                                    ty_path: TypePath(`core::list::List`, `Extern`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `ilen`,
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
                                                                        TypePath(`core::num::i32`, `Extern`),
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
                                                                21,
                                                            ),
                                                            ident: `i32`,
                                                            entity_path: EntityPath::ModuleItem(
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
                                                        data: [
                                                            InheritedSymbol {
                                                                parent_symbol_idx: Current(
                                                                    0,
                                                                ),
                                                                modifier: Const,
                                                                kind: InheritedSymbolKind::ImplicitParameter(
                                                                    InheritedImplicitParameterSymbol::Type {
                                                                        ident: `E`,
                                                                    },
                                                                ),
                                                            },
                                                        ],
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
                                                    18,
                                                ),
                                            ),
                                            self_parameter: None,
                                            comma_after_self_parameter: None,
                                            regular_parameters: [],
                                            commas: [],
                                            rpar: RightParenthesisToken(
                                                TokenIdx(
                                                    19,
                                                ),
                                            ),
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    20,
                                                ),
                                            ),
                                        ),
                                        return_ty: Some(
                                            ReturnTypeExpr {
                                                expr: 0,
                                            },
                                        ),
                                        eol_colon: EolToken::Semicolon(
                                            EolSemicolonToken {
                                                token_idx: TokenIdx(
                                                    22,
                                                ),
                                            },
                                        ),
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::Type(
                                                                                TypeImplBlockId {
                                                                                    module_path: `core::list`,
                                                                                    ty_path: TypePath(`core::list::List`, `Extern`),
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
                                                                                            TypePath(`core::list::List`, `Extern`),
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            Expr::CurrentSymbol {
                                                                                ident: `E`,
                                                                                token_idx: TokenIdx(
                                                                                    13,
                                                                                ),
                                                                                current_symbol_idx: 0,
                                                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                                        ident_token: IdentToken {
                                                                                            ident: `E`,
                                                                                            token_idx: TokenIdx(
                                                                                                10,
                                                                                            ),
                                                                                        },
                                                                                    },
                                                                                },
                                                                            },
                                                                            Expr::ExplicitApplication {
                                                                                function: 0,
                                                                                argument: 1,
                                                                            },
                                                                        ],
                                                                    },
                                                                    entity_path_expr_arena: Arena {
                                                                        data: [
                                                                            EntityPathExpr::Root {
                                                                                token_idx: TokenIdx(
                                                                                    12,
                                                                                ),
                                                                                ident: `List`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`core::list::List`, `Extern`),
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
                                                                            data: [
                                                                                CurrentSymbol {
                                                                                    modifier: Const,
                                                                                    access_start: TokenIdx(
                                                                                        11,
                                                                                    ),
                                                                                    access_end: None,
                                                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                            ident_token: IdentToken {
                                                                                                ident: `E`,
                                                                                                token_idx: TokenIdx(
                                                                                                    10,
                                                                                                ),
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                        allow_self_type: True,
                                                                        allow_self_value: False,
                                                                        pattern_ty_constraints: [
                                                                            (
                                                                                ImplicitTypeParameter,
                                                                                ArenaIdxRange(
                                                                                    0..1,
                                                                                ),
                                                                            ),
                                                                        ],
                                                                    },
                                                                    roots: [
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr_idx: 2,
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
                                                                            module_path: `core::list`,
                                                                            ty_path: TypePath(`core::list::List`, `Extern`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `ilen`,
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
                                                                                TypePath(`core::num::i32`, `Extern`),
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
                                                                        21,
                                                                    ),
                                                                    ident: `i32`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                                data: [
                                                                    InheritedSymbol {
                                                                        parent_symbol_idx: Current(
                                                                            0,
                                                                        ),
                                                                        modifier: Const,
                                                                        kind: InheritedSymbolKind::ImplicitParameter(
                                                                            InheritedImplicitParameterSymbol::Type {
                                                                                ident: `E`,
                                                                            },
                                                                        ),
                                                                    },
                                                                ],
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
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `core::list`,
                                                                ty_path: TypePath(`core::list::List`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `ilen`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
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
                                                    data: [
                                                        InheritedSymbol {
                                                            parent_symbol_idx: Current(
                                                                0,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSymbol::Type {
                                                                    ident: `E`,
                                                                },
                                                            ),
                                                        },
                                                    ],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [],
                                            },
                                            roots: [],
                                        },
                                    },
                                    body: None,
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `core::list`,
                                ty_path: TypePath(`core::list::List`, `Extern`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `push`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::MethodFn(
                                TypeMethodFnDefn {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId::Type(
                                            TypeImplBlockId {
                                                module_path: `core::list`,
                                                ty_path: TypePath(`core::list::List`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `push`,
                                    },
                                    decl: TypeMethodFnDecl {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `core::list`,
                                                    ty_path: TypePath(`core::list::List`, `Extern`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `push`,
                                        },
                                        path: Some(
                                            TypeItemPath {
                                                parent_ty: TypePath(`core::list::List`, `Extern`),
                                                ident: `push`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `core::list`,
                                                        ty_path: TypePath(`core::list::List`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `push`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        parent_ty: TypePath(`core::list::List`, `Extern`),
                                                        ident: `push`,
                                                        item_kind: MethodFn,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module_path: `core::list`,
                                                        ty_path: TypePath(`core::list::List`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 3,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                    ty_expr: 0,
                                                    body: Type(
                                                        TypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                0..2,
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                            ast_idx: 1,
                                            ident: `push`,
                                            associated_item_kind: TypeItem(
                                                MethodFn,
                                            ),
                                            visibility: Scope::Pub,
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
                                                                    ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `core::list`,
                                                                            ty_path: TypePath(`core::list::List`, `Extern`),
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
                                                                                    TypePath(`core::list::List`, `Extern`),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    Expr::CurrentSymbol {
                                                                        ident: `E`,
                                                                        token_idx: TokenIdx(
                                                                            13,
                                                                        ),
                                                                        current_symbol_idx: 0,
                                                                        current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                            implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `E`,
                                                                                    token_idx: TokenIdx(
                                                                                        10,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    Expr::ExplicitApplication {
                                                                        function: 0,
                                                                        argument: 1,
                                                                    },
                                                                ],
                                                            },
                                                            entity_path_expr_arena: Arena {
                                                                data: [
                                                                    EntityPathExpr::Root {
                                                                        token_idx: TokenIdx(
                                                                            12,
                                                                        ),
                                                                        ident: `List`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`core::list::List`, `Extern`),
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
                                                                    data: [
                                                                        CurrentSymbol {
                                                                            modifier: Const,
                                                                            access_start: TokenIdx(
                                                                                11,
                                                                            ),
                                                                            access_end: None,
                                                                            variant: CurrentSymbolVariant::ImplicitParameter {
                                                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                    ident_token: IdentToken {
                                                                                        ident: `E`,
                                                                                        token_idx: TokenIdx(
                                                                                            10,
                                                                                        ),
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                                allow_self_type: True,
                                                                allow_self_value: False,
                                                                pattern_ty_constraints: [
                                                                    (
                                                                        ImplicitTypeParameter,
                                                                        ArenaIdxRange(
                                                                            0..1,
                                                                        ),
                                                                    ),
                                                                ],
                                                            },
                                                            roots: [
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr_idx: 2,
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
                                                                    module_path: `core::list`,
                                                                    ty_path: TypePath(`core::list::List`, `Extern`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `push`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::InheritedSymbol {
                                                            ident: `E`,
                                                            token_idx: TokenIdx(
                                                                32,
                                                            ),
                                                            inherited_symbol_idx: 0,
                                                            inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSymbol::Type {
                                                                    ident: `E`,
                                                                },
                                                            ),
                                                        },
                                                    ],
                                                },
                                                entity_path_expr_arena: Arena {
                                                    data: [],
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
                                                                    ident: `e`,
                                                                    token_idx: TokenIdx(
                                                                        30,
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
                                                                `e`,
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
                                                        data: [
                                                            InheritedSymbol {
                                                                parent_symbol_idx: Current(
                                                                    0,
                                                                ),
                                                                modifier: Const,
                                                                kind: InheritedSymbolKind::ImplicitParameter(
                                                                    InheritedImplicitParameterSymbol::Type {
                                                                        ident: `E`,
                                                                    },
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    current_symbol_arena: Arena {
                                                        data: [
                                                            CurrentSymbol {
                                                                modifier: Pure,
                                                                access_start: TokenIdx(
                                                                    31,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::ExplicitParameter {
                                                                    ident: `e`,
                                                                    pattern_symbol_idx: 0,
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: True,
                                                    pattern_ty_constraints: [
                                                        (
                                                            ExplicitParameter {
                                                                pattern_expr: 0,
                                                                ty: 0,
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
                                                        expr_idx: 0,
                                                    },
                                                ],
                                            },
                                        },
                                        implicit_parameter_decl_list: None,
                                        explicit_parameter_decl_list: ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    26,
                                                ),
                                            ),
                                            self_parameter: Some(
                                                SelfParameterDeclPattern::Mut {
                                                    mut_token: MutToken {
                                                        token_idx: TokenIdx(
                                                            27,
                                                        ),
                                                    },
                                                    self_value_token: SelfValueToken {
                                                        token_idx: TokenIdx(
                                                            28,
                                                        ),
                                                    },
                                                },
                                            ),
                                            comma_after_self_parameter: Some(
                                                CommaToken(
                                                    TokenIdx(
                                                        29,
                                                    ),
                                                ),
                                            ),
                                            regular_parameters: [
                                                RegularParameterDeclPattern {
                                                    pattern: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                    colon: ColonToken(
                                                        TokenIdx(
                                                            31,
                                                        ),
                                                    ),
                                                    ty: 0,
                                                },
                                            ],
                                            commas: [],
                                            rpar: RightParenthesisToken(
                                                TokenIdx(
                                                    33,
                                                ),
                                            ),
                                        },
                                        curry_token: None,
                                        return_ty: None,
                                        eol_colon: EolToken::Semicolon(
                                            EolSemicolonToken {
                                                token_idx: TokenIdx(
                                                    34,
                                                ),
                                            },
                                        ),
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::Type(
                                                                                TypeImplBlockId {
                                                                                    module_path: `core::list`,
                                                                                    ty_path: TypePath(`core::list::List`, `Extern`),
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
                                                                                            TypePath(`core::list::List`, `Extern`),
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            Expr::CurrentSymbol {
                                                                                ident: `E`,
                                                                                token_idx: TokenIdx(
                                                                                    13,
                                                                                ),
                                                                                current_symbol_idx: 0,
                                                                                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                                        ident_token: IdentToken {
                                                                                            ident: `E`,
                                                                                            token_idx: TokenIdx(
                                                                                                10,
                                                                                            ),
                                                                                        },
                                                                                    },
                                                                                },
                                                                            },
                                                                            Expr::ExplicitApplication {
                                                                                function: 0,
                                                                                argument: 1,
                                                                            },
                                                                        ],
                                                                    },
                                                                    entity_path_expr_arena: Arena {
                                                                        data: [
                                                                            EntityPathExpr::Root {
                                                                                token_idx: TokenIdx(
                                                                                    12,
                                                                                ),
                                                                                ident: `List`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`core::list::List`, `Extern`),
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
                                                                            data: [
                                                                                CurrentSymbol {
                                                                                    modifier: Const,
                                                                                    access_start: TokenIdx(
                                                                                        11,
                                                                                    ),
                                                                                    access_end: None,
                                                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                            ident_token: IdentToken {
                                                                                                ident: `E`,
                                                                                                token_idx: TokenIdx(
                                                                                                    10,
                                                                                                ),
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                        allow_self_type: True,
                                                                        allow_self_value: False,
                                                                        pattern_ty_constraints: [
                                                                            (
                                                                                ImplicitTypeParameter,
                                                                                ArenaIdxRange(
                                                                                    0..1,
                                                                                ),
                                                                            ),
                                                                        ],
                                                                    },
                                                                    roots: [
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr_idx: 2,
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
                                                                            module_path: `core::list`,
                                                                            ty_path: TypePath(`core::list::List`, `Extern`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `push`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::InheritedSymbol {
                                                                    ident: `E`,
                                                                    token_idx: TokenIdx(
                                                                        32,
                                                                    ),
                                                                    inherited_symbol_idx: 0,
                                                                    inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(
                                                                        InheritedImplicitParameterSymbol::Type {
                                                                            ident: `E`,
                                                                        },
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [],
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
                                                                            ident: `e`,
                                                                            token_idx: TokenIdx(
                                                                                30,
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
                                                                        `e`,
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
                                                                data: [
                                                                    InheritedSymbol {
                                                                        parent_symbol_idx: Current(
                                                                            0,
                                                                        ),
                                                                        modifier: Const,
                                                                        kind: InheritedSymbolKind::ImplicitParameter(
                                                                            InheritedImplicitParameterSymbol::Type {
                                                                                ident: `E`,
                                                                            },
                                                                        ),
                                                                    },
                                                                ],
                                                            },
                                                            current_symbol_arena: Arena {
                                                                data: [
                                                                    CurrentSymbol {
                                                                        modifier: Pure,
                                                                        access_start: TokenIdx(
                                                                            31,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                                            ident: `e`,
                                                                            pattern_symbol_idx: 0,
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                            allow_self_type: True,
                                                            allow_self_value: True,
                                                            pattern_ty_constraints: [
                                                                (
                                                                    ExplicitParameter {
                                                                        pattern_expr: 0,
                                                                        ty: 0,
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
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `core::list`,
                                                                ty_path: TypePath(`core::list::List`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `push`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
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
                                                    data: [
                                                        InheritedSymbol {
                                                            parent_symbol_idx: Current(
                                                                0,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSymbol::Type {
                                                                    ident: `E`,
                                                                },
                                                            ),
                                                        },
                                                        InheritedSymbol {
                                                            parent_symbol_idx: Current(
                                                                0,
                                                            ),
                                                            modifier: Pure,
                                                            kind: InheritedSymbolKind::ExplicitParameter {
                                                                ident: `e`,
                                                            },
                                                        },
                                                    ],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [],
                                            },
                                            roots: [],
                                        },
                                    },
                                    body: None,
                                },
                            ),
                        ),
                    ),
                ),
            ),
        ],
    },
)