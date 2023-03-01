#![feature(result_flattening)]
#![feature(trait_upcasting)]
mod error;
mod parser;
mod precedence;
mod stmt;
mod symbol;
#[cfg(test)]
mod tests;

pub use self::error::*;
pub use self::parser::*;
pub use self::stmt::*;
pub use self::symbol::*;

#[cfg(test)]
use self::tests::*;
use husky_doc::*;
use husky_entity_path::EntityPath;
use husky_entity_path_expr::*;
use husky_entity_tree::*;
use husky_opn_syntax::*;
use husky_pattern_expr::*;
use husky_term_expr::*;
use husky_token::*;
use husky_word::*;
use idx_arena::*;
use precedence::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum RuntimeExpr {
    Literal(TokenIdx),
    EntityPath {
        entity_path_expr: EntityPathExprIdx,
        entity_path: Option<EntityPath>,
    },
    InheritedSymbol {
        ident: Identifier,
        token_idx: TokenIdx,
        inherited_symbol_idx: InheritedRuntimeExprSymbolIdx,
        inherited_symbol_kind: InheritedRuntimeExprSymbolKind,
    },
    CurrentRuntimeExprSymbol {
        ident: Identifier,
        token_idx: TokenIdx,
        current_symbol_idx: CurrentRuntimeExprSymbolIdx,
        current_symbol_kind: CurrentRuntimeExprSymbolKind,
    },
    FrameVarDecl {
        token_idx: TokenIdx,
        ident: Identifier,
        frame_var_symbol_idx: CurrentRuntimeExprSymbolIdx,
        current_symbol_kind: CurrentRuntimeExprSymbolKind,
    },
    SelfType(TokenIdx),
    SelfValue(TokenIdx),
    BinaryOpn {
        lopd: RuntimeExprIdx,
        opr: BinaryOpr,
        opr_token_idx: TokenIdx,
        ropd: RuntimeExprIdx,
    },
    Be {
        src: RuntimeExprIdx,
        be_token_idx: TokenIdx,
        target: RuntimeExprResult<BeVariableDeclPattern>,
    },
    PrefixOpn {
        opr: PrefixOpr,
        opr_token_idx: TokenIdx,
        opd: RuntimeExprIdx,
    },
    SuffixOpn {
        opd: RuntimeExprIdx,
        opr: SuffixOpr,
        opr_token_idx: TokenIdx,
    },
    ApplicationOrRitchieCall {
        function: RuntimeExprIdx,
        lpar_token_idx: TokenIdx,
        argument: RuntimeExprIdx,
        rpar_token_idx: TokenIdx,
    },
    RitchieCall {
        function: RuntimeExprIdx,
        implicit_arguments: Option<ImplicitArgumentList>,
        lpar_token_idx: TokenIdx,
        arguments: RuntimeExprIdxRange,
        rpar_token_idx: TokenIdx,
    },
    Field {
        owner: RuntimeExprIdx,
        dot_token_idx: TokenIdx,
        ident_token: IdentifierToken,
    },
    MethodCall {
        self_argument: RuntimeExprIdx,
        dot_token_idx: TokenIdx,
        ident_token: IdentifierToken,
        implicit_arguments: Option<ImplicitArgumentList>,
        lpar_token_idx: TokenIdx,
        nonself_arguments: RuntimeExprIdxRange,
        rpar_token_idx: TokenIdx,
    },
    TemplateInstantiation {
        template: RuntimeExprIdx,
        implicit_arguments: ImplicitArgumentList,
    },
    Application {
        function: RuntimeExprIdx,
        argument: RuntimeExprIdx,
    },
    Bracketed {
        lpar_token_idx: TokenIdx,
        item: RuntimeExprIdx,
        rpar_token_idx: TokenIdx,
    },
    NewTuple {
        lpar_token_idx: TokenIdx,
        items: RuntimeExprIdxRange,
        commas: Vec<TokenIdx>,
        rpar_token_idx: TokenIdx,
    },
    NewBoxList {
        caller: Option<RuntimeExprIdx>,
        lbox_token_idx: TokenIdx,
        items: RuntimeExprIdxRange,
        rbox_token_idx: TokenIdx,
    },
    BoxColon {
        caller: Option<RuntimeExprIdx>,
        lbox_token_idx: TokenIdx,
        colon_token_idx: TokenIdx,
        rbox_token: RightBoxBracketToken,
    },
    Block {
        stmts: RuntimeStmtIdxRange,
    },
    Err(RuntimeExprError),
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct ImplicitArgumentList {}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct BeVariableDeclPattern {}

pub type RuntimeExprIdx = ArenaIdx<RuntimeExpr>;
pub type RuntimeExprArena = Arena<RuntimeExpr>;
pub type RuntimeExprIdxRange = ArenaIdxRange<RuntimeExpr>;
