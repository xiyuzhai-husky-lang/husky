use super::*;
use husky_entity_path::EntityPath;
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
    Identifier { ident_token: IdentifierToken },
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

#[derive(Debug, Default, PartialEq, Eq)]
pub struct PatternExprSheet {
    arena: PatternExprArena,
    pattern_environments: Vec<PatternEnvironment>,
    pattern_symbol_maps: Vec<IdentPairMap<PatternSymbolIdx>>,
    pattern_symbol_arena: PatternSymbolArena,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PatternSymbol {
    Atom(PatternExprIdx),
}

pub type PatternSymbolArena = Arena<PatternSymbol>;
pub type PatternSymbolIdx = ArenaIdx<PatternSymbol>;

fn collect_symbols(
    pattern_expr_idx: PatternExprIdx,
    pattern_expr: &PatternExpr,
    pattern_symbol_arena: &mut PatternSymbolArena,
) -> IdentPairMap<PatternSymbolIdx> {
    match pattern_expr {
        PatternExpr::Literal(_) => Default::default(),
        PatternExpr::Identifier { ident_token } => [(
            ident_token.ident(),
            pattern_symbol_arena.alloc_one(PatternSymbol::Atom(pattern_expr_idx)),
        )]
        .into(),
        PatternExpr::Entity(_) => todo!(),
        PatternExpr::Tuple { name, fields } => todo!(),
        PatternExpr::Struct { name, fields } => todo!(),
        PatternExpr::OneOf { options } => todo!(),
        PatternExpr::Binding {
            ident_token,
            asperand_token,
            src,
        } => todo!(),
        PatternExpr::Range {
            start,
            dot_dot_token,
            end,
        } => todo!(),
    }
}

impl PatternExprSheet {
    pub fn alloc_one(&mut self, expr: PatternExpr, env: PatternEnvironment) -> PatternExprIdx {
        let expr_idx = self.arena.alloc_one(expr);
        assert_eq!(expr_idx.raw(), self.pattern_environments.len());
        self.pattern_environments.push(env);
        let expr = &self.arena[expr_idx];
        assert_eq!(expr_idx.raw(), self.pattern_symbol_maps.len());
        self.pattern_symbol_maps.push(collect_symbols(
            expr_idx,
            expr,
            &mut self.pattern_symbol_arena,
        ));
        expr_idx
    }

    pub fn pattern_exprs<'a>(
        &'a self,
    ) -> impl Iterator<Item = (PatternExprIdx, &'a PatternExpr)> + 'a {
        self.arena.indexed_iter()
    }

    pub fn pattern_symbol_map(
        &self,
        pattern_expr_idx: ArenaIdx<PatternExpr>,
    ) -> &IdentPairMap<PatternSymbolIdx> {
        &self.pattern_symbol_maps[pattern_expr_idx.raw()]
    }

    pub fn pattern_env(&self, pattern_expr_idx: ArenaIdx<PatternExpr>) -> PatternEnvironment {
        self.pattern_environments[pattern_expr_idx.raw()]
    }
}

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
                pattern_expr_idx: ctx.alloc_pattern_expr(
                    PatternExpr::Identifier { ident_token },
                    PatternEnvironment::Parameter,
                ),
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

#[derive(Debug, PartialEq, Eq)]
pub struct LetVariablePattern {
    pattern_expr_idx: PatternExprIdx,
    variables: VariableIdxRange,
}

impl<'a, 'b, 'c> ParseFrom<ExprParseContext<'a, 'b>> for LetVariablePattern {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, ExprError> {
        // ad hoc
        if let Some(pattern_expr_idx) = ctx.parse_pattern_expr(PatternEnvironment::Let)? {
            let symbols = ctx
                .pattern_expr_sheet()
                .pattern_symbol_map(pattern_expr_idx);
            let variables = symbols
                .iter()
                .map(|(ident, pattern_symbol)| Variable::Let {
                    pattern_symbol: *pattern_symbol,
                })
                .collect();
            let variables = ctx.define_variables(variables);
            Ok(Some(LetVariablePattern {
                pattern_expr_idx,
                variables,
            }))
        } else {
            Ok(None)
        }
    }
}

impl LetVariablePattern {
    pub fn pattern_expr_idx(&self) -> ArenaIdx<PatternExpr> {
        self.pattern_expr_idx
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BePattern {
    pattern_expr_idx: PatternExprIdx,
}

impl<'a, 'b, 'c> ParseFrom<ExprParseContext<'a, 'b>> for BePattern {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, ExprError> {
        // ad hoc
        if let Some(ident_token) = ctx.parse::<IdentifierToken>()? {
            Ok(Some(BePattern {
                pattern_expr_idx: ctx.alloc_pattern_expr(
                    PatternExpr::Identifier { ident_token },
                    PatternEnvironment::Be,
                ),
            }))
        } else {
            Ok(None)
        }
    }
}

impl BePattern {
    pub fn pattern_expr_idx(&self) -> ArenaIdx<PatternExpr> {
        self.pattern_expr_idx
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PatternEnvironment {
    Parameter,
    Let,
    Match,
    Be,
}
