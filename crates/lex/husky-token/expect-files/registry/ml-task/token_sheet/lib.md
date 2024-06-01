```rust
TokenSheetData {
    tokens: [
        TokenData::Keyword(
            Keyword::Pub,
        ),
        TokenData::Keyword(
            Keyword::Trait,
        ),
        TokenData::Ident(
            `IsMlTask`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Colon,
            ),
        ),
        TokenData::Keyword(
            Keyword::Form(
                Type,
            ),
        ),
        TokenData::Ident(
            `Input`,
        ),
        TokenData::Keyword(
            Keyword::Form(
                Static,
            ),
        ),
        TokenData::Ident(
            `INPUT`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Colon,
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
            `Input`,
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
                            5,
                        ),
                    ),
                    indent: 4,
                },
                TokenVerseData {
                    start: TokenVerseStart(
                        TokenIdx(
                            7,
                        ),
                    ),
                    indent: 4,
                },
            ],
        },
        nested_sequences: [],
    },
}
```