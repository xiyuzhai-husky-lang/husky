use super::*;
use husky_cybertron::seq::any::AnySeq;
use std::collections::HashMap;

pub(super) fn reduce_asts_by_opr(
    pre_asts: Seq<Option<PreAst>>,
    allocated_asts: Seq<Option<AstData>>,
) -> (Seq<Option<PreAst>>, Seq<Option<AstData>>) {
    let pre_asts_nearest_left2 = pre_asts.nearest_left2();
    let pre_asts_nearest_right2 = pre_asts.nearest_right2();
    let new_opr_asts = new_opr_ast.apply(pre_asts_nearest_left2, pre_asts, pre_asts_nearest_right2);
    let (pre_asts, new_parent_idxs) = reduce_pre_asts_by_opr(pre_asts, new_opr_asts);
    let pre_asts = add_pre_asts(pre_asts, new_opr_asts);
    let allocated_asts = allocate_asts(allocated_asts, new_opr_asts);
    (pre_asts, allocated_asts)
}

#[test]
fn reduce_asts_by_opr_works() {
    fn t(input: &str, expect: Expect) {
        let toks = tokenize(input);
        let pre_asts = calc_pre_ast_initial_seq(toks);
        let mut seqs: HashMap<String, AnySeq> = Default::default();
        seqs.insert("pre_asts".into(), pre_asts.into());
        let allocated_asts: Seq<Option<AstData>> = (|_| None).apply(toks);
        let (pre_asts1, allocated_asts1) = reduce_asts_by_opr(pre_asts, allocated_asts);
        seqs.insert("pre_asts1".into(), pre_asts1.into());
        seqs.insert("allocated_asts1".into(), allocated_asts1.into());
        expect.assert_debug_eq(&seqs)
    }
    t(
        "hello",
        expect![[r#"
            {
                "pre_asts1": [None],
                "allocated_asts1": [None],
                "pre_asts": [Some(Ident(i`hello`))],
            }
        "#]],
    );
    t(
        "1",
        expect![[r#"
            {
                "pre_asts": [Some(Literal(Int(1)))],
                "pre_asts1": [None],
                "allocated_asts1": [None],
            }
        "#]],
    );
    t(
        "1+1",
        expect![[r#"
            {
                "allocated_asts1": [None, Some(Binary { lopd: Idx(0), ropd: Idx(2) }), None],
                "pre_asts": [Some(Literal(Int(1))), Some(Binary(Add)), Some(Literal(Int(1)))],
                "pre_asts1": [None, Some(Binary { lopd: Idx(0), ropd: Idx(2) }), None],
            }
        "#]],
    );
}

/// a finite function
pub(crate) fn new_opr_ast(
    nearest_left2: Option2<(Idx, PreAst)>,
    current: Option<PreAst>,
    nearest_right2: Option2<(Idx, PreAst)>,
) -> Option<AstData> {
    let Some(PreAst::Opr(opr)) = current else {
        return None;
    };
    match opr {
        Opr::Binary(_) => {
            let Some((lopd, PreAst::Ast(_))) = nearest_left2.first() else {
                return None;
            };
            let Some((ropd, PreAst::Ast(_))) = nearest_right2.first() else {
                return None;
            };
            // todo: check precedence
            Some(AstData::Binary { lopd, ropd })
        }
    }
}

/// returns sequence of remaining PreAsts and new parent idxs
pub(crate) fn reduce_pre_asts_by_opr(
    pre_asts: Seq<Option<PreAst>>,
    new_asts: Seq<Option<AstData>>,
) -> (Seq<Option<PreAst>>, Seq<Option<Idx>>) {
    let new_asts_nearest_left = new_asts.nearest_left();
    let (pre_asts, new_parents) = reduce_pre_ast_by_opr_left
        .apply_enumerated(new_asts_nearest_left, pre_asts)
        .decouple();
    let new_asts_nearest_right = new_asts.nearest_right();
    reduce_pre_ast_by_opr_right
        .apply_enumerated(new_asts_nearest_left, pre_asts, new_parents)
        .decouple()
}

fn reduce_pre_ast_by_opr_left(
    idx: Idx,
    new_ast_nearest_left: Option<(Idx, AstData)>,
    pre_ast: Option<PreAst>,
) -> (Option<PreAst>, Option<Idx>) {
    let Some(pre_ast) = pre_ast else {
        return (None, None);
    };
    let Some((new_ast_idx, new_ast_data)) = new_ast_nearest_left else {
        return (None, None);
    };
    match new_ast_data {
        AstData::Literal(_) | AstData::Ident(_) => unreachable!(),
        AstData::Binary { lopd, ropd } => {
            if ropd == idx {
                (None, Some(new_ast_idx))
            } else {
                (Some(pre_ast), None)
            }
        }
        AstData::LetInit => todo!(),
    }
}

fn reduce_pre_ast_by_opr_right(
    idx: Idx,
    new_ast_nearest_left: Option<(Idx, AstData)>,
    pre_ast: Option<PreAst>,
    new_parent: Option<Idx>,
) -> (Option<PreAst>, Option<Idx>) {
    let Some(pre_ast) = pre_ast else {
        return (None, None);
    };
    if let Some(new_parent) = new_parent {
        return (None, Some(new_parent));
    }
    let Some((new_ast_idx, new_ast_data)) = new_ast_nearest_left else {
        return (None, None);
    };
    match new_ast_data {
        AstData::Literal(_) | AstData::Ident(_) => unreachable!(),
        AstData::Binary { lopd, ropd } => {
            if lopd == idx {
                (None, Some(new_ast_idx))
            } else {
                (Some(pre_ast), None)
            }
        }
        AstData::LetInit => todo!(),
    }
}

fn add_pre_asts(
    pre_asts: Seq<Option<PreAst>>,
    new_asts: Seq<Option<AstData>>,
) -> Seq<Option<PreAst>> {
    add_pre_ast.apply(pre_asts, new_asts)
}

fn add_pre_ast(pre_ast: Option<PreAst>, new_ast: Option<AstData>) -> Option<PreAst> {
    pre_ast.or(new_ast.map(PreAst::Ast))
}
