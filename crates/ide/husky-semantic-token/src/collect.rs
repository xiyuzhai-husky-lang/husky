use crate::*;

pub(crate) fn collect_semantic_tokens(
    db: &dyn SemanticTokenDb,
    module_path: ModulePath,
) -> EntityTreeResult<Vec<RangedSemanticToken>> {
    let ranged_token_sheet = db.ranged_token_sheet(module_path)?;
    let token_sheet_data = db.token_sheet_data(module_path)?;
    let token_infer_sheet = db.token_info_sheet(module_path)?;
    let iter0 = token_infer_sheet
        .informative_ranged_token_iter(ranged_token_sheet, db)
        .filter_map(|(info, (range, token))| token_to_semantic_token(info, token, range));
    let iter1 = ranged_token_sheet
        .comments()
        .iter()
        .map(|comment| comment_to_semantic_token(comment));
    Ok(itertools::merge(iter0, iter1).collect())
}

fn token_to_semantic_token(
    info: &TokenInfo,
    token: &Token,
    range: &husky_text::TextRange,
) -> Option<RangedSemanticToken> {
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
}

fn comment_to_semantic_token(comment: &Comment) -> RangedSemanticToken {
    RangedSemanticToken {
        semantic_token: SemanticToken::Comment,
        range: comment.range(),
    }
}
