use crate::{ExprError, ExprParseContext, ExprSheet};
use husky_entity_path::EntityPath;
use husky_symbol::SymbolContext;
use husky_token::{AtToken, DotDotToken, IdentifierToken, TokenStream};
use husky_word::Identifier;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use ordered_float::NotNan;
use parsec::{ParseContext, ParseFrom};

#[derive(Debug, PartialEq, Eq)]
pub enum PatternExpr {
    /// example: `1`
    Literal(LiteralData),
    /// example: `a`
    ParameterIdentifier { ident_token: IdentifierToken },
    /// example: `A::B`
    Entity(EntityPath),
    /// example: `(a, b)`
    Tuple {
        name: Option<EntityPath>,
        fields: PatternExprIdxRange,
    },
    /// example: `C { .. }`
    Struct {
        name: Option<EntityPath>,
        fields: PatternExprIdxRange,
    },
    /// example: `A | B | C { .. }`
    OneOf { options: PatternExprIdxRange },
    /// example: `x @ 1..9`
    Binding {
        ident_token: IdentifierToken,
        asperand_token: AtToken,
        /// example: `1..9`
        src: PatternExprIdx,
    },
    /// example: `1..9`
    Range {
        start: PatternExprIdx,
        dot_dot_token: DotDotToken,
        end: PatternExprIdx,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum LiteralData {
    NotNanFloat,
    NotNanF32(NotNan<f32>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum PatternOpn {}

pub(crate) type PatternExprArena = Arena<PatternExpr>;
pub type PatternExprIdx = ArenaIdx<PatternExpr>;
pub type PatternExprIdxRange = ArenaIdxRange<PatternExpr>;

// impl<'a, 'b, 'c> ParseFrom<ExprParseContext<'a, 'b>> for PatternExprIdx {
//     fn parse_from_without_guaranteed_rollback(
//         ctx: &mut ExprParseContext<'a, 'b>,
//     ) -> Result<Option<Self>, ExprError> {
//         // ad hoc
//         if let Some(ident_token) = ctx.parse::<IdentifierToken>()? {
//             Ok(Some(ctx.alloc_pattern_expr(PatternExpr::Identifier {
//                 ident_token,
//             })))
//         } else {
//             Ok(None)
//         }
//     }
// }

#[derive(Debug, PartialEq, Eq)]
pub struct ParameterPattern {
    pattern_expr_idx: PatternExprIdx,
}

impl<'a, 'b, 'c> ParseFrom<ExprParseContext<'a, 'b>> for ParameterPattern {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, ExprError> {
        // ad hoc
        if let Some(ident_token) = ctx.parse::<IdentifierToken>()? {
            Ok(Some(ParameterPattern {
                pattern_expr_idx: ctx
                    .alloc_pattern_expr(PatternExpr::ParameterIdentifier { ident_token }),
            }))
        } else {
            Ok(None)
        }
    }
}

impl ParameterPattern {
    pub fn pattern_expr_idx(&self) -> ArenaIdx<PatternExpr> {
        self.pattern_expr_idx
    }
}
