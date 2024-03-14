TokenSheetData {
    tokens: [
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Pound,
            ),
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::LeftDelimiter(
                    Delimiter::Box,
                ),
            ),
        ),
        TokenData::Ident(
            `to_rust`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Eq,
            ),
        ),
        TokenData::Literal(
            LiteralTokenData::String(
                StringLiteralTokenData {
                    data: "std::ops::Add",
                },
            ),
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::RightDelimiter(
                    Delimiter::Box,
                ),
            ),
        ),
        TokenData::Keyword(
            Keyword::Trait,
        ),
        TokenData::Ident(
            `Add`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::LaOrLt,
            ),
        ),
        TokenData::Ident(
            `B`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::RaOrGt,
            ),
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Colon,
            ),
        ),
        TokenData::Keyword(
            Keyword::Fugitive(
                Type,
            ),
        ),
        TokenData::Ident(
            `Output`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Semicolon,
            ),
        ),
        TokenData::Keyword(
            Keyword::Fugitive(
                Fn,
            ),
        ),
        TokenData::Ident(
            `add`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::LeftDelimiter(
                    Delimiter::Par,
                ),
            ),
        ),
        TokenData::Ident(
            `other`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Colon,
            ),
        ),
        TokenData::Ident(
            `B`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::RightDelimiter(
                    Delimiter::Par,
                ),
            ),
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::CurryType,
                ),
            ),
        ),
        TokenData::Keyword(
            Keyword::Pronoun(
                SelfType,
            ),
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::ScopeResolution,
                ),
            ),
        ),
        TokenData::Ident(
            `Output`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Semicolon,
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
                            7,
                        ),
                    ),
                    indent: 0,
                },
                TokenVerseData {
                    start: TokenVerseStart(
                        TokenIdx(
                            13,
                        ),
                    ),
                    indent: 4,
                },
                TokenVerseData {
                    start: TokenVerseStart(
                        TokenIdx(
                            16,
                        ),
                    ),
                    indent: 4,
                },
            ],
        },
        nested_sequences: [],
    },
}