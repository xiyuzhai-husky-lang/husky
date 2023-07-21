Ok(
    TokenInfoSheet {
        token_infos: [
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntitySynNodePath::ModuleItem(
                    ModuleItemSynNodePath::Trait(
                        TraitSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::fmt::Debug`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ModuleItem {
                    module_item_kind: Trait,
                    connection: Connected,
                },
            ),
            TokenInfo::None,
        ],
    },
)