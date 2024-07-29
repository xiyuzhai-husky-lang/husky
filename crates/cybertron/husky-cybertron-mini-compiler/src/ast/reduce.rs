use super::*;
use husky_cybertron::{
    debug::{is_debug, set_debug},
    prelude::*,
    seq::any::AnySeq,
};
use husky_print_utils::p;
use indexmap::IndexMap;

pub(super) fn reduce_asts_by_opr(
    pre_asts: Seq<Option<PreAst>>,
    allocated_asts: Seq<Option<AstData>>,
) -> (Seq<Option<PreAst>>, Seq<Option<AstData>>) {
    use husky_print_utils::p;
    let pre_asts_nearest_left2 = pre_asts.nearest_left2();
    let pre_asts_nearest_right2 = pre_asts.nearest_right2();
    let new_opr_asts = new_opr_ast.apply(pre_asts_nearest_left2, pre_asts, pre_asts_nearest_right2);
    let (pre_asts_reduced, new_parent_idxs) = reduce_pre_asts_by_opr(pre_asts, new_opr_asts);
    p!(new_opr_asts, pre_asts_reduced, new_parent_idxs);
    let pre_asts = add_pre_asts(pre_asts_reduced, new_opr_asts);
    // p!(pre_asts, new_parent_idxs);
    let allocated_asts = allocate_asts(allocated_asts, new_opr_asts);
    (pre_asts, allocated_asts)
}

#[test]
fn reduce_asts_by_opr_works1() {
    let pre_asts = seq![
        Some(PreAst::Ast(AstData::Literal(Literal::Int(1)))),
        Some(PreAst::Opr(Opr::Binary(BinaryOpr::Add))),
        Some(PreAst::Ast(AstData::Literal(Literal::Int(1)))),
    ];
    let allocated_asts = seq![
        Some(AstData::Literal(Literal::Int(1))),
        None,
        Some(AstData::Literal(Literal::Int(1))),
    ];
    let (pre_asts1, allocated_asts1) = reduce_asts_by_opr(pre_asts, allocated_asts);
    assert_eq!(
        pre_asts1,
        seq![
            None,
            Some(PreAst::Ast(AstData::Binary {
                lopd: Idx::new(0),
                opr: BinaryOpr::Add,
                ropd: Idx::new(2),
            })),
            None
        ]
    );
    // assert_eq!(allocated_asts1, seq![]);
}

