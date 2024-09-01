use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SymbolDefn {
    pub symbol: Symbol,
}

pub fn calc_symbol_defns(asts: Seq<Option<Ast>>) -> Seq<Option<SymbolDefn>> {
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
