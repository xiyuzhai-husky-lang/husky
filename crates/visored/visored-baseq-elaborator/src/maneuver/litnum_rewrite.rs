use super::*;
use crate::{
    coercion::VdBsqCoercionOutcome,
    elabm::ElabM,
    expr::{VdBsqExprFld, VdBsqExprFldData},
    hypothesis::construction::VdBsqHypothesisConstruction,
    Mhr,
};
use alt_option::*;
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
        match elaborator
            .hypothesis_constructor
            .stack()
            .get_active_litnum_equality(expr)
        {
            Some(litnum) => todo!(),
            None => rewrite_subexprs(expr).eval(elaborator, heuristic),
        }
    }
}

fn rewrite_subexprs<'db, 'sess>(
    expr: VdBsqExprFld<'sess>,
) -> impl ElabM<'db, 'sess, VdBsqExprFld<'sess>>
where
    'db: 'sess,
{
    // move |_: &mut VdBsqElaboratorInner<'db, 'sess>,
    //       _: &dyn Fn(&mut VdBsqElaboratorInner<'db, 'sess>, VdBsqExprFld<'sess>) -> Mhr<'sess>| todo!()
    #[unify_elabm]
    match expr.data() {
        VdBsqExprFldData::Literal(vd_literal) => todo!(),
        VdBsqExprFldData::Variable(lx_math_letter, arena_idx) => todo!(),
        VdBsqExprFldData::Application {
            function,
            arguments,
        } => todo!(),
        VdBsqExprFldData::FoldingSeparatedList { leader, followers } => todo!(),
        VdBsqExprFldData::ChainingSeparatedList {
            leader,
            followers,
            joined_signature,
        } => todo!(),
        VdBsqExprFldData::ItemPath(vd_item_path) => todo!(),
    }
}
