Ok(
    TokenInfoSheet {
        token_infos: [
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Trait(
                        TraitNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::marker::Copy`),
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Trait(
                        TraitNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitPath(`core::marker::Sized`),
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