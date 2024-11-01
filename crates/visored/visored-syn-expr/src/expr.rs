pub mod attach;
pub mod binary;
pub mod list_item;
pub mod literal;
pub mod notation;
pub mod prefix;
pub mod suffix;
pub mod uniadic_array;
pub mod uniadic_chain;
pub mod variadic_array;
pub mod variadic_chain;

use crate::builder::{ToVdSyn, VdSynExprBuilder};
use crate::*;
use either::*;
use error::{OriginalVdSynExprError, VdSynExprError};
use idx_arena::{
    map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef,
};
use latex_ast::ast::math::{LxMathAstIdx, LxMathAstIdxRange};
use latex_prelude::script::LxScriptKind;
use latex_token::idx::{LxMathTokenIdx, LxTokenIdxRange};
use range::VdSynExprTokenIdxRange;
use visored_opr::{
    delimiter::{
        VdBaseLeftDelimiter, VdBaseRightDelimiter, VdCompositeLeftDelimiter,
        VdCompositeRightDelimiter,
    },
    opr::{binary::VdBaseBinaryOpr, prefix::VdBasePrefixOpr, suffix::VdBaseSuffixOpr, VdBaseOpr},
    separator::{VdBaseSeparator, VdCompositeSeparator, VdSeparator},
};
use visored_zfc_ty::term::literal::VdZfcLiteral;