#[test]
fn reduce_asts_by_opr_works2() {
    fn t(input: &str, expect: Expect) {
        let toks = tokenize(input);
        let pre_asts = calc_pre_ast_initial_seq(toks);
        let mut seqs: IndexMap<String, AnySeq> = Default::default();
        let allocated_asts: Seq<Option<AstData>> = toks.map(|tok| tok.into());
        seqs.insert("pre_asts".into(), pre_asts.into());
        seqs.insert("allocated_asts".into(), allocated_asts.into());
        let (pre_asts1, allocated_asts1) = reduce_asts_by_opr(pre_asts, allocated_asts);
        seqs.insert("pre_asts1".into(), pre_asts1.into());
        seqs.insert("allocated_asts1".into(), allocated_asts1.into());
        let (pre_asts2, allocated_asts2) = reduce_asts_by_opr(pre_asts1, allocated_asts1);
        seqs.insert("pre_asts2".into(), pre_asts2.into());
        seqs.insert("allocated_asts2".into(), allocated_asts2.into());
        let (pre_asts3, allocated_asts3) = reduce_asts_by_opr(pre_asts2, allocated_asts2);
        seqs.insert("pre_asts3".into(), pre_asts3.into());
        seqs.insert("allocated_asts3".into(), allocated_asts3.into());
        expect.assert_debug_eq(&seqs)
    }
    t(
        "hello",
        expect![[r#"
            {
                "pre_asts": [Some(Ident(`hello`))],
                "allocated_asts": [Some(Ident(`hello`))],
                "pre_asts1": [Some(Ident(`hello`))],
                "allocated_asts1": [Some(Ident(`hello`))],
                "pre_asts2": [Some(Ident(`hello`))],
                "allocated_asts2": [Some(Ident(`hello`))],
                "pre_asts3": [Some(Ident(`hello`))],
                "allocated_asts3": [Some(Ident(`hello`))],
            }
        "#]],
    );
    t(
        "1",
        expect![[r#"
            {
                "pre_asts": [Some(Literal(`1`))],
                "allocated_asts": [Some(Literal(`1`))],
                "pre_asts1": [Some(Literal(`1`))],
                "allocated_asts1": [Some(Literal(`1`))],
                "pre_asts2": [Some(Literal(`1`))],
                "allocated_asts2": [Some(Literal(`1`))],
                "pre_asts3": [Some(Literal(`1`))],
                "allocated_asts3": [Some(Literal(`1`))],
            }
        "#]],
    );
    t(
        "1+1",
        expect![[r#"
            {
                "pre_asts": [Some(Literal(`1`)), Some(`+`), Some(Literal(`1`))],
                "allocated_asts": [Some(Literal(`1`)), None, Some(Literal(`1`))],
                "pre_asts1": [None, Some(Binary { lopd: #0, opr: `+`, ropd: #2 }), None],
                "allocated_asts1": [
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                    Some(
                        Binary {
                            lopd: #0,
                            opr: `+`,
                            ropd: #2,
                        },
                    ),
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                ],
                "pre_asts2": [None, Some(Binary { lopd: #0, opr: `+`, ropd: #2 }), None],
                "allocated_asts2": [
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                    Some(
                        Binary {
                            lopd: #0,
                            opr: `+`,
                            ropd: #2,
                        },
                    ),
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                ],
                "pre_asts3": [None, Some(Binary { lopd: #0, opr: `+`, ropd: #2 }), None],
                "allocated_asts3": [
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                    Some(
                        Binary {
                            lopd: #0,
                            opr: `+`,
                            ropd: #2,
                        },
                    ),
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                ],
            }
        "#]],
    );
    t(
        "1+1+1",
        expect![[r#"
            {
                "pre_asts": [
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                    Some(
                        `+`,
                    ),
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                    Some(
                        `+`,
                    ),
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                ],
                "allocated_asts": [Some(Literal(`1`)), None, Some(Literal(`1`)), None, Some(Literal(`1`))],
                "pre_asts1": [
                    None,
                    Some(
                        Binary {
                            lopd: #0,
                            opr: `+`,
                            ropd: #2,
                        },
                    ),
                    None,
                    Some(
                        `+`,
                    ),
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                ],
                "allocated_asts1": [
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                    Some(
                        Binary {
                            lopd: #0,
                            opr: `+`,
                            ropd: #2,
                        },
                    ),
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                    None,
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                ],
                "pre_asts2": [None, None, None, Some(Binary { lopd: #1, opr: `+`, ropd: #4 }), None],
                "allocated_asts2": [
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                    Some(
                        Binary {
                            lopd: #0,
                            opr: `+`,
                            ropd: #2,
                        },
                    ),
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                    Some(
                        Binary {
                            lopd: #1,
                            opr: `+`,
                            ropd: #4,
                        },
                    ),
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                ],
                "pre_asts3": [None, None, None, Some(Binary { lopd: #1, opr: `+`, ropd: #4 }), None],
                "allocated_asts3": [
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                    Some(
                        Binary {
                            lopd: #0,
                            opr: `+`,
                            ropd: #2,
                        },
                    ),
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                    Some(
                        Binary {
                            lopd: #1,
                            opr: `+`,
                            ropd: #4,
                        },
                    ),
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                ],
            }
        "#]],
    );
    set_debug();
    t(
        "1+1*1",
        expect![[r#"
            {
                "pre_asts": [
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                    Some(
                        `+`,
                    ),
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                    Some(
                        `*`,
                    ),
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                ],
                "allocated_asts": [Some(Literal(`1`)), None, Some(Literal(`1`)), None, Some(Literal(`1`))],
                "pre_asts1": [
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                    Some(
                        `+`,
                    ),
                    None,
                    Some(
                        Binary {
                            lopd: #2,
                            opr: `*`,
                            ropd: #4,
                        },
                    ),
                    None,
                ],
                "allocated_asts1": [
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                    None,
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                    Some(
                        Binary {
                            lopd: #2,
                            opr: `*`,
                            ropd: #4,
                        },
                    ),
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                ],
                "pre_asts2": [None, Some(Binary { lopd: #0, opr: `+`, ropd: #3 }), None, None, None],
                "allocated_asts2": [
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                    Some(
                        Binary {
                            lopd: #0,
                            opr: `+`,
                            ropd: #3,
                        },
                    ),
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                    Some(
                        Binary {
                            lopd: #2,
                            opr: `*`,
                            ropd: #4,
                        },
                    ),
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                ],
                "pre_asts3": [None, Some(Binary { lopd: #0, opr: `+`, ropd: #3 }), None, None, None],
                "allocated_asts3": [
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                    Some(
                        Binary {
                            lopd: #0,
                            opr: `+`,
                            ropd: #3,
                        },
                    ),
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                    Some(
                        Binary {
                            lopd: #2,
                            opr: `*`,
                            ropd: #4,
                        },
                    ),
                    Some(
                        Literal(
                            `1`,
                        ),
                    ),
                ],
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
        Opr::Binary(opr) => {
            let Some((lopd, PreAst::Ast(_))) = nearest_left2.first() else {
                return None;
            };
            let Some((ropd, PreAst::Ast(_))) = nearest_right2.first() else {
                return None;
            };
            if let Some((_, ast)) = nearest_left2.second() {
                match ast {
                    PreAst::Keyword(_) => (),
                    PreAst::Opr(left_opr) => match left_opr {
                        Opr::Binary(left_opr) => {
                            /// every binary opr in our small language is left associative, so `>=` instead of `>`
                            if left_opr.precedence() >= opr.precedence() {
                                return None;
                            }
                        }
                    },
                    PreAst::Ast(_) => (),
                }
            };
            if let Some((_, ast)) = nearest_right2.second() {
                match ast {
                    PreAst::Keyword(_) => (),
                    PreAst::Opr(right_opr) => match right_opr {
                        Opr::Binary(right_opr) => {
                            /// every binary opr in our small language is left associative, so `<` instead of `<=`
                            if right_opr.precedence() > opr.precedence() {
                                return None;
                            }
                        }
                    },
                    PreAst::Ast(_) => (),
                }
            };
            // todo: check precedence
            Some(AstData::Binary { lopd, opr, ropd })
        }
    }
}

/// returns sequence of remaining PreAsts and new parent idxs
pub(crate) fn reduce_pre_asts_by_opr(
    pre_asts: Seq<Option<PreAst>>,
    new_asts: Seq<Option<AstData>>,
) -> (Seq<Option<PreAst>>, Seq<Option<Idx>>) {
    let new_asts_nearest_left = new_asts.nearest_left();
    let pre_asts = reduce_pre_ast_by_new_ast.apply(pre_asts, new_asts);
    let (pre_asts, new_parents) = reduce_pre_ast_by_opr_left
        .apply_enumerated(new_asts_nearest_left, pre_asts)
        .decouple();
    let new_asts_nearest_right = new_asts.nearest_right();
    reduce_pre_ast_by_opr_right
        .apply_enumerated(new_asts_nearest_right, pre_asts, new_parents)
        .decouple()
}

fn reduce_pre_ast_by_new_ast(pre_ast: Option<PreAst>, new_ast: Option<AstData>) -> Option<PreAst> {
    if new_ast.is_some() {
        None
    } else {
        pre_ast
    }
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
        return (Some(pre_ast), None);
    };
    match new_ast_data {
        AstData::Literal(_) | AstData::Ident(_) => unreachable!(),
        AstData::Binary { ropd, .. } => {
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
    new_ast_nearest_right: Option<(Idx, AstData)>,
    pre_ast: Option<PreAst>,
    new_parent: Option<Idx>,
) -> (Option<PreAst>, Option<Idx>) {
    let Some(pre_ast) = pre_ast else {
        return (None, None);
    };
    if let Some(new_parent) = new_parent {
        return (None, Some(new_parent));
    }
    let Some((new_ast_idx, new_ast_data)) = new_ast_nearest_right else {
        return (Some(pre_ast), None);
    };
    match new_ast_data {
        AstData::Literal(_) | AstData::Ident(_) => unreachable!(),
        AstData::Binary { lopd, .. } => {
            if lopd == idx {
                (None, Some(new_ast_idx))
            } else {
                (Some(pre_ast), None)
            }
        }
        AstData::LetInit => todo!(),
    }
}

#[test]
fn reduce_pre_ast_by_opr_right_works() {
    assert_eq!(
        reduce_pre_ast_by_opr_right(
            Idx::new(0),
            Some((
                Idx::new(1),
                AstData::Binary {
                    lopd: Idx::new(0),
                    opr: BinaryOpr::Add,
                    ropd: Idx::new(2),
                }
            )),
            Some(PreAst::Ast(AstData::Literal(Literal::Int(1)))),
            None,
        ),
        (None, Some(Idx::new(1)))
    );
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
