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

fn calc_symbols(asts: Seq<Option<Ast>>) -> Seq<Option<Symbol>> {
    asts.map(|ast| match ast?.data {
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
    })
}

#[test]
fn calc_symbols_works() {
    fn t(input: &str, expect: Expect) {
        let (_, asts) = calc_asts_from_input(input, 10);
        let symbols = calc_symbols(asts);
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
                Some(Symbol { ident: `A`, source: #1, data: Item { kind: Struct } }),
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
                Some(Symbol { ident: `B`, source: #1, data: Item { kind: Enum } }),
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
                Some(Symbol { ident: `f`, source: #1, data: Item { kind: Fn } }),
                None,
                None,
                None,
                None,
                None,
            ]
        "#]],
    );
}
