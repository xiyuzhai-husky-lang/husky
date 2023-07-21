Ok(
    TokenInfoSheet {
        token_infos: [
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntitySynNodePath::ModuleItem(
                    ModuleItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::array::Array`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ModuleItem {
                    module_item_kind: Type(
                        Extern,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Constant {
                        ident_token: IdentToken {
                            ident: `L`,
                            token_idx: TokenIdx(
                                5,
                            ),
                        },
                    },
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::usize`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
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
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
        ],
    },
)