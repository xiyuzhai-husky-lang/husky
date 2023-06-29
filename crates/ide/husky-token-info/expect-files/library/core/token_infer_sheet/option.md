Ok(
    TokenInfoSheet {
        token_infos: [
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 1,
                rule_idx: OnceUseRuleIdx(
                    0,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::ModuleItem {
                            module_item_path: ModuleItemPath::Type(
                                TypePath(`core::option::Option`, `Enum`),
                            ),
                            node: ModuleItemNode {
                                node_path: ModuleItemNodePath::Type(
                                    TypeNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypePath(`core::option::Option`, `Enum`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 3,
                                ident_token: IdentToken {
                                    ident: `Option`,
                                    token_idx: TokenIdx(
                                        7,
                                    ),
                                },
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExprStar,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::option::Option`, `Enum`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ModuleItem {
                    module_item_kind: Type(
                        Enum,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `T`,
                            token_idx: TokenIdx(
                                9,
                            ),
                        },
                    },
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
        ],
    },
)