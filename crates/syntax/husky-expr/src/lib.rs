#![feature(result_flattening)]
#![feature(trait_upcasting)]
mod db;
mod entity_path;
mod error;
mod parser;
mod pattern;
mod precedence;
mod range;
mod region;
mod regular;
mod snippet;
mod stmt;
mod symbol;
mod term;
#[cfg(test)]
mod tests;

pub use db::*;
pub use entity_path::*;
pub use error::*;
pub use parser::*;
pub use pattern::*;
pub use range::*;
pub use region::*;
pub use stmt::*;
pub use symbol::*;

use husky_doc::*;
use husky_entity_path::EntityPath;
use husky_entity_tree::*;
use husky_opn_syntax::*;
use husky_token::*;
use husky_word::*;
use precedence::*;
use range::*;
use snippet::*;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = ExprDb)]
pub struct ExprJar(ExprRegion, parse_expr_from_snippet, expr_range_region);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BaseEntityPath {
    None,
    Some(EntityPath),
    Uncertain {
        inclination: BaseEntityPathInclination,
    },
    Err,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BaseEntityPathInclination {
    GlobalValue,
    FunctionOrLocalValue,
    TypeOrVariant,
}

impl BaseEntityPathInclination {
    pub fn from_case(case: IdentifierCase) -> Self {
        match case {
            IdentifierCase::SingleCapital | IdentifierCase::PascalCase => {
                BaseEntityPathInclination::TypeOrVariant
            }
            IdentifierCase::AllCapital => BaseEntityPathInclination::GlobalValue,
            IdentifierCase::SnakeCase => BaseEntityPathInclination::FunctionOrLocalValue,
            _ => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum Expr {
    Literal(TokenIdx),
    EntityPath {
        entity_path_expr: EntityPathExprIdx,
        entity_path: Option<EntityPath>,
    },
    InheritedSymbol {
        ident: Identifier,
        token_idx: TokenIdx,
        inherited_symbol_idx: InheritedSymbolIdx,
        inherited_symbol_kind: InheritedSymbolKind,
    },
    CurrentSymbol {
        ident: Identifier,
        token_idx: TokenIdx,
        current_symbol_idx: CurrentSymbolIdx,
        current_symbol_kind: CurrentSymbolKind,
    },
    FrameVarDecl {
        token_idx: TokenIdx,
        ident: Identifier,
        frame_var_symbol_idx: CurrentSymbolIdx,
        current_symbol_kind: CurrentSymbolKind,
    },
    SelfType(TokenIdx),
    SelfValue(TokenIdx),
    BinaryOpn {
        lopd: ExprIdx,
        opr: BinaryOpr,
        opr_token_idx: TokenIdx,
        ropd: ExprIdx,
    },
    Be {
        src: ExprIdx,
        be_token_idx: TokenIdx,
        target: ExprResult<BeVariableDeclPattern>,
    },
    PrefixOpn {
        opr: PrefixOpr,
        opr_token_idx: TokenIdx,
        opd: ExprIdx,
    },
    SuffixOpn {
        opd: ExprIdx,
        opr: SuffixOpr,
        opr_token_idx: TokenIdx,
    },
    ApplicationOrRitchieCall {
        function: ExprIdx,
        lpar_token_idx: TokenIdx,
        argument: ExprIdx,
        rpar_token_idx: TokenIdx,
    },
    RitchieCall {
        function: ExprIdx,
        implicit_arguments: Option<ImplicitArgumentList>,
        lpar_token_idx: TokenIdx,
        arguments: ExprIdxRange,
        rpar_token_idx: TokenIdx,
    },
    Field {
        owner: ExprIdx,
        dot_token_idx: TokenIdx,
        ident_token: IdentifierToken,
    },
    MethodCall {
        self_argument: ExprIdx,
        dot_token_idx: TokenIdx,
        ident_token: IdentifierToken,
        implicit_arguments: Option<ImplicitArgumentList>,
        lpar_token_idx: TokenIdx,
        nonself_arguments: ExprIdxRange,
        rpar_token_idx: TokenIdx,
    },
    TemplateInstantiation {
        template: ExprIdx,
        implicit_arguments: ImplicitArgumentList,
    },
    Application {
        function: ExprIdx,
        argument: ExprIdx,
    },
    Bracketed {
        lpar_token_idx: TokenIdx,
        item: ExprIdx,
        rpar_token_idx: TokenIdx,
    },
    NewTuple {
        lpar_token_idx: TokenIdx,
        items: ExprIdxRange,
        commas: Vec<TokenIdx>,
        rpar_token_idx: TokenIdx,
    },
    NewBoxList {
        caller: Option<ExprIdx>,
        lbox_token_idx: TokenIdx,
        items: ExprIdxRange,
        rbox_token_idx: TokenIdx,
    },
    BoxColon {
        caller: Option<ExprIdx>,
        lbox_token_idx: TokenIdx,
        colon_token_idx: TokenIdx,
        rbox_token: RightBoxBracketToken,
    },
    Block {
        stmts: StmtIdxRange,
    },
    Err(ExprError),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ImplicitArgumentList {
    langle: TokenIdx,
    arguments: ExprIdxRange,
    rangle: TokenIdx,
}

impl ImplicitArgumentList {
    pub(crate) fn new(langle: TokenIdx, arguments: ExprIdxRange, rangle: TokenIdx) -> Self {
        Self {
            langle,
            arguments,
            rangle,
        }
    }

    pub fn langle(&self) -> TokenIdx {
        self.langle
    }

    pub fn arguments(&self) -> &ExprIdxRange {
        &self.arguments
    }

    pub fn rangle(&self) -> TokenIdx {
        self.rangle
    }
}

use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};

pub type ExprArena = Arena<Expr>;
pub type ExprIdx = ArenaIdx<Expr>;
pub type ExprIdxRange = ArenaIdxRange<Expr>;
pub type ExprMap<V> = ArenaMap<Expr, V>;
