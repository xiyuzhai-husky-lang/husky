```rust
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
                PunctuationMapped::LeftDelimiter(
                    Delimiter::Par,
                ),
            ),
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
                PunctuationMapped::LeftDelimiter(
                    Delimiter::NestedCurl,
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
                PunctuationMapped::RightDelimiter(
                    Delimiter::NestedCurl,
                ),
            ),
        ),
        TokenData::Keyword(
            Keyword::Fugitive(
                Fn,
            ),
        ),
        TokenData::Ident(
            `closure_inline`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::LeftDelimiter(
                    Delimiter::Par,
                ),
            ),
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
                PunctuationMapped::Vert,
            ),
        ),
        TokenData::Ident(
            `x`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Colon,
            ),
        ),
        TokenData::Ident(
            `i32`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Vert,
            ),
        ),
        TokenData::Ident(
            `x`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::Closed(
                        BinaryClosedOpr::Add,
                    ),
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
        TokenData::Keyword(
            Keyword::Fugitive(
                Fn,
            ),
        ),
        TokenData::Ident(
            `closure_nested`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::LeftDelimiter(
                    Delimiter::Par,
                ),
            ),
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
                PunctuationMapped::Vert,
            ),
        ),
        TokenData::Ident(
            `x`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Colon,
            ),
        ),
        TokenData::Ident(
            `i32`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Vert,
            ),
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::LeftDelimiter(
                    Delimiter::NestedCurl,
                ),
            ),
        ),
        TokenData::Ident(
            `x`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::Closed(
                        BinaryClosedOpr::Add,
                    ),
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
                PunctuationMapped::RightDelimiter(
                    Delimiter::NestedCurl,
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
                TokenVerseData {
                    start: TokenVerseStart(
                        TokenIdx(
                            12,
                        ),
                    ),
                    indent: 0,
                },
                TokenVerseData {
                    start: TokenVerseStart(
                        TokenIdx(
                            17,
                        ),
                    ),
                    indent: 4,
                },
                TokenVerseData {
                    start: TokenVerseStart(
                        TokenIdx(
                            28,
                        ),
                    ),
                    indent: 0,
                },
                TokenVerseData {
                    start: TokenVerseStart(
                        TokenIdx(
                            33,
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
            NestedTokenVerseSequence {
                lcurl: TokenIdx(
                    41,
                ),
                verses_data: [
                    TokenVerseData {
                        start: TokenVerseStart(
                            TokenIdx(
                                42,
                            ),
                        ),
                        indent: 8,
                    },
                ],
                end: TokenIdx(
                    45,
                ),
            },
        ],
    },
}
```