use super::*;
use crate::{
    coercion::VdBsqCoercionOutcome,
    elabm::{foldm::mapm_collect, ElabM},
    expr::{VdBsqExprFld, VdBsqExprFldData},
    hypothesis::construction::VdBsqHypothesisConstruction,
    Mhr,
};
use alt_option::*;
use expr::VdBsqExprFollowers;
use smallvec::SmallVec;
use visored_baseq_elaborator_macros::unify_elabm;
use visored_entity_path::{
    path::{
        set::{VdPreludeSetPath, VdSetPath},
        VdItemPath,
    },
    theorem::VdTheoremPath,
};
use visored_mir_expr::expr::application::VdMirFunc;
use visored_mir_opr::separator::VdMirBaseSeparator;
use visored_term::term::VdTerm;

pub(crate) fn litnum_rewritem<'db, 'sess>(
    expr: VdBsqExprFld<'sess>,
) -> impl ElabM<'db, 'sess, VdBsqExprFld<'sess>>
where
    'db: 'sess,
{
    VdBsqManeuverCall::LitnumRewrite.wrap(litnum_rewrite_inner(expr))
}

fn litnum_rewrite_inner<'db, 'sess>(
    expr: VdBsqExprFld<'sess>,
) -> impl ElabM<'db, 'sess, VdBsqExprFld<'sess>>
where
    'db: 'sess,
{
    move |elaborator: &mut Elr<'db, 'sess>,
          heuristic: &dyn Fn(
        &mut VdBsqElaboratorInner<'db, 'sess>,
        VdBsqExprFld<'sess>,
    ) -> Mhr<'sess>| {
        let db = elaborator.floater_db();
        match elaborator
            .hypothesis_constructor
            .stack()
            .get_active_litnum_equality(expr, db)
        {
            Some(litnum) => todo!(),
            None => rewrite_subexprs(expr, litnum_rewrite_inner).eval(elaborator, heuristic),
        }
    }
}

fn rewrite_subexprs<'a, 'db, 'sess, MExpr>(
    expr: VdBsqExprFld<'sess>,
    f: impl Fn(VdBsqExprFld<'sess>) -> MExpr + Copy + 'a,
) -> impl ElabM<'db, 'sess, VdBsqExprFld<'sess>> + 'a
where
    'db: 'sess + 'a,
    'sess: 'a,
    MExpr: ElabM<'db, 'sess, VdBsqExprFld<'sess>> + 'a,
{
    #[unify_elabm]
    match *expr.data() {
        VdBsqExprFldData::Literal(vd_literal) => todo!(),
        VdBsqExprFldData::Variable(lx_math_letter, arena_idx) => todo!(),
        VdBsqExprFldData::Application {
            function,
            ref arguments,
        } => todo!(),
        VdBsqExprFldData::FoldingSeparatedList {
            leader,
            ref followers,
        } => todo!(),
        VdBsqExprFldData::ChainingSeparatedList {
            leader,
            ref followers,
            joined_signature,
        } => f(leader).bind(|elr, leader| {
            mapm_collect(followers, |&(func, follower)| {
                f(follower).map(move |elr, follower| (func, follower))
            })
            .map(|elr, followers: VdBsqExprFollowers<'sess>| todo!())
        }),
        VdBsqExprFldData::ItemPath(vd_item_path) => todo!(),
    }
}
