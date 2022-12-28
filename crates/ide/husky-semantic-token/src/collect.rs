use crate::*;

pub(crate) fn collect_semantic_tokens(
    db: &dyn SemanticTokenDb,
    module_path: ModulePath,
) -> EntityTreeResult<Vec<RangedSemanticToken>> {
    let token_sheet = db.token_sheet(module_path)?;
    let token_infer_sheet = db.token_infer_sheet(module_path)?;
    Ok(token_infer_sheet
        .informative_tokens(token_sheet)
        .filter_map(|(info, token)| {
            let semantic_token = match info {
                TokenInfo::None => match token.kind {
                    TokenKind::Attr(_) => SemanticToken::Attribute,
                    TokenKind::Keyword(kw) => SemanticToken::Keyword(kw),
                    TokenKind::Identifier(_) => return None,
                    TokenKind::Special(_) => SemanticToken::Special,
                    TokenKind::WordOpr(_) => SemanticToken::WordOpr,
                    TokenKind::Literal(_) => SemanticToken::Literal,
                    TokenKind::Comment => SemanticToken::Comment,
                    TokenKind::Err(_) => return None,
                },
            };
            Some(RangedSemanticToken {
                semantic_token,
                range: token.range,
            })
        })
        .collect())
}
