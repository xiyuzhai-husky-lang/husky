TokenSheetData {
    tokens: [
        TokenData::Keyword(
            Keyword::Fugitive(
                Fn,
            ),
        ),
        TokenData::Ident(
            `nested`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Bra(
                    Delimiter::Par,
                ),
            ),
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Ket(
                    Delimiter::Par,
                ),
            ),
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Colon,
            ),
        ),
        TokenData::Keyword(
            Keyword::Stmt(
                Let,
            ),
        ),
        TokenData::Ident(
            `t`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Eq,
            ),
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Bra(
                    Delimiter::BlockCurl,
                ),
            ),
        ),
        TokenData::Literal(
            LiteralTokenData::Integer(
                UnspecifiedRegular(
                    1,
                ),
            ),
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Ket(
                    Delimiter::BlockCurl,
                ),
            ),
        ),
    ],
    token_verses: TokenVerses {
        main_sequence: MainTokenVerseSequence {
            verses_data: [
                TokenVerseData {
                    start: TokenVerseStart(
                        TokenIdx(
                            1,
                        ),
                    ),
                    indent: 0,
                },
                TokenVerseData {
                    start: TokenVerseStart(
                        TokenIdx(
                            6,
                        ),
                    ),
                    indent: 4,
                },
            ],
        },
        nested_sequences: [
            NestedTokenVerseSequence {
                lcurl: TokenIdx(
                    9,
                ),
                verses_data: [
                    TokenVerseData {
                        start: TokenVerseStart(
                            TokenIdx(
                                10,
                            ),
                        ),
                        indent: 8,
                    },
                ],
                end: TokenIdx(
                    11,
                ),
            },
        ],
    },
}