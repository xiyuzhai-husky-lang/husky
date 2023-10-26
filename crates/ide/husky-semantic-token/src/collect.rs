use husky_entity_kind::{EntityKind, MajorItemConnectionKind, MajorItemKind, TypeKind};
use husky_entity_syn_tree::OnceUseRuleState;
use husky_syn_expr::{SynCurrentSymbolKind, SynInheritedSymbolKind};
use husky_token_data::TokenData;
use husky_token_protocol::TokenClass;

use crate::*;

pub(crate) fn collect_semantic_tokens(
    db: &dyn SemanticTokenDb,
    module_path: ModulePath,
) -> EntitySynTreeResult<Vec<SemanticToken>> {
    let ranged_token_sheet = db.ranged_token_sheet(module_path)?;
    let _token_sheet_data = db.token_sheet_data(module_path)?;
    let token_infer_sheet = db.token_info_sheet(module_path)?;
    let iter0 = token_infer_sheet
        .informative_ranged_token_iter(ranged_token_sheet, db)
        .filter_map(|(info, (range, token))| token_to_semantic_token(db, info, token, range));
    let iter1 = ranged_token_sheet
        .comments()
        .iter()
        .map(|comment| comment_to_semantic_token(comment));
    Ok(itertools::merge(iter0, iter1).collect())
}

fn token_to_semantic_token(
    db: &dyn SemanticTokenDb,
    info: Option<&TokenInfo>,
    token_data: &TokenData,
    range: &husky_text_protocol::range::TextRange,
) -> Option<SemanticToken> {
    Some(SemanticToken {
        token_class: match info {
            Some(info) => info.data().semantic_token_kind(db),
            None => token_data.default_protocol(),
        },
        range: *range,
    })
}

fn comment_to_semantic_token(comment: &Comment) -> SemanticToken {
    SemanticToken {
        token_class: TokenClass::Comment,
        range: comment.range(),
    }
}
