```rust
TokenSheetData {
    tokens: [
        TokenData::Keyword(
            Keyword::Pub,
        ),
        TokenData::Keyword(
            Keyword::Use,
        ),
        TokenData::Ident(
            `core`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::ScopeResolution,
                ),
            ),
        ),
        TokenData::Ident(
            `logic`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::ScopeResolution,
                ),
            ),
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Star,
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
            ],
        },
        nested_sequences: [],
    },
}
```