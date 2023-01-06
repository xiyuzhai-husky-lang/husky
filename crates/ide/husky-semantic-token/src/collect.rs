use crate::*;

pub(crate) fn collect_semantic_tokens(
    db: &dyn SemanticTokenDb,
    module_path: ModulePath,
) -> EntityTreeResult<Vec<RangedSemanticToken>> {
    let ranged_token_sheet = db.ranged_token_sheet(module_path)?;
    let token_sheet_data = db.token_sheet_data(module_path)?;
    let token_infer_sheet = db.token_info_sheet(module_path)?;
    Ok(token_infer_sheet
        .informative_tokens(ranged_token_sheet, token_sheet_data)
        .filter_map(|(info, (range, token))| {
            let semantic_token = match info {
                TokenInfo::None => match token {
                    Token::Attr(_) => SemanticToken::Attribute,
                    Token::Keyword(kw) => SemanticToken::Keyword(*kw),
                    Token::Identifier(_) => return None,
                    Token::Punctuation(_) => SemanticToken::Special,
                    Token::WordOpr(_) => SemanticToken::WordOpr,
                    Token::Literal(_) => SemanticToken::Literal,
                    Token::Err(_) => return None,
                },
                TokenInfo::Entity(entity_kind) => SemanticToken::Entity(*entity_kind),
                TokenInfo::ImplicitParameter => SemanticToken::ImplicitParameter,
                TokenInfo::Parameter => SemanticToken::Parameter,
                TokenInfo::Variable { .. } => SemanticToken::Variable,
                TokenInfo::Field => SemanticToken::Field,
                TokenInfo::Method => SemanticToken::Method,
            };
            Some(RangedSemanticToken {
                semantic_token,
                range: *range,
            })
        })
        .collect())
}
