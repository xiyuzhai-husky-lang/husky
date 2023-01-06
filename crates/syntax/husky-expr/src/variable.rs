use crate::*;

#[salsa::tracked(jar = ExprJar)]
pub struct VariableSheet {
    #[return_ref]
    pub data: VariableSheetData,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct VariableSheetData {
    arena: VariableArena,
}
impl VariableSheetData {
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

    pub fn index_variable_iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = (VariableIdx, &'a Variable)> + 'a {
        self.arena.indexed_iter()
    }
}

pub type VariableArena = Arena<Variable>;
pub type VariableIdx = ArenaIdx<Variable>;
pub type VariableIdxRange = ArenaIdxRange<Variable>;

impl std::ops::Index<VariableIdx> for VariableSheetData {
    type Output = Variable;

    fn index(&self, index: VariableIdx) -> &Self::Output {
        &self.arena[index]
    }
}

#[derive(Debug, PartialEq, Eq)]
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

    pub fn kind(&self) -> VariableKind {
        self.kind
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VariableKind {
    Let { pattern_symbol: PatternSymbolIdx },
    Lambda,
}

pub enum Prevariable {}
