use husky_entity_kind::{EntityKind, MajorItemConnectionKind, MajorItemKind, TypeKind};
use husky_entity_syn_tree::OnceUseRuleState;
use husky_semantic_token_kind::SemanticTokenKind;
use husky_syn_expr::{SynCurrentSymbolKind, SynInheritedSymbolKind};
use husky_token_data::TokenData;

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
    token: &TokenData,
    range: &husky_text::TextRange,
) -> Option<SemanticToken> {
    let semantic_token_kind = match info {
        None => match token {
            TokenData::Keyword(kw) => SemanticTokenKind::Keyword(kw.kind()),
            TokenData::Punctuation(_) => SemanticTokenKind::Special,
            TokenData::WordOpr(_) => SemanticTokenKind::WordOpr,
            TokenData::Literal(_) => SemanticTokenKind::Literal,
            TokenData::Ident(_) | TokenData::Label(_) | TokenData::Error(_) => return None,
        },
        Some(info) => match info.data() {
            TokenInfoData::Entity(path) => SemanticTokenKind::Entity(path.item_kind(db)),
            TokenInfoData::EntityNode(path, item_kind) => SemanticTokenKind::Entity(*item_kind),
            TokenInfoData::CurrentSymbol {
                current_symbol_kind,
                ..
            } => match current_symbol_kind {
                SynCurrentSymbolKind::LetVariable { .. }
                | SynCurrentSymbolKind::BeVariable { .. }
                | SynCurrentSymbolKind::CaseVariable { .. } => SemanticTokenKind::Variable,
                SynCurrentSymbolKind::ExplicitRegularParameter { .. } => {
                    SemanticTokenKind::Parameter
                }
                SynCurrentSymbolKind::FrameVariable(_) => SemanticTokenKind::FrameVariable,
                SynCurrentSymbolKind::ImplicitParameter { .. } => {
                    SemanticTokenKind::ImplicitParameter
                }
                SynCurrentSymbolKind::ExplicitVariadicParameter { .. } => {
                    SemanticTokenKind::Parameter
                }
                SynCurrentSymbolKind::FieldVariable { .. } => SemanticTokenKind::Variable,
            },
            // SemanticTokenKind::Variable,
            TokenInfoData::InheritedSymbol {
                inherited_symbol_kind,
                ..
            } => match inherited_symbol_kind {
                SynInheritedSymbolKind::ParenateParameter { .. } => SemanticTokenKind::Parameter,
                SynInheritedSymbolKind::TemplateParameter { .. } => {
                    SemanticTokenKind::ImplicitParameter
                }
                SynInheritedSymbolKind::FieldVariable { .. } => SemanticTokenKind::Variable,
            },
            TokenInfoData::SelfType => SemanticTokenKind::SelfType,
            TokenInfoData::SelfValue => SemanticTokenKind::SelfValue,
            // SemanticTokenKind::Variable,
            TokenInfoData::Field => SemanticTokenKind::Field,
            TokenInfoData::Method => SemanticTokenKind::Method,
            TokenInfoData::BoxColon | TokenInfoData::BoxPrefix => {
                SemanticTokenKind::Entity(EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Type(TypeKind::Extern),
                    connection: MajorItemConnectionKind::Connected,
                })
            }
            TokenInfoData::UseExpr { state, .. } => match state {
                OnceUseRuleState::Resolved {
                    original_symbol: Some(original_symbol),
                } => SemanticTokenKind::Entity(original_symbol.path(db).item_kind(db)),
                _ => return None,
            },
            TokenInfoData::UseExprStar => SemanticTokenKind::Special,
            TokenInfoData::HtmlFunctionIdent => SemanticTokenKind::HtmlFunctionIdent,
            TokenInfoData::HtmlPropertyIdent => SemanticTokenKind::HtmlPropertyIdent,
            TokenInfoData::SubmoduleIdent => SemanticTokenKind::SubmoduleIdent,
            TokenInfoData::UnitLeftParenthesis | TokenInfoData::UnitRightParenthesis => {
                SemanticTokenKind::Entity(EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Type(TypeKind::Extern),
                    connection: MajorItemConnectionKind::Connected,
                })
            }
            TokenInfoData::Todo => SemanticTokenKind::Todo,
            TokenInfoData::Unreachable => SemanticTokenKind::Unreachable,
        },
    };
    Some(SemanticToken {
        kind: semantic_token_kind,
        range: *range,
    })
}

fn comment_to_semantic_token(comment: &Comment) -> SemanticToken {
    SemanticToken {
        kind: SemanticTokenKind::Comment,
        range: comment.range(),
    }
}
