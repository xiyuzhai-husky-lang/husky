Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::list::List`, `Alien`),
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
        ],
    },
)