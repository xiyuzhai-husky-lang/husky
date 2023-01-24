use husky_entity_taxonomy::{EntityKind, ModuleItemConnectionKind, ModuleItemKind, TypeKind};
use husky_entity_tree::UseExprRuleState;
use husky_expr::{CurrentSymbolKind, InheritedSymbolKind};

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
        .filter_map(|(info, (range, token))| token_to_semantic_token(db, info, token, range));
    let iter1 = ranged_token_sheet
        .comments()
        .iter()
        .map(|comment| comment_to_semantic_token(comment));
    Ok(itertools::merge(iter0, iter1).collect())
}

fn token_to_semantic_token(
    db: &dyn SemanticTokenDb,
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
        TokenInfo::Entity(path, kind) => {
            if let Some(path) = path {
                SemanticToken::Entity(path.entity_kind(db))
            } else if let Some(kind) = kind {
                SemanticToken::Entity(*kind)
            } else {
                return None;
            }
        }
        TokenInfo::CurrentSymbol {
            current_symbol_kind,
            ..
        } => match current_symbol_kind {
            CurrentSymbolKind::LetVariable { .. } => SemanticToken::Variable,
            CurrentSymbolKind::Parameter { .. } => SemanticToken::Parameter,
            CurrentSymbolKind::FrameVariable(_) => SemanticToken::FrameVariable,
            CurrentSymbolKind::ImplicitParameter { .. } => SemanticToken::ImplicitParameter,
        },
        // SemanticToken::Variable,
        TokenInfo::InheritedSymbol {
            inherited_symbol_kind,
            ..
        } => match inherited_symbol_kind {
            InheritedSymbolKind::Parameter => SemanticToken::Parameter,
            InheritedSymbolKind::ImplicitParameter => SemanticToken::ImplicitParameter,
        },
        TokenInfo::SelfType => todo!(),
        TokenInfo::SelfValue => SemanticToken::Variable,
        // SemanticToken::Variable,
        TokenInfo::Field => SemanticToken::Field,
        TokenInfo::Method => SemanticToken::Method,
        TokenInfo::BoxColon | TokenInfo::BoxPrefix => {
            SemanticToken::Entity(EntityKind::ModuleItem {
                module_item_kind: ModuleItemKind::Type(TypeKind::Alien),
                connection: ModuleItemConnectionKind::Connected,
            })
        }
        TokenInfo::UseExpr { state, .. } => match state {
            UseExprRuleState::Unresolved => return None,
            UseExprRuleState::Erroneous => return None,
            UseExprRuleState::Resolved { original_symbol } => {
                SemanticToken::Entity(original_symbol.path(db).entity_kind(db))
            }
        },
        TokenInfo::UseExprStar => SemanticToken::Special,
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
