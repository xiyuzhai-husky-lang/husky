Ok(
    DeclSheet {
        decls: [
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::result::Result`, `Enum`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Type(
                        TypeDecl::Enum(
                            EnumTypeDecl {
                                path: TypePath(`core::result::Result`, `Enum`),
                                ast_idx: 5,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::result::Result`, `Enum`),
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
                                                            5,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `T`,
                                                                    token_idx: TokenIdx(
                                                                        4,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            7,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `E`,
                                                                    token_idx: TokenIdx(
                                                                        6,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [
                                                (
                                                    ImplicitTypeParameter,
                                                    ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                ),
                                                (
                                                    ImplicitTypeParameter,
                                                    ArenaIdxRange(
                                                        1..2,
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
                                                annotated_variance_token: None,
                                                symbol: 0,
                                                variant: ImplicitParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `T`,
                                                        token_idx: TokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                            ImplicitParameterDeclPattern {
                                                annotated_variance_token: None,
                                                symbol: 1,
                                                variant: ImplicitParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `E`,
                                                        token_idx: TokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    5,
                                                ),
                                            ),
                                        ],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RightAngleBracketToken(
                                            TokenIdx(
                                                7,
                                            ),
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
                            module_path: `core::result`,
                            trai_path: TraitPath(`core::ops::Unveil`),
                            ty_path: TypePath(`core::result::Result`, `Enum`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Decl::ImplBlock(
                        ImplBlockDecl::TraitForType(
                            TraitForTypeImplBlockDecl {
                                ast_idx: 6,
                                impl_block: TraitForTypeImplBlock {
                                    id: TraitForTypeImplBlockId {
                                        module_path: `core::result`,
                                        trai_path: TraitPath(`core::ops::Unveil`),
                                        ty_path: TypePath(`core::result::Result`, `Enum`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 6,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            18,
                                        ),
                                    },
                                    trai_expr: 47,
                                    for_token: TokenIdx(
                                        36,
                                    ),
                                    ty_expr: 48,
                                    items: Some(
                                        TraitForType(
                                            TraitForTypeItems {
                                                ast_idx_range: ArenaIdxRange(
                                                    3..5,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        18,
                                    ),
                                },
                                implicit_parameter_decl_list: Some(
                                    ImplicitParameterDeclList {
                                        langle: LeftAngleBracketOrLessThanToken(
                                            TokenIdx(
                                                19,
                                            ),
                                        ),
                                        implicit_parameters: [
                                            ImplicitParameterDeclPattern {
                                                annotated_variance_token: None,
                                                symbol: 0,
                                                variant: ImplicitParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `T`,
                                                        token_idx: TokenIdx(
                                                            20,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                            ImplicitParameterDeclPattern {
                                                annotated_variance_token: None,
                                                symbol: 1,
                                                variant: ImplicitParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `E1`,
                                                        token_idx: TokenIdx(
                                                            22,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                            ImplicitParameterDeclPattern {
                                                annotated_variance_token: None,
                                                symbol: 2,
                                                variant: ImplicitParameterDeclPatternVariant::Type {
                                                    ident_token: IdentToken {
                                                        ident: `E2`,
                                                        token_idx: TokenIdx(
                                                            24,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    21,
                                                ),
                                            ),
                                            CommaToken(
                                                TokenIdx(
                                                    23,
                                                ),
                                            ),
                                        ],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RightAngleBracketToken(
                                            TokenIdx(
                                                25,
                                            ),
                                        ),
                                    },
                                ),
                                trai_expr: TraitExpr {
                                    expr: 6,
                                },
                                for_token: ConnectionForToken {
                                    token_idx: TokenIdx(
                                        36,
                                    ),
                                },
                                ty_expr: TypeExpr {
                                    expr: 11,
                                },
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            40,
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
                                                        module_path: `core::result`,
                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                        ty_path: TypePath(`core::result::Result`, `Enum`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::ops::Unveil`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::result::Result`, `Enum`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `T`,
                                                    token_idx: TokenIdx(
                                                        33,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                            ident_token: IdentToken {
                                                                ident: `T`,
                                                                token_idx: TokenIdx(
                                                                    20,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 1,
                                                    argument: 2,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `E1`,
                                                    token_idx: TokenIdx(
                                                        34,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                            ident_token: IdentToken {
                                                                ident: `E1`,
                                                                token_idx: TokenIdx(
                                                                    22,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 3,
                                                    argument: 4,
                                                },
                                                Expr::FunctionApplicationOrCall {
                                                    function: 0,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        31,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        5..6,
                                                    ),
                                                    commas: [],
                                                    rpar_token_idx: TokenIdx(
                                                        35,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 4,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::result::Result`, `Enum`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `T`,
                                                    token_idx: TokenIdx(
                                                        38,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                            ident_token: IdentToken {
                                                                ident: `T`,
                                                                token_idx: TokenIdx(
                                                                    20,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 7,
                                                    argument: 8,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `E2`,
                                                    token_idx: TokenIdx(
                                                        39,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                            ident_token: IdentToken {
                                                                ident: `E2`,
                                                                token_idx: TokenIdx(
                                                                    24,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 9,
                                                    argument: 10,
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    path_name_token: PathNameToken::CrateRoot(
                                                        CrateToken {
                                                            token_idx: TokenIdx(
                                                                26,
                                                            ),
                                                        },
                                                    ),
                                                    entity_path: EntityPath::Module(
                                                        `core`,
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 0,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            27,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `ops`,
                                                            token_idx: TokenIdx(
                                                                28,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::Module(
                                                            `core::ops`,
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 1,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            29,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `Unveil`,
                                                            token_idx: TokenIdx(
                                                                30,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::ops::Unveil`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `Result`,
                                                            token_idx: TokenIdx(
                                                                32,
                                                            ),
                                                        },
                                                    ),
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::result::Result`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `Result`,
                                                            token_idx: TokenIdx(
                                                                37,
                                                            ),
                                                        },
                                                    ),
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::result::Result`, `Enum`),
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
                                                            21,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `T`,
                                                                    token_idx: TokenIdx(
                                                                        20,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            23,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `E1`,
                                                                    token_idx: TokenIdx(
                                                                        22,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            25,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `E2`,
                                                                    token_idx: TokenIdx(
                                                                        24,
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
                                                (
                                                    ImplicitTypeParameter,
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                                (
                                                    ImplicitTypeParameter,
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: Trait,
                                                expr_idx: 6,
                                            },
                                            ExprRoot {
                                                kind: SelfType,
                                                expr_idx: 11,
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
                                module_path: `core::result`,
                                trai_path: TraitPath(`core::ops::Unveil`),
                                ty_path: TypePath(`core::result::Result`, `Enum`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `Continue`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TraitForTypeItem(
                            TraitForTypeItemDecl::AssociatedType(
                                TraitForTypeAssociatedTypeDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::TraitForType(
                                                TraitForTypeImplBlockId {
                                                    module_path: `core::result`,
                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                    ty_path: TypePath(`core::result::Result`, `Enum`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `Continue`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TraitForTypeItem(
                                                TraitForTypeItemPath {
                                                    parent_ty: TypePath(`core::result::Result`, `Enum`),
                                                    trai: TraitPath(`core::ops::Unveil`),
                                                    ident: `Continue`,
                                                    item_kind: AssociatedType,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::TraitForType(
                                            TraitForTypeImplBlock {
                                                id: TraitForTypeImplBlockId {
                                                    module_path: `core::result`,
                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                    ty_path: TypePath(`core::result::Result`, `Enum`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 6,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        18,
                                                    ),
                                                },
                                                trai_expr: 47,
                                                for_token: TokenIdx(
                                                    36,
                                                ),
                                                ty_expr: 48,
                                                items: Some(
                                                    TraitForType(
                                                        TraitForTypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                3..5,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        ast_idx: 3,
                                        ident: `Continue`,
                                        associated_item_kind: TraitForTypeItem(
                                            AssociatedType,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `core::result`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 3,
                                    implicit_parameter_decl_list: None,
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
                                                                        module_path: `core::result`,
                                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                                        ty_path: TypePath(`core::result::Result`, `Enum`),
                                                                        disambiguator: 0,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 2,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Trait(
                                                                                TraitPath(`core::ops::Unveil`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 3,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`core::result::Result`, `Enum`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                Expr::CurrentSymbol {
                                                                    ident: `T`,
                                                                    token_idx: TokenIdx(
                                                                        33,
                                                                    ),
                                                                    current_symbol_idx: 0,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `T`,
                                                                                token_idx: TokenIdx(
                                                                                    20,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                Expr::ExplicitApplication {
                                                                    function: 1,
                                                                    argument: 2,
                                                                },
                                                                Expr::CurrentSymbol {
                                                                    ident: `E1`,
                                                                    token_idx: TokenIdx(
                                                                        34,
                                                                    ),
                                                                    current_symbol_idx: 1,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E1`,
                                                                                token_idx: TokenIdx(
                                                                                    22,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                Expr::ExplicitApplication {
                                                                    function: 3,
                                                                    argument: 4,
                                                                },
                                                                Expr::FunctionApplicationOrCall {
                                                                    function: 0,
                                                                    implicit_arguments: None,
                                                                    lpar_token_idx: TokenIdx(
                                                                        31,
                                                                    ),
                                                                    items: ArenaIdxRange(
                                                                        5..6,
                                                                    ),
                                                                    commas: [],
                                                                    rpar_token_idx: TokenIdx(
                                                                        35,
                                                                    ),
                                                                },
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 4,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`core::result::Result`, `Enum`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                Expr::CurrentSymbol {
                                                                    ident: `T`,
                                                                    token_idx: TokenIdx(
                                                                        38,
                                                                    ),
                                                                    current_symbol_idx: 0,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `T`,
                                                                                token_idx: TokenIdx(
                                                                                    20,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                Expr::ExplicitApplication {
                                                                    function: 7,
                                                                    argument: 8,
                                                                },
                                                                Expr::CurrentSymbol {
                                                                    ident: `E2`,
                                                                    token_idx: TokenIdx(
                                                                        39,
                                                                    ),
                                                                    current_symbol_idx: 2,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E2`,
                                                                                token_idx: TokenIdx(
                                                                                    24,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                Expr::ExplicitApplication {
                                                                    function: 9,
                                                                    argument: 10,
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::CrateRoot(
                                                                        CrateToken {
                                                                            token_idx: TokenIdx(
                                                                                26,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    entity_path: EntityPath::Module(
                                                                        `core`,
                                                                    ),
                                                                },
                                                                EntityPathExpr::Subentity {
                                                                    parent: 0,
                                                                    scope_resolution_token: ScopeResolutionToken(
                                                                        TokenIdx(
                                                                            27,
                                                                        ),
                                                                    ),
                                                                    ident_token: Ok(
                                                                        IdentToken {
                                                                            ident: `ops`,
                                                                            token_idx: TokenIdx(
                                                                                28,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    path: Ok(
                                                                        EntityPath::Module(
                                                                            `core::ops`,
                                                                        ),
                                                                    ),
                                                                },
                                                                EntityPathExpr::Subentity {
                                                                    parent: 1,
                                                                    scope_resolution_token: ScopeResolutionToken(
                                                                        TokenIdx(
                                                                            29,
                                                                        ),
                                                                    ),
                                                                    ident_token: Ok(
                                                                        IdentToken {
                                                                            ident: `Unveil`,
                                                                            token_idx: TokenIdx(
                                                                                30,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    path: Ok(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Trait(
                                                                                TraitPath(`core::ops::Unveil`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                EntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `Result`,
                                                                            token_idx: TokenIdx(
                                                                                32,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::result::Result`, `Enum`),
                                                                        ),
                                                                    ),
                                                                },
                                                                EntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `Result`,
                                                                            token_idx: TokenIdx(
                                                                                37,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::result::Result`, `Enum`),
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
                                                                            21,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `T`,
                                                                                    token_idx: TokenIdx(
                                                                                        20,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            23,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `E1`,
                                                                                    token_idx: TokenIdx(
                                                                                        22,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            25,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `E2`,
                                                                                    token_idx: TokenIdx(
                                                                                        24,
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
                                                                (
                                                                    ImplicitTypeParameter,
                                                                    ArenaIdxRange(
                                                                        1..2,
                                                                    ),
                                                                ),
                                                                (
                                                                    ImplicitTypeParameter,
                                                                    ArenaIdxRange(
                                                                        2..3,
                                                                    ),
                                                                ),
                                                            ],
                                                        },
                                                        roots: [
                                                            ExprRoot {
                                                                kind: Trait,
                                                                expr_idx: 6,
                                                            },
                                                            ExprRoot {
                                                                kind: SelfType,
                                                                expr_idx: 11,
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
                                                                module_path: `core::result`,
                                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                                ty_path: TypePath(`core::result::Result`, `Enum`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `Continue`,
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
                                                                    ident: `T`,
                                                                },
                                                            ),
                                                        },
                                                        InheritedSymbol {
                                                            parent_symbol_idx: Current(
                                                                1,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSymbol::Type {
                                                                    ident: `E1`,
                                                                },
                                                            ),
                                                        },
                                                        InheritedSymbol {
                                                            parent_symbol_idx: Current(
                                                                2,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSymbol::Type {
                                                                    ident: `E2`,
                                                                },
                                                            ),
                                                        },
                                                    ],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [],
                                            },
                                            roots: [],
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
                        impl_block_id: ImplBlockId::TraitForType(
                            TraitForTypeImplBlockId {
                                module_path: `core::result`,
                                trai_path: TraitPath(`core::ops::Unveil`),
                                ty_path: TypePath(`core::result::Result`, `Enum`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `branch`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TraitForTypeItem(
                            TraitForTypeItemDecl::MethodFn(
                                TraitForTypeMethodFnDecl {
                                    path: TraitForTypeItemPath {
                                        parent_ty: TypePath(`core::result::Result`, `Enum`),
                                        trai: TraitPath(`core::ops::Unveil`),
                                        ident: `branch`,
                                        item_kind: MethodFn,
                                    },
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::TraitForType(
                                                TraitForTypeImplBlockId {
                                                    module_path: `core::result`,
                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                    ty_path: TypePath(`core::result::Result`, `Enum`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `branch`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TraitForTypeItem(
                                                TraitForTypeItemPath {
                                                    parent_ty: TypePath(`core::result::Result`, `Enum`),
                                                    trai: TraitPath(`core::ops::Unveil`),
                                                    ident: `branch`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::TraitForType(
                                            TraitForTypeImplBlock {
                                                id: TraitForTypeImplBlockId {
                                                    module_path: `core::result`,
                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                    ty_path: TypePath(`core::result::Result`, `Enum`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 6,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        18,
                                                    ),
                                                },
                                                trai_expr: 47,
                                                for_token: TokenIdx(
                                                    36,
                                                ),
                                                ty_expr: 48,
                                                items: Some(
                                                    TraitForType(
                                                        TraitForTypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                3..5,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        ast_idx: 4,
                                        ident: `branch`,
                                        associated_item_kind: TraitForTypeItem(
                                            MethodFn,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `core::result`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 4,
                                    implicit_parameter_decl_list: None,
                                    explicit_parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                47,
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
                                                        49,
                                                    ),
                                                ),
                                                ty: 4,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                53,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                54,
                                            ),
                                        ),
                                    ),
                                    return_ty: Some(
                                        ReturnTypeExpr {
                                            expr: 9,
                                        },
                                    ),
                                    eol_colon: EolToken::Colon(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                58,
                                            ),
                                        },
                                    ),
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
                                                                        module_path: `core::result`,
                                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                                        ty_path: TypePath(`core::result::Result`, `Enum`),
                                                                        disambiguator: 0,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 2,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Trait(
                                                                                TraitPath(`core::ops::Unveil`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 3,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`core::result::Result`, `Enum`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                Expr::CurrentSymbol {
                                                                    ident: `T`,
                                                                    token_idx: TokenIdx(
                                                                        33,
                                                                    ),
                                                                    current_symbol_idx: 0,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `T`,
                                                                                token_idx: TokenIdx(
                                                                                    20,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                Expr::ExplicitApplication {
                                                                    function: 1,
                                                                    argument: 2,
                                                                },
                                                                Expr::CurrentSymbol {
                                                                    ident: `E1`,
                                                                    token_idx: TokenIdx(
                                                                        34,
                                                                    ),
                                                                    current_symbol_idx: 1,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E1`,
                                                                                token_idx: TokenIdx(
                                                                                    22,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                Expr::ExplicitApplication {
                                                                    function: 3,
                                                                    argument: 4,
                                                                },
                                                                Expr::FunctionApplicationOrCall {
                                                                    function: 0,
                                                                    implicit_arguments: None,
                                                                    lpar_token_idx: TokenIdx(
                                                                        31,
                                                                    ),
                                                                    items: ArenaIdxRange(
                                                                        5..6,
                                                                    ),
                                                                    commas: [],
                                                                    rpar_token_idx: TokenIdx(
                                                                        35,
                                                                    ),
                                                                },
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 4,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`core::result::Result`, `Enum`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                Expr::CurrentSymbol {
                                                                    ident: `T`,
                                                                    token_idx: TokenIdx(
                                                                        38,
                                                                    ),
                                                                    current_symbol_idx: 0,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `T`,
                                                                                token_idx: TokenIdx(
                                                                                    20,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                Expr::ExplicitApplication {
                                                                    function: 7,
                                                                    argument: 8,
                                                                },
                                                                Expr::CurrentSymbol {
                                                                    ident: `E2`,
                                                                    token_idx: TokenIdx(
                                                                        39,
                                                                    ),
                                                                    current_symbol_idx: 2,
                                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `E2`,
                                                                                token_idx: TokenIdx(
                                                                                    24,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                Expr::ExplicitApplication {
                                                                    function: 9,
                                                                    argument: 10,
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::CrateRoot(
                                                                        CrateToken {
                                                                            token_idx: TokenIdx(
                                                                                26,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    entity_path: EntityPath::Module(
                                                                        `core`,
                                                                    ),
                                                                },
                                                                EntityPathExpr::Subentity {
                                                                    parent: 0,
                                                                    scope_resolution_token: ScopeResolutionToken(
                                                                        TokenIdx(
                                                                            27,
                                                                        ),
                                                                    ),
                                                                    ident_token: Ok(
                                                                        IdentToken {
                                                                            ident: `ops`,
                                                                            token_idx: TokenIdx(
                                                                                28,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    path: Ok(
                                                                        EntityPath::Module(
                                                                            `core::ops`,
                                                                        ),
                                                                    ),
                                                                },
                                                                EntityPathExpr::Subentity {
                                                                    parent: 1,
                                                                    scope_resolution_token: ScopeResolutionToken(
                                                                        TokenIdx(
                                                                            29,
                                                                        ),
                                                                    ),
                                                                    ident_token: Ok(
                                                                        IdentToken {
                                                                            ident: `Unveil`,
                                                                            token_idx: TokenIdx(
                                                                                30,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    path: Ok(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Trait(
                                                                                TraitPath(`core::ops::Unveil`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                EntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `Result`,
                                                                            token_idx: TokenIdx(
                                                                                32,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::result::Result`, `Enum`),
                                                                        ),
                                                                    ),
                                                                },
                                                                EntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `Result`,
                                                                            token_idx: TokenIdx(
                                                                                37,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::result::Result`, `Enum`),
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
                                                                            21,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `T`,
                                                                                    token_idx: TokenIdx(
                                                                                        20,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            23,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `E1`,
                                                                                    token_idx: TokenIdx(
                                                                                        22,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            25,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `E2`,
                                                                                    token_idx: TokenIdx(
                                                                                        24,
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
                                                                (
                                                                    ImplicitTypeParameter,
                                                                    ArenaIdxRange(
                                                                        1..2,
                                                                    ),
                                                                ),
                                                                (
                                                                    ImplicitTypeParameter,
                                                                    ArenaIdxRange(
                                                                        2..3,
                                                                    ),
                                                                ),
                                                            ],
                                                        },
                                                        roots: [
                                                            ExprRoot {
                                                                kind: Trait,
                                                                expr_idx: 6,
                                                            },
                                                            ExprRoot {
                                                                kind: SelfType,
                                                                expr_idx: 11,
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
                                                                module_path: `core::result`,
                                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                                ty_path: TypePath(`core::result::Result`, `Enum`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `branch`,
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
                                                                    TypePath(`core::result::Result`, `Enum`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::InheritedSymbol {
                                                        ident: `T`,
                                                        token_idx: TokenIdx(
                                                            51,
                                                        ),
                                                        inherited_symbol_idx: 0,
                                                        inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSymbol::Type {
                                                                ident: `T`,
                                                            },
                                                        ),
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function: 0,
                                                        argument: 1,
                                                    },
                                                    Expr::InheritedSymbol {
                                                        ident: `E1`,
                                                        token_idx: TokenIdx(
                                                            52,
                                                        ),
                                                        inherited_symbol_idx: 1,
                                                        inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSymbol::Type {
                                                                ident: `E1`,
                                                            },
                                                        ),
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function: 2,
                                                        argument: 3,
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::result::Result`, `Enum`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::InheritedSymbol {
                                                        ident: `T`,
                                                        token_idx: TokenIdx(
                                                            56,
                                                        ),
                                                        inherited_symbol_idx: 0,
                                                        inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSymbol::Type {
                                                                ident: `T`,
                                                            },
                                                        ),
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function: 5,
                                                        argument: 6,
                                                    },
                                                    Expr::InheritedSymbol {
                                                        ident: `E2`,
                                                        token_idx: TokenIdx(
                                                            57,
                                                        ),
                                                        inherited_symbol_idx: 2,
                                                        inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(
                                                            InheritedImplicitParameterSymbol::Type {
                                                                ident: `E2`,
                                                            },
                                                        ),
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function: 7,
                                                        argument: 8,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `Result`,
                                                                token_idx: TokenIdx(
                                                                    50,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::result::Result`, `Enum`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `Result`,
                                                                token_idx: TokenIdx(
                                                                    55,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::result::Result`, `Enum`),
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
                                                                ident: `result`,
                                                                token_idx: TokenIdx(
                                                                    48,
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
                                                            `result`,
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
                                                                    ident: `T`,
                                                                },
                                                            ),
                                                        },
                                                        InheritedSymbol {
                                                            parent_symbol_idx: Current(
                                                                1,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSymbol::Type {
                                                                    ident: `E1`,
                                                                },
                                                            ),
                                                        },
                                                        InheritedSymbol {
                                                            parent_symbol_idx: Current(
                                                                2,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSymbol::Type {
                                                                    ident: `E2`,
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
                                                                49,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `result`,
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
                                                            ty: 4,
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
                                                    expr_idx: 4,
                                                },
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 9,
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
        ],
    },
)