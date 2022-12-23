Ok(
    TokenSheet {
        tokens: [
            Token {
                range: [1:1, 1:5),
                kind: Keyword(
                    Config(
                        Task,
                    ),
                ),
            },
            Token {
                range: [1:5, 1:6),
                kind: Special(
                    Colon,
                ),
            },
            Token {
                range: [2:5, 2:13),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 63,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [2:13, 2:15),
                kind: Special(
                    BinaryOpr(
                        ScopeResolution,
                    ),
                ),
            },
            Token {
                range: [2:15, 2:21),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 64,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [2:21, 2:23),
                kind: Special(
                    BinaryOpr(
                        ScopeResolution,
                    ),
                ),
            },
            Token {
                range: [2:23, 2:40),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 65,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [2:40, 2:41),
                kind: Special(
                    Bra(
                        Par,
                    ),
                ),
            },
            Token {
                range: [2:41, 2:42),
                kind: Special(
                    Ket(
                        Par,
                    ),
                ),
            },
            Token {
                range: [4:1, 4:5),
                kind: Keyword(
                    Main,
                ),
            },
            Token {
                range: [4:5, 4:6),
                kind: Special(
                    Colon,
                ),
            },
            Token {
                range: [5:5, 5:6),
                kind: Literal(
                    Integer(
                        1,
                    ),
                ),
            },
        ],
        group_starts: [
            0,
            2,
            9,
            11,
        ],
    },
)