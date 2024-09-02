use show::{show_asts, show_asts_mapped_values};

use super::*;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Rank(u8);

impl Rank {
    fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

pub fn calc_ranks(asts: Seq<Option<Ast>>, n: usize) -> Seq<Option<Rank>> {
    let mut ranks: Seq<Option<Rank>> = asts.map(|_| None);
    for _ in 0..n {
        ranks = calc_sibling_indicies_step(asts, ranks)
    }
    ranks
}

fn calc_sibling_indicies_step(
    asts: Seq<Option<Ast>>,
    ranks: Seq<Option<Rank>>,
) -> Seq<Option<Rank>> {
    (|rank, previous_rank: Option<(Idx, Option<Rank>)>| {
        if let Some(rank) = rank {
            return Some(rank);
        }
        let Some((idx, Some(previous_rank))) = previous_rank else {
            return Some(Default::default());
        };
        Some(previous_rank.next())
    })
    .apply(
        ranks,
        ranks.first_filtered_by_attention_enumerated(asts, asts, |ast, ast1| {
            let Some(ast) = ast else { return false };
            let Some(ast1) = ast1 else { return false };
            ast.parent == ast1.parent
        }),
    )
}

#[test]
fn calc_ranks_works() {
    fn t(input: &str, n: usize, expect: Expect) {
        let (tokens, pre_asts, asts) =
            calc_asts_from_input_together_with_tokens_and_pre_asts(input, n);
        let ranks = calc_ranks(asts, 10);
        expect.assert_debug_eq(&show_asts_mapped_values(tokens, pre_asts, asts, ranks));
    }
    t(
        "1",
        10,
        expect![[r#"
        [
            #0 `1`: "1" ✓ → Rank(0),
        ]
    "#]],
    );
    t(
        "1+1",
        10,
        expect![[r#"
        [
            #0 `1`: "1" → Rank(0),
            #1 `+`: "1 + 1" ✓ → Rank(0),
            #2 `1`: "1" → Rank(0),
        ]
    "#]],
    );
    t(
        "1+1*2",
        10,
        expect![[r#"
        [
            #0 `1`: "1" → Rank(0),
            #1 `+`: "1 + 1 * 2" ✓ → Rank(0),
            #2 `1`: "1" → Rank(0),
            #3 `*`: "1 * 2" → Rank(0),
            #4 `2`: "2" → Rank(0),
        ]
    "#]],
    );
    t(
        "fn f() { let x = 1; let y = g(x, 1, 2); }",
        10,
        expect![[r#"
            [
                #0 `fn`: "fn f() { let x = 1; let y = g(x, 1, 2);  }" ✓ → Rank(0),
                #1 `f`: "f" → Rank(0),
                #2 `(`: `(` → Rank(0),
                #3 `)`: "()" → Rank(0),
                #4 `{`: "() { let x = 1; let y = g(x, 1, 2);  }" → Rank(0),
                #5 `let`: "let x = 1" → Rank(0),
                #6 `x`: "x" → Rank(0),
                #7 `=`: "x = 1" → Rank(0),
                #8 `1`: "1" → Rank(0),
                #9 `;`: "let x = 1; " → Rank(0),
                #10 `let`: "let y = g(x, 1, 2)" → Rank(0),
                #11 `y`: "y" → Rank(0),
                #12 `=`: "y = g(x, 1, 2)" → Rank(0),
                #13 `g`: "g" → Rank(0),
                #14 `(`: "g(x, 1, 2)" → Rank(0),
                #15 `x`: "x" → Rank(0),
                #16 `,`: "x, " → Rank(0),
                #17 `1`: "1" → Rank(0),
                #18 `,`: "1, " → Rank(0),
                #19 `2`: "2" → Rank(0),
                #20 `)`: "(x, 1, 2)" → Rank(0),
                #21 `;`: "let y = g(x, 1, 2); " → Rank(0),
                #22 `}`: "{ let x = 1; let y = g(x, 1, 2);  }" → Rank(0),
            ]
        "#]],
    );
}
