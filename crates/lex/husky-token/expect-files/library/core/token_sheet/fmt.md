TokenSheetData {
    tokens: [
        TokenData::Keyword(
            Keyword::Use,
        ),
        TokenData::Keyword(
            Keyword::Pronoun(
                Crate,
            ),
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
        TokenData::Keyword(
            Keyword::Pub,
        ),
        TokenData::Keyword(
            Keyword::Trait,
        ),
        TokenData::Ident(
            `Debug`,
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
                            5,
                        ),
                    ),
                    indent: 0,
                },
            ],
        },
        nested_sequences: [],
    },
}