Ok(
    DeclSheet {
        decls: [
            (
                TypePath(`core::logic::LogicAnd`, `Structure`),
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
                TypePath(`core::logic::LogicOr`, `Inductive`),
                Err(
                    Expr(
                        MissingRightAngleBracket {
                            langle_token_idx: TokenIdx(
                                25,
                            ),
                        },
                    ),
                ),
            ),
        ],
    },
)