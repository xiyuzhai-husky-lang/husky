Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::mem::Ref`, `Alien`),
                        ),
                    ),
                ),
                Err(
                    Expr(
                        MissingRightAngleBracket {
                            langle_token_idx: TokenIdx(
                                3,
                            ),
                        },
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::mem::RefMut`, `Alien`),
                        ),
                    ),
                ),
                Err(
                    Expr(
                        MissingRightAngleBracket {
                            langle_token_idx: TokenIdx(
                                14,
                            ),
                        },
                    ),
                ),
            ),
        ],
    },
)