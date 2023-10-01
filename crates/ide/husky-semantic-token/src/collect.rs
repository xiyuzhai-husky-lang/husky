use husky_entity_syn_tree::OnceUseRuleState;
use husky_entity_taxonomy::{EntityKind, MajorItemConnectionKind, MajorItemKind, TypeKind};
use husky_syn_expr::{SynCurrentSymbolKind, SynInheritedSymbolKind};
use husky_token_data::TokenData;

use crate::*;

pub(crate) fn collect_semantic_tokens(
    db: &dyn SemanticTokenDb,
    module_path: ModulePath,
) -> EntitySynTreeResult<Vec<RangedSemanticToken>> {
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
    token: &TokenData,
    range: &husky_text::TextRange,
) -> Option<RangedSemanticToken> {
    let semantic_token = match info {
        None => match token {
            TokenData::Keyword(kw) => SemanticToken::Keyword(*kw),
            TokenData::Punctuation(_) => SemanticToken::Special,
            TokenData::WordOpr(_) => SemanticToken::WordOpr,
            TokenData::Literal(_) => SemanticToken::Literal,
            TokenData::Ident(_) | TokenData::Label(_) | TokenData::Error(_) => return None,
        },
        Some(info) => match info.data() {
            TokenInfoData::Entity(path) => SemanticToken::Entity(path.item_kind(db)),
            TokenInfoData::EntityNode(path, item_kind) => SemanticToken::Entity(*item_kind),
            TokenInfoData::CurrentSymbol {
                current_symbol_kind,
                ..
            } => match current_symbol_kind {
                SynCurrentSymbolKind::LetVariable { .. }
                | SynCurrentSymbolKind::BeVariable { .. }
                | SynCurrentSymbolKind::CaseVariable { .. } => SemanticToken::Variable,
                SynCurrentSymbolKind::ExplicitRegularParameter { .. } => SemanticToken::Parameter,
                SynCurrentSymbolKind::FrameVariable(_) => SemanticToken::FrameVariable,
                SynCurrentSymbolKind::ImplicitParameter { .. } => SemanticToken::ImplicitParameter,
                SynCurrentSymbolKind::ExplicitVariadicParameter { .. } => SemanticToken::Parameter,
                SynCurrentSymbolKind::FieldVariable { .. } => SemanticToken::Variable,
            },
            // SemanticToken::Variable,
            TokenInfoData::InheritedSymbol {
                inherited_symbol_kind,
                ..
            } => match inherited_symbol_kind {
                SynInheritedSymbolKind::ParenateParameter { .. } => SemanticToken::Parameter,
                SynInheritedSymbolKind::TemplateParameter { .. } => {
                    SemanticToken::ImplicitParameter
                }
                SynInheritedSymbolKind::FieldVariable { .. } => SemanticToken::Variable,
            },
            TokenInfoData::SelfType => SemanticToken::SelfType,
            TokenInfoData::SelfValue => SemanticToken::SelfValue,
            // SemanticToken::Variable,
            TokenInfoData::Field => SemanticToken::Field,
            TokenInfoData::Method => SemanticToken::Method,
            TokenInfoData::BoxColon | TokenInfoData::BoxPrefix => {
                SemanticToken::Entity(EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Type(TypeKind::Extern),
                    connection: MajorItemConnectionKind::Connected,
                })
            }
            TokenInfoData::UseExpr { state, .. } => match state {
                OnceUseRuleState::Resolved {
                    original_symbol: Some(original_symbol),
                } => SemanticToken::Entity(original_symbol.path(db).item_kind(db)),
                _ => return None,
            },
            TokenInfoData::UseExprStar => SemanticToken::Special,
            TokenInfoData::HtmlFunctionIdent => SemanticToken::HtmlFunctionIdent,
            TokenInfoData::HtmlPropertyIdent => SemanticToken::HtmlPropertyIdent,
            TokenInfoData::SubmoduleIdent => SemanticToken::SubmoduleIdent,
            TokenInfoData::UnitLeftParenthesis | TokenInfoData::UnitRightParenthesis => {
                SemanticToken::Entity(EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Type(TypeKind::Extern),
                    connection: MajorItemConnectionKind::Connected,
                })
            }
            TokenInfoData::Todo => SemanticToken::Todo,
            TokenInfoData::Unreachable => SemanticToken::Unreachable,
        },
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
