use super::defn::calc_symbol_defns;
use super::*;

pub enum SymbolResolution {}

fn calc_symbol_resolution(asts: Seq<Option<Ast>>, n: usize) -> Seq<Option<Symbol>> {
    let symbol_defns = calc_symbol_defns(asts, n);
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
fn calc_symbol_resoluion_works() {
    fn t(input: &str, expect: Expect) {
        let asts = calc_asts_from_input(input, 10);
        let symbol_resolutions = calc_symbol_resolution(asts, 10);
        expect.assert_debug_eq(&symbol_resolutions)
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
                Some(Symbol { ident: `f`, source: #0, data: Item { kind: Fn } }),
                None,
                None,
                None,
                None,
                None,
                Some(Symbol { ident: `g`, source: #6, data: Item { kind: Fn } }),
                None,
                None,
                None,
                Some(Symbol { ident: `f`, source: #0, data: Item { kind: Fn } }),
                None,
                None,
                None,
            ]
        "#]],
    );
}