/// It's a tree of both form and meaning
#[derive(Debug, PartialEq, Eq)]
pub enum VdSynExprData {
    Literal {
        token_idx_range: LxTokenIdxRange,
        literal: VdZfcLiteral,
    },
    Notation,
    BaseOpr {
        opr: VdBaseOpr,
    },
    Binary {
        lopd: VdSynExprIdx,
        opr: Either<VdBaseBinaryOpr, VdSynExprIdx>,
        ropd: VdSynExprIdx,
    },
    Prefix {
        opr: Either<VdBasePrefixOpr, VdSynExprIdx>,
        opd: VdSynExprIdx,
    },
    Suffix {
        opd: VdSynExprIdx,
        opr: Either<VdBaseSuffixOpr, VdSynExprIdx>,
    },
    SeparatedList {
        separator: VdSeparator,
        fragments: SmallVec<[Either<VdSynExprIdx, VdSynSeparator>; 4]>,
    },
    Attach {
        base: VdSynExprIdx,
        // INVARIANCE: at least one of these are some
        scripts: Vec<(LxScriptKind, VdSynExprIdx)>,
    },
    UniadicChain,
    VariadicChain,
    UniadicArray,
    VariadicArray,
    Err(VdSynExprError),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSynSeparator {
    Base(LxMathTokenIdx, VdBaseSeparator),
    Composite(VdSynExprIdx, VdCompositeSeparator),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSynLeftDelimiter {
    Base(VdBaseLeftDelimiter),
    Composite(VdSynExprIdx, VdCompositeLeftDelimiter),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSynRightDelimiter {
    Base(VdBaseRightDelimiter),
    Composite(VdSynExprIdx, VdCompositeRightDelimiter),
}

pub type VdSynExprIdx = ArenaIdx<VdSynExprData>;
pub type VdSynExprIdxRange = ArenaIdxRange<VdSynExprData>;
pub type VdSynExprArena = Arena<VdSynExprData>;
pub type VdSynExprMap<T> = ArenaMap<VdSynExprData, T>;
pub type VdSynExprOrderedMap<T> = ArenaOrderedMap<VdSynExprData, T>;
pub type VdSynExprArenaRef<'a> = ArenaRef<'a, VdSynExprData>;

impl VdSynExprData {
    pub fn children(&self) -> Vec<VdSynExprIdx> {
        match *self {
            VdSynExprData::Literal { .. } => vec![],
            VdSynExprData::Notation => vec![],
            VdSynExprData::BaseOpr { opr } => vec![],
            VdSynExprData::Binary { lopd, opr, ropd } => match opr {
                Left(_) => vec![lopd, ropd],
                Right(opr) => vec![lopd, opr, ropd],
            },
            VdSynExprData::Prefix { opr, opd } => match opr {
                Left(_) => vec![opd],
                Right(opr) => vec![opr, opd],
            },
            VdSynExprData::Suffix { opd, opr } => match opr {
                Left(_) => vec![opd],
                Right(opr) => vec![opd, opr],
            },
            VdSynExprData::Attach { base, ref scripts } => [base]
                .into_iter()
                .chain(scripts.iter().map(|&(_, script)| script))
                .collect(),
            // ad hoc
            VdSynExprData::UniadicChain => vec![],
            // ad hoc
            VdSynExprData::VariadicChain => vec![],
            // ad hoc
            VdSynExprData::UniadicArray => vec![],
            // ad hoc
            VdSynExprData::VariadicArray => vec![],
            VdSynExprData::Err(..) => vec![],
            VdSynExprData::SeparatedList { ref fragments, .. } => fragments
                .iter()
                .filter_map(|fragment| match *fragment {
                    Left(expr) | Right(VdSynSeparator::Composite(expr, _)) => Some(expr),
                    Right(VdSynSeparator::Base(_, _)) => None,
                })
                .collect(),
        }
    }

    pub fn class(&self) -> VdSynExprClass {
        match *self {
            VdSynExprData::Literal {
                token_idx_range,
                literal,
            } => VdSynExprClass::Atom,
            VdSynExprData::Notation => todo!(),
            VdSynExprData::BaseOpr { opr } => todo!(),
            VdSynExprData::Binary { lopd, opr, ropd } => todo!(),
            VdSynExprData::Prefix { opr, opd } => todo!(),
            VdSynExprData::Suffix { opd, opr } => todo!(),
            VdSynExprData::Attach { base, ref scripts } => todo!(),
            VdSynExprData::UniadicChain => todo!(),
            VdSynExprData::VariadicChain => todo!(),
            VdSynExprData::UniadicArray => todo!(),
            VdSynExprData::VariadicArray => todo!(),
            VdSynExprData::Err(..) => todo!(),
            VdSynExprData::SeparatedList { .. } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSynExprClass {
    Atom,
    Prefix,
    Suffix,
    Separator,
}

// token idx range is needed because the ast idx range might be empty,
// in which case we need to return an error yet can't determine the token idx range from the ast idx range alone
impl ToVdSyn<VdSynExprIdx> for (LxTokenIdxRange, LxMathAstIdxRange) {
    fn to_vd_syn(self, builder: &mut VdSynExprBuilder) -> VdSynExprIdx {
        let (token_range, asts) = self;
        if asts.is_empty() {
            builder.alloc_expr(VdSynExprData::Err(
                OriginalVdSynExprError::Empty(token_range).into(),
            ))
        } else {
            let parser = builder.parser();
            parser.parse_asts(asts)
        }
    }
}

impl ToVdSyn<VdSynExprIdx> for LxMathAstIdx {
    fn to_vd_syn(self, builder: &mut VdSynExprBuilder) -> VdSynExprIdx {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use visored_annotation::annotation::{space::VdSpaceAnnotation, token::VdTokenAnnotation};

    #[test]
    fn math_ast_idx_to_vd_syn_expr_idx_works() {
        use crate::test_helpers::example::VdSynExprExample;
        use expect_test::{expect, Expect};
        use latex_prelude::mode::LxMode;

        fn t(
            input: &str,
            token_annotations: &[((&str, &str), VdTokenAnnotation)],
            space_annotations: &[((&str, &str), VdSpaceAnnotation)],
            expected: &Expect,
        ) {
            use crate::helpers::show::display_tree::VdSynExprDisplayTreeBuilder;

            let db = &DB::default();
            let example = VdSynExprExample::new(
                input,
                LxMode::Math,
                token_annotations,
                space_annotations,
                db,
            );
            expected.assert_debug_eq(&example.show_display_tree(db));
        }

        t(
            "",
            &[],
            &[],
            &expect![[r#"
                "\n"
            "#]],
        );
        t(
            "1",
            &[],
            &[],
            &expect![[r#"
                "1\n"
            "#]],
        );
        t(
            "11",
            &[],
            &[],
            &expect![[r#"
                "11\n"
            "#]],
        );
        t(
            "1 1",
            &[],
            &[],
            &expect![[r#"
                "1 1\n├─ 1\n└─ 1\n"
            "#]],
        );
    }
}
