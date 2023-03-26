Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::marker::Copy`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Trait(
                        TraitDefn {
                            path: TraitPath(`core::marker::Copy`),
                            decl: TraitDecl {
                                path: TraitPath(`core::marker::Copy`),
                                ast_idx: 0,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Trait(
                                                        TraitPath(`core::marker::Copy`),
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
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [],
                                    },
                                },
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                            },
                        },
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::marker::Sized`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Trait(
                        TraitDefn {
                            path: TraitPath(`core::marker::Sized`),
                            decl: TraitDecl {
                                path: TraitPath(`core::marker::Sized`),
                                ast_idx: 7,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Trait(
                                                        TraitPath(`core::marker::Sized`),
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
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [],
                                    },
                                },
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                            },
                        },
                    ),
                ),
            ),
            (
                DefnRegionPath::Impl(
                    ImplBlockId::TypeAsTrait(
                        TraitForTypeImplBlockId {
                            module_path: `core::marker`,
                            trai_path: TraitPath(`core::marker::Copy`),
                            ty_path: TypePath(`core::num::i8`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Defn::Impl(
                        ImplBlockDecl::TraitForType(
                            TraitForTypeImplBlockDecl {
                                ast_idx: 1,
                                impl_block: TraitForTypeImplBlock {
                                    id: TraitForTypeImplBlockId {
                                        module_path: `core::marker`,
                                        trai_path: TraitPath(`core::marker::Copy`),
                                        ty_path: TypePath(`core::num::i8`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 1,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            4,
                                        ),
                                    },
                                    trai_expr: 54,
                                    for_token: TokenIdx(
                                        6,
                                    ),
                                    ty_expr: 55,
                                    body: ArenaIdxRange(
                                        0..0,
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        4,
                                    ),
                                },
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                trai_expr: TraitExpr {
                                    expr: 0,
                                },
                                for_token: ConnectionForToken {
                                    token_idx: TokenIdx(
                                        6,
                                    ),
                                },
                                ty_expr: TypeExpr {
                                    expr: 1,
                                },
                                eol_colon: Err(
                                    DeclExprError::Original(
                                        OriginalDeclExprError::ExpectEolColon(
                                            TokenIdx(
                                                8,
                                            ),
                                        ),
                                    ),
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId::TypeAsTrait(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `core::marker`,
                                                        trai_path: TraitPath(`core::marker::Copy`),
                                                        ty_path: TypePath(`core::num::i8`, `Extern`),
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
                                                                TraitPath(`core::marker::Copy`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i8`, `Extern`),
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
                                                        5,
                                                    ),
                                                    ident: `Copy`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::marker::Copy`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        7,
                                                    ),
                                                    ident: `i8`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i8`, `Extern`),
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
                DefnRegionPath::Impl(
                    ImplBlockId::TypeAsTrait(
                        TraitForTypeImplBlockId {
                            module_path: `core::marker`,
                            trai_path: TraitPath(`core::marker::Copy`),
                            ty_path: TypePath(`core::num::i16`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Defn::Impl(
                        ImplBlockDecl::TraitForType(
                            TraitForTypeImplBlockDecl {
                                ast_idx: 2,
                                impl_block: TraitForTypeImplBlock {
                                    id: TraitForTypeImplBlockId {
                                        module_path: `core::marker`,
                                        trai_path: TraitPath(`core::marker::Copy`),
                                        ty_path: TypePath(`core::num::i16`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 2,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            8,
                                        ),
                                    },
                                    trai_expr: 56,
                                    for_token: TokenIdx(
                                        10,
                                    ),
                                    ty_expr: 57,
                                    body: ArenaIdxRange(
                                        0..0,
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        8,
                                    ),
                                },
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                trai_expr: TraitExpr {
                                    expr: 0,
                                },
                                for_token: ConnectionForToken {
                                    token_idx: TokenIdx(
                                        10,
                                    ),
                                },
                                ty_expr: TypeExpr {
                                    expr: 1,
                                },
                                eol_colon: Err(
                                    DeclExprError::Original(
                                        OriginalDeclExprError::ExpectEolColon(
                                            TokenIdx(
                                                12,
                                            ),
                                        ),
                                    ),
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId::TypeAsTrait(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `core::marker`,
                                                        trai_path: TraitPath(`core::marker::Copy`),
                                                        ty_path: TypePath(`core::num::i16`, `Extern`),
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
                                                                TraitPath(`core::marker::Copy`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i16`, `Extern`),
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
                                                        9,
                                                    ),
                                                    ident: `Copy`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::marker::Copy`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        11,
                                                    ),
                                                    ident: `i16`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i16`, `Extern`),
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
                DefnRegionPath::Impl(
                    ImplBlockId::TypeAsTrait(
                        TraitForTypeImplBlockId {
                            module_path: `core::marker`,
                            trai_path: TraitPath(`core::marker::Copy`),
                            ty_path: TypePath(`core::num::i32`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Defn::Impl(
                        ImplBlockDecl::TraitForType(
                            TraitForTypeImplBlockDecl {
                                ast_idx: 3,
                                impl_block: TraitForTypeImplBlock {
                                    id: TraitForTypeImplBlockId {
                                        module_path: `core::marker`,
                                        trai_path: TraitPath(`core::marker::Copy`),
                                        ty_path: TypePath(`core::num::i32`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 3,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            12,
                                        ),
                                    },
                                    trai_expr: 58,
                                    for_token: TokenIdx(
                                        14,
                                    ),
                                    ty_expr: 59,
                                    body: ArenaIdxRange(
                                        0..0,
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        12,
                                    ),
                                },
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                trai_expr: TraitExpr {
                                    expr: 0,
                                },
                                for_token: ConnectionForToken {
                                    token_idx: TokenIdx(
                                        14,
                                    ),
                                },
                                ty_expr: TypeExpr {
                                    expr: 1,
                                },
                                eol_colon: Err(
                                    DeclExprError::Original(
                                        OriginalDeclExprError::ExpectEolColon(
                                            TokenIdx(
                                                16,
                                            ),
                                        ),
                                    ),
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId::TypeAsTrait(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `core::marker`,
                                                        trai_path: TraitPath(`core::marker::Copy`),
                                                        ty_path: TypePath(`core::num::i32`, `Extern`),
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
                                                                TraitPath(`core::marker::Copy`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
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
                                                        13,
                                                    ),
                                                    ident: `Copy`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::marker::Copy`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        15,
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
                DefnRegionPath::Impl(
                    ImplBlockId::TypeAsTrait(
                        TraitForTypeImplBlockId {
                            module_path: `core::marker`,
                            trai_path: TraitPath(`core::marker::Copy`),
                            ty_path: TypePath(`core::num::i64`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Defn::Impl(
                        ImplBlockDecl::TraitForType(
                            TraitForTypeImplBlockDecl {
                                ast_idx: 4,
                                impl_block: TraitForTypeImplBlock {
                                    id: TraitForTypeImplBlockId {
                                        module_path: `core::marker`,
                                        trai_path: TraitPath(`core::marker::Copy`),
                                        ty_path: TypePath(`core::num::i64`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 4,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            16,
                                        ),
                                    },
                                    trai_expr: 60,
                                    for_token: TokenIdx(
                                        18,
                                    ),
                                    ty_expr: 61,
                                    body: ArenaIdxRange(
                                        0..0,
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        16,
                                    ),
                                },
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                trai_expr: TraitExpr {
                                    expr: 0,
                                },
                                for_token: ConnectionForToken {
                                    token_idx: TokenIdx(
                                        18,
                                    ),
                                },
                                ty_expr: TypeExpr {
                                    expr: 1,
                                },
                                eol_colon: Err(
                                    DeclExprError::Original(
                                        OriginalDeclExprError::ExpectEolColon(
                                            TokenIdx(
                                                20,
                                            ),
                                        ),
                                    ),
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId::TypeAsTrait(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `core::marker`,
                                                        trai_path: TraitPath(`core::marker::Copy`),
                                                        ty_path: TypePath(`core::num::i64`, `Extern`),
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
                                                                TraitPath(`core::marker::Copy`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i64`, `Extern`),
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
                                                        17,
                                                    ),
                                                    ident: `Copy`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::marker::Copy`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        19,
                                                    ),
                                                    ident: `i64`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i64`, `Extern`),
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
                DefnRegionPath::Impl(
                    ImplBlockId::TypeAsTrait(
                        TraitForTypeImplBlockId {
                            module_path: `core::marker`,
                            trai_path: TraitPath(`core::marker::Copy`),
                            ty_path: TypePath(`core::num::i128`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Defn::Impl(
                        ImplBlockDecl::TraitForType(
                            TraitForTypeImplBlockDecl {
                                ast_idx: 5,
                                impl_block: TraitForTypeImplBlock {
                                    id: TraitForTypeImplBlockId {
                                        module_path: `core::marker`,
                                        trai_path: TraitPath(`core::marker::Copy`),
                                        ty_path: TypePath(`core::num::i128`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 5,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            20,
                                        ),
                                    },
                                    trai_expr: 62,
                                    for_token: TokenIdx(
                                        22,
                                    ),
                                    ty_expr: 63,
                                    body: ArenaIdxRange(
                                        0..0,
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        20,
                                    ),
                                },
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                trai_expr: TraitExpr {
                                    expr: 0,
                                },
                                for_token: ConnectionForToken {
                                    token_idx: TokenIdx(
                                        22,
                                    ),
                                },
                                ty_expr: TypeExpr {
                                    expr: 1,
                                },
                                eol_colon: Err(
                                    DeclExprError::Original(
                                        OriginalDeclExprError::ExpectEolColon(
                                            TokenIdx(
                                                24,
                                            ),
                                        ),
                                    ),
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId::TypeAsTrait(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `core::marker`,
                                                        trai_path: TraitPath(`core::marker::Copy`),
                                                        ty_path: TypePath(`core::num::i128`, `Extern`),
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
                                                                TraitPath(`core::marker::Copy`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i128`, `Extern`),
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
                                                    ident: `Copy`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::marker::Copy`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        23,
                                                    ),
                                                    ident: `i128`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i128`, `Extern`),
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
                DefnRegionPath::Impl(
                    ImplBlockId::TypeAsTrait(
                        TraitForTypeImplBlockId {
                            module_path: `core::marker`,
                            trai_path: TraitPath(`core::marker::Copy`),
                            ty_path: TypePath(`core::num::isize`, `Extern`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Defn::Impl(
                        ImplBlockDecl::TraitForType(
                            TraitForTypeImplBlockDecl {
                                ast_idx: 6,
                                impl_block: TraitForTypeImplBlock {
                                    id: TraitForTypeImplBlockId {
                                        module_path: `core::marker`,
                                        trai_path: TraitPath(`core::marker::Copy`),
                                        ty_path: TypePath(`core::num::isize`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 6,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            24,
                                        ),
                                    },
                                    trai_expr: 64,
                                    for_token: TokenIdx(
                                        26,
                                    ),
                                    ty_expr: 65,
                                    body: ArenaIdxRange(
                                        0..0,
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        24,
                                    ),
                                },
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                trai_expr: TraitExpr {
                                    expr: 0,
                                },
                                for_token: ConnectionForToken {
                                    token_idx: TokenIdx(
                                        26,
                                    ),
                                },
                                ty_expr: TypeExpr {
                                    expr: 1,
                                },
                                eol_colon: Err(
                                    DeclExprError::Original(
                                        OriginalDeclExprError::ExpectEolColon(
                                            TokenIdx(
                                                28,
                                            ),
                                        ),
                                    ),
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId::TypeAsTrait(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `core::marker`,
                                                        trai_path: TraitPath(`core::marker::Copy`),
                                                        ty_path: TypePath(`core::num::isize`, `Extern`),
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
                                                                TraitPath(`core::marker::Copy`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
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
                                                        25,
                                                    ),
                                                    ident: `Copy`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::marker::Copy`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        27,
                                                    ),
                                                    ident: `isize`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
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
        ],
    },
)