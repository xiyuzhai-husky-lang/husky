use crate::*;
use husky_entity_tree::CratePrelude;

pub trait SymbolContext {
    fn resolve_ident(&self, token_idx: TokenIdx, ident: Identifier) -> Option<Symbol>;

    fn exprs(&self) -> &[Expr];
}

pub trait SymbolContextMut: SymbolContext {
    type ExprSheet;

    fn into_expr_sheet(
        self,
        db: &dyn ExprDb,
        expr_arena: ExprArena,
        entity_path_expr_arena: EntityPathExprArena,
        pattern_expr_sheet: PatternExprSheet,
        stmt_arena: StmtArena,
    ) -> Self::ExprSheet;
}

pub trait BlockSymbolContextMut: SymbolContextMut {
    fn define_variables(&mut self, variables: Vec<Variable>) -> VariableIdxRange;
}

// module item decl

pub struct ModuleItemDeclSymbolContextMut {}

impl SymbolContext for ModuleItemDeclSymbolContextMut {
    fn resolve_ident(&self, token_idx: TokenIdx, ident: Identifier) -> Option<Symbol> {
        todo!()
    }

    fn exprs(&self) -> &[Expr] {
        todo!()
    }
}

impl SymbolContextMut for ModuleItemDeclSymbolContextMut {
    type ExprSheet = ModuleItemDeclExprSheet;

    fn into_expr_sheet(
        self,
        db: &dyn ExprDb,
        expr_arena: ExprArena,
        entity_path_expr_arena: EntityPathExprArena,
        pattern_expr_sheet: PatternExprSheet,
        stmt_arena: StmtArena,
    ) -> Self::ExprSheet {
        todo!()
    }
}

// module item defn

pub struct ModuleItemDefnSymbolContextMut {}

impl SymbolContext for ModuleItemDefnSymbolContextMut {
    fn resolve_ident(&self, token_idx: TokenIdx, ident: Identifier) -> Option<Symbol> {
        todo!()
    }

    fn exprs(&self) -> &[Expr] {
        todo!()
    }
}

impl SymbolContextMut for ModuleItemDefnSymbolContextMut {
    type ExprSheet = ModuleItemDefnExprSheet;

    fn into_expr_sheet(
        self,
        db: &dyn ExprDb,
        expr_arena: ExprArena,
        entity_path_expr_arena: EntityPathExprArena,
        pattern_expr_sheet: PatternExprSheet,
        stmt_arena: StmtArena,
    ) -> Self::ExprSheet {
        todo!()
    }
}

impl BlockSymbolContextMut for ModuleItemDefnSymbolContextMut {
    fn define_variables(&mut self, variables: Vec<Variable>) -> VariableIdxRange {
        todo!()
    }
}

pub struct SymbolSheet<'a> {
    crate_prelude: CratePrelude<'a>,
    variable_sheet_data: VariableSheetData,
}

impl<'a> SymbolSheet<'a> {
    pub fn new(crate_prelude: CratePrelude<'a>) -> Self {
        Self {
            crate_prelude,
            variable_sheet_data: Default::default(),
        }
    }

    pub(crate) fn resolve_ident(&self, token_idx: TokenIdx, ident: Identifier) -> Option<Symbol> {
        // ad hoc
        if let Some(variable) = self.variable_sheet_data.resolve_ident(token_idx, ident) {
            Some(Symbol::Variable(variable))
        } else if let Some(entity_symbol) = self.crate_prelude.resolve_ident(ident) {
            Some(Symbol::Entity(entity_symbol.entity_path()))
        } else {
            None
        }
    }

    #[inline(always)]
    pub(crate) fn define_variables(&mut self, variables: Vec<Variable>) -> ArenaIdxRange<Variable> {
        self.variable_sheet_data.define_variables(variables)
    }

    pub fn variable_sheet_data(self) -> VariableSheetData {
        self.variable_sheet_data
    }
}

pub enum Symbol {
    Variable(VariableIdx),
    Entity(EntityPath),
}
