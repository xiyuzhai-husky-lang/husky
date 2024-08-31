use crate::{
    ast::{Ast, AstData},
    token::{
        ident::Ident,
        keyword::{DefnKeyword, Keyword},
    },
    *,
};
use ast::calc_asts_from_input;
use husky_cybertron::{prelude::Idx, seq::Seq};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SymbolDefn {
    symbol: Symbol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Symbol {
    ident: Ident,
    source: Idx,
    data: SymbolData,
}

impl Symbol {
    pub fn ident(&self) -> Ident {
        self.ident
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolData {
    Item { kind: ItemKind },
    Variable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemKind {
    Struct,
    Enum,
    Fn,
}

impl From<DefnKeyword> for ItemKind {
    fn from(kw: DefnKeyword) -> Self {
        match kw {
            DefnKeyword::Struct => ItemKind::Struct,
            DefnKeyword::Enum => ItemKind::Enum,
            DefnKeyword::Fn => ItemKind::Fn,
        }
    }
}

fn calc_symbol_defns(asts: Seq<Option<Ast>>) -> Seq<Option<SymbolDefn>> {
    asts.map(|ast| {
        match ast?.data {
            AstData::LetInit {
                expr,
                pattern,
                initial_value,
            } => todo!(),
            AstData::Defn {
                keyword,
                ident_idx,
                ident,
                content,
            } => Some(Symbol {
                ident,
                source: ident_idx,
                data: SymbolData::Item {
                    kind: keyword.into(),
                },
            }),
            _ => None,
        }
        .map(|symbol| SymbolDefn { symbol })
    })
}

#[test]
fn calc_symbol_defns_works() {
    fn t(input: &str, expect: Expect) {
        let asts = calc_asts_from_input(input, 10);
        let symbols = calc_symbol_defns(asts);
        expect.assert_debug_eq(&symbols)
    }
    t(
        "1",
        expect![[r#"
        [None]
    "#]],
    );
    t(
        "struct A {}",
        expect![[r#"
            [
                Some(SymbolDefn { symbol: Symbol { ident: `A`, source: #1, data: Item { kind: Struct } } }),
                None,
                None,
                None,
            ]
        "#]],
    );
    t(
        "enum B {}",
        expect![[r#"
            [
                Some(SymbolDefn { symbol: Symbol { ident: `B`, source: #1, data: Item { kind: Enum } } }),
                None,
                None,
                None,
            ]
        "#]],
    );
    t(
        "fn f() {}",
        expect![[r#"
            [
                Some(SymbolDefn { symbol: Symbol { ident: `f`, source: #1, data: Item { kind: Fn } } }),
                None,
                None,
                None,
                None,
                None,
            ]
        "#]],
    );
}

fn calc_symbol_lookup(asts: Seq<Option<Ast>>) -> Seq<Option<Symbol>> {
    let symbol_defns = calc_symbol_defns(asts);
    let idents = asts.map(|ast| match ast?.data {
        AstData::Ident(ident) => Some(ident),
        _ => None,
    });
    symbol_defns
        .map(|symbol_defn| Some(symbol_defn?.symbol))
        .first_filtered_by_attention(idents, symbol_defns, |ident, symbol_defn| {
            let Some(ident) = ident else { return false };
            let Some(symbol_defn) = symbol_defn else {
                return false;
            };
            // todo: consider scope
            symbol_defn.symbol.ident == ident
        })
        .map(|s| s.flatten())
}

#[test]
fn calc_symbol_lookup_works() {
    fn t(input: &str, expect: Expect) {
        let asts = calc_asts_from_input(input, 10);
        let symbol_lookups = calc_symbol_lookup(asts);
        expect.assert_debug_eq(&symbol_lookups)
    }
    t(
        "1",
        expect![[r#"
        [None]
    "#]],
    );
    t(
        "fn f() {} fn g() { f() }",
        expect![[r#"
            [
                None,
                Some(Symbol { ident: `f`, source: #1, data: Item { kind: Fn } }),
                None,
                None,
                None,
                None,
                None,
                Some(Symbol { ident: `g`, source: #7, data: Item { kind: Fn } }),
                None,
                None,
                None,
                Some(Symbol { ident: `f`, source: #1, data: Item { kind: Fn } }),
                None,
                None,
                None,
            ]
        "#]],
    );
}
