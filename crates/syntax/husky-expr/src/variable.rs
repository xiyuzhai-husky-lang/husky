use crate::*;

#[derive(Default)]
pub struct VariableSheet {
    arena: VariableArena,
}
impl VariableSheet {
    #[inline(always)]
    pub(crate) fn define_variables(&mut self, variables: Vec<Variable>) -> ArenaIdxRange<Variable> {
        self.arena.alloc_batch(variables)
    }

    pub(crate) fn resolve_ident(
        &self,
        token_idx: TokenIdx,
        ident: Identifier,
    ) -> Option<VariableIdx> {
        // ad hoc
        self.arena.find_rev(|variable| {
            let accessible = match variable.access_end {
                Some(access_end) => access_end.token_idx() > token_idx,
                None => todo!(),
            };
            variable.ident == ident && accessible
        })
    }
}

pub type VariableArena = Arena<Variable>;
pub type VariableIdx = ArenaIdx<Variable>;
pub type VariableIdxRange = ArenaIdxRange<Variable>;

pub struct Variable {
    ident: Identifier,
    access_start: TokenIdx,
    /// this is none only for lambda variable
    access_end: Option<TokenIdxRangeEnd>,
    kind: VariableKind,
}

impl Variable {
    pub fn new(
        ident: Identifier,
        access_start: TokenIdx,
        access_end: Option<TokenIdxRangeEnd>,
        kind: VariableKind,
    ) -> Self {
        Self {
            ident,
            access_start,
            access_end,
            kind,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VariableKind {
    Let { pattern_symbol: PatternSymbolIdx },
    Lambda,
}

pub enum Prevariable {}
