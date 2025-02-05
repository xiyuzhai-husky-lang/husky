pub mod attach;
pub mod binary;
pub mod delimited;
pub mod frac;
pub mod letter;
pub mod literal;
pub mod prefix;
pub mod separated_list;
pub mod sqrt;
pub mod suffix;
#[cfg(test)]
pub mod tests;
pub mod uniadic_array;
pub mod uniadic_chain;
pub mod variadic_array;
pub mod variadic_chain;

use self::{attach::*, binary::*, delimited::*, prefix::*, separated_list::*, suffix::*};
use crate::*;
use either::*;
use frac::VdSemFracDispatch;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use latex_ast::ast::math::LxMathCompleteCommandArgument;
use latex_math_letter::letter::LxMathLetter;
use latex_prelude::script::LxScriptKind;
use latex_token::idx::{LxMathTokenIdx, LxTokenIdx, LxTokenIdxRange};
use letter::VdSemLetterDispatch;
use once_place::OncePlace;
use sqrt::VdSemSqrtDispatch;
use visored_opr::{
    delimiter::{
        VdBaseLeftDelimiter, VdBaseRightDelimiter, VdCompositeLeftDelimiter,
        VdCompositeRightDelimiter,
    },
    opr::{
        binary::{VdBaseBinaryOpr, VdCompositeBinaryOpr},
        prefix::{VdBasePrefixOpr, VdCompositePrefixOpr},
        suffix::{VdBaseSuffixOpr, VdCompositeSuffixOpr},
        VdBaseOpr,
    },
    separator::{VdBaseSeparator, VdSeparatorClass},
};
use visored_signature::signature::separator::base::VdBaseSeparatorSignature;
use visored_syn_expr::expr::{VdSynExprData, VdSynSeparator};
use visored_term::{
    term::{literal::VdLiteral, VdTerm},
    ty::VdType,
};

/// It's a tree of both form and meaning
#[derive(Debug, PartialEq, Eq)]
pub enum VdSemExprData {
    Literal {
        token_idx_range: LxTokenIdxRange,
        literal: VdLiteral,
    },
    // TODO: split into namespace and variable, using dispatch??
    Letter {
        token_idx_range: LxTokenIdxRange,
        letter: LxMathLetter,
        dispatch: VdSemLetterDispatch,
    },
    BaseOpr {
        opr: VdBaseOpr,
    },
    Binary {
        lopd: VdSemExprIdx,
        opr: VdSemBinaryOpr,
        ropd: VdSemExprIdx,
        dispatch: VdSemBinaryDispatch,
    },
    Prefix {
        opr: VdSemPrefixOpr,
        opd: VdSemExprIdx,
        dispatch: VdSemPrefixDispatch,
    },
    Suffix {
        opd: VdSemExprIdx,
        opr: VdSemSuffixOpr,
        dispatch: (),
    },
    Attach {
        base: VdSemExprIdx,
        // INVARIANCE: at least one of these are some
        scripts: Vec<(LxScriptKind, VdSemExprIdx)>,
        dispatch: VdSemAttachDispatch,
    },
    FoldingSeparatedList {
        separator_class: VdSeparatorClass,
        leader: VdSemExprIdx,
        followers: VdSemSeparatedListFollowers,
    },
    ChainingSeparatedList {
        separator_class: VdSeparatorClass,
        leader: VdSemExprIdx,
        followers: VdSemSeparatedListFollowers,
        joined_separator_and_signature: Option<(VdBaseSeparator, VdBaseSeparatorSignature)>,
    },
    // TODO: maybe these two are just separated lists?
    UniadicChain,
    VariadicChain,
    UniadicArray,
    VariadicArray,
    LxDelimited {
        left_delimiter_token_idx: LxMathTokenIdx,
        item: VdSemExprIdx,
        right_delimiter_token_idx: LxMathTokenIdx,
    },
    Delimited {
        left_delimiter: VdSemLeftDelimiter,
        item: VdSemExprIdx,
        right_delimiter: VdSemRightDelimiter,
    },
    Frac {
        command_token_idx: LxMathTokenIdx,
        numerator: VdSemExprIdx,
        denominator: VdSemExprIdx,
        denominator_arg: LxMathCompleteCommandArgument,
        dispatch: VdSemFracDispatch,
    },
    Sqrt {
        command_token_idx: LxMathTokenIdx,
        radicand: VdSemExprIdx,
        radicand_arg: LxMathCompleteCommandArgument,
        dispatch: VdSemSqrtDispatch,
    },
}

pub struct VdSemExprEntry {
    data: VdSemExprData,
    ty: VdType,
    expected_ty: OncePlace<VdType>,
    /// `None` if not inferred
    ///
    /// Note that all terms are inferred.
    term: OncePlace<VdTerm>,
    // todo: var deps
}

pub type VdSemExprIdx = ArenaIdx<VdSemExprEntry>;
pub type VdSemExprIdxRange = ArenaIdxRange<VdSemExprEntry>;
pub type VdSemExprArena = Arena<VdSemExprEntry>;
pub type VdSemExprArenaRef<'a> = ArenaRef<'a, VdSemExprEntry>;
pub type VdSemExprMap<T> = ArenaMap<VdSemExprEntry, T>;

impl VdSemExprEntry {
    pub fn new(data: VdSemExprData, ty: VdType) -> Self {
        Self {
            data,
            ty,
            expected_ty: OncePlace::default(),
            term: OncePlace::default(),
        }
    }

    pub fn data(&self) -> &VdSemExprData {
        &self.data
    }

    pub fn ty(&self) -> VdType {
        self.ty
    }

    pub fn expected_ty(&self) -> Option<VdType> {
        self.expected_ty.get().copied()
    }
}

impl VdSemExprEntry {
    pub(crate) fn set_expected_ty(&mut self, expected_ty: VdType) {
        if format!("{:?}", expected_ty) == "Prop" && format!("{:?}", self.ty()) == "ℚ" {
            todo!()
        }
        self.expected_ty.set(expected_ty);
    }

    pub(crate) fn set_term(&mut self, term: VdTerm) {
        self.term.set(term);
    }
}

impl<I> ToVdSem<VdSemExprIdxRange> for I
where
    I: IntoIterator<Item = VdSynExprIdx>,
{
    fn to_vd_sem(self, builder: &mut VdSemExprBuilder) -> VdSemExprIdxRange {
        let mut exprs: Vec<VdSemExprEntry> = vec![];
        for expr in self {
            exprs.push(builder.build_expr_entry(expr));
        }
        builder.alloc_exprs(exprs)
    }
}

impl ToVdSem<VdSemExprIdx> for (VdSynExprIdx, VdType) {
    fn to_vd_sem(self, builder: &mut VdSemExprBuilder) -> VdSemExprIdx {
        let (slf, expected_ty) = self;
        if let Some(&idx) = builder.syn_to_sem_expr_map().get(slf) {
            return idx;
        }
        let entry = builder.build_expr_entry(slf);
        builder.alloc_expr(slf, entry, Some(expected_ty))
    }
}

impl<'a> VdSemExprBuilder<'a> {
    pub(crate) fn build_expr_entry(&mut self, syn_expr: VdSynExprIdx) -> VdSemExprEntry {
        let db = self.db();
        let (data, ty) = match self.syn_expr_arena()[syn_expr] {
            VdSynExprData::Literal {
                token_idx_range,
                literal,
            } => (
                VdSemExprData::Literal {
                    token_idx_range,
                    literal,
                },
                literal.ty(db),
            ),
            VdSynExprData::Letter {
                token_idx_range,
                letter,
            } => self.build_letter(syn_expr, token_idx_range, letter),
            VdSynExprData::BaseOpr { opr } => todo!("opr = {:?}", opr),
            VdSynExprData::Binary { lopd, opr, ropd } => self.build_binary(lopd, opr, ropd),
            VdSynExprData::Prefix { opr, opd } => self.build_prefix(opr, opd),
            VdSynExprData::Suffix { opd, opr } => todo!("opr = {:?}", opr),
            VdSynExprData::SeparatedList {
                separator_class,
                items,
                ref separators,
            } => return self.build_separated_list(separator_class, items, separators),
            VdSynExprData::LxDelimited {
                left_delimiter_token_idx,
                left_delimiter,
                item: syn_item,
                right_delimiter_token_idx,
                right_delimiter,
            } => {
                // this is much simpler than the delimited case because the delimiter has no semantic meaning
                let item = self.build_expr_entry(syn_item);
                let ty = item.ty();
                let item = self.alloc_expr(syn_item, item, None);
                (
                    VdSemExprData::LxDelimited {
                        left_delimiter_token_idx,
                        item,
                        right_delimiter_token_idx,
                    },
                    ty,
                )
            }
            VdSynExprData::Delimited {
                left_delimiter,
                item,
                right_delimiter,
            } => self.build_delimited(left_delimiter, item, right_delimiter),
            VdSynExprData::Attach { base, ref scripts } => self.build_attach(base, scripts),
            VdSynExprData::Fraction {
                command_token_idx,
                numerator,
                denominator,
                denominator_arg,
                ..
            } => self.build_frac(command_token_idx, numerator, denominator, denominator_arg),
            VdSynExprData::Sqrt {
                command_token_idx,
                radicand,
                radicand_arg,
                ..
            } => self.build_sqrt(command_token_idx, radicand, radicand_arg),
            VdSynExprData::UniadicChain => todo!(),
            VdSynExprData::VariadicChain => todo!(),
            VdSynExprData::UniadicArray => todo!(),
            VdSynExprData::VariadicArray => todo!(),
            VdSynExprData::Err(ref error) => todo!(),
        };
        VdSemExprEntry::new(data, ty)
    }
}

impl ToVdSem<VdSemSeparator> for VdSynSeparator {
    fn to_vd_sem(self, builder: &mut VdSemExprBuilder) -> VdSemSeparator {
        match self {
            VdSynSeparator::Base(lx_token_idx_range, vd_base_separator) => {
                VdSemSeparator::Base(lx_token_idx_range, vd_base_separator)
            }
            VdSynSeparator::Composite(arena_idx, vd_composite_separator) => todo!(),
        }
    }
}

impl VdSemExprData {
    pub(crate) fn children(&self) -> Vec<VdSemExprIdx> {
        match *self {
            VdSemExprData::Literal { .. }
            | VdSemExprData::Letter { .. }
            | VdSemExprData::BaseOpr { .. } => vec![],
            VdSemExprData::Binary {
                lopd, opr, ropd, ..
            } => match opr {
                VdSemBinaryOpr::Base(_, _) => vec![lopd, ropd],
                VdSemBinaryOpr::Composite(opr, _) => vec![lopd, opr, ropd],
            },
            VdSemExprData::Prefix { opr, opd, .. } => match opr {
                VdSemPrefixOpr::Base(_, _) => vec![opd],
                VdSemPrefixOpr::Composite(opr, _) => vec![opr, opd],
            },
            VdSemExprData::Suffix { opd, opr, .. } => match opr {
                VdSemSuffixOpr::Base(_, _) => vec![opd],
                VdSemSuffixOpr::Composite(opr, _) => vec![opd, opr],
            },
            VdSemExprData::Attach {
                base, ref scripts, ..
            } => [base]
                .into_iter()
                .chain(scripts.iter().map(|&(_, script)| script))
                .collect(),
            // ad hoc
            VdSemExprData::UniadicChain => vec![],
            // ad hoc
            VdSemExprData::VariadicChain => vec![],
            // ad hoc
            VdSemExprData::UniadicArray => vec![],
            // ad hoc
            VdSemExprData::VariadicArray => vec![],
            VdSemExprData::FoldingSeparatedList {
                leader,
                ref followers,
                ..
            }
            | VdSemExprData::ChainingSeparatedList {
                leader,
                ref followers,
                ..
            } => [leader]
                .into_iter()
                .chain(
                    followers
                        .iter()
                        .map(|f| match f.separator {
                            VdSemSeparator::Base(..) => vec![f.expr],
                            VdSemSeparator::Composite(expr, _) => vec![expr, f.expr],
                        })
                        .flatten(),
                )
                .collect(),
            VdSemExprData::LxDelimited { item, .. } => vec![item],
            VdSemExprData::Delimited {
                left_delimiter,
                item,
                right_delimiter,
            } => {
                let mut children = vec![];
                match left_delimiter {
                    VdSemLeftDelimiter::Base(_, _) => (),
                    VdSemLeftDelimiter::Composite(expr, _) => children.push(expr),
                }
                children.push(item);
                match right_delimiter {
                    VdSemRightDelimiter::Base(_, _) => (),
                    VdSemRightDelimiter::Composite(expr, _) => children.push(expr),
                }
                children
            }
            VdSemExprData::Frac {
                numerator,
                denominator,
                ..
            } => vec![numerator, denominator],
            VdSemExprData::Sqrt { radicand, .. } => vec![radicand],
        }
    }
}

impl<'db> VdSemExprBuilder<'db> {
    pub fn calc_expr_term(&mut self, expr: VdSemExprIdx) -> VdTerm {
        match *self.expr_arena()[expr].data() {
            VdSemExprData::Literal {
                token_idx_range,
                literal,
            } => todo!(),
            VdSemExprData::Letter {
                token_idx_range,
                letter,
                ref dispatch,
            } => self.calc_letter_term(expr, token_idx_range, letter, dispatch),
            VdSemExprData::BaseOpr { opr } => todo!(),
            VdSemExprData::Binary {
                lopd,
                opr,
                ropd,
                ref dispatch,
            } => todo!(),
            VdSemExprData::Prefix { opr, opd, dispatch } => todo!(),
            VdSemExprData::Suffix { opd, opr, dispatch } => todo!(),
            VdSemExprData::Attach {
                base,
                ref scripts,
                ref dispatch,
            } => todo!(),
            VdSemExprData::FoldingSeparatedList { .. } => todo!(),
            VdSemExprData::ChainingSeparatedList { .. } => todo!(),
            VdSemExprData::UniadicChain => todo!(),
            VdSemExprData::VariadicChain => todo!(),
            VdSemExprData::UniadicArray => todo!(),
            VdSemExprData::VariadicArray => todo!(),
            VdSemExprData::LxDelimited {
                left_delimiter_token_idx,
                item,
                right_delimiter_token_idx,
            } => todo!(),
            VdSemExprData::Delimited {
                left_delimiter,
                item,
                right_delimiter,
            } => todo!(),
            VdSemExprData::Frac { .. } => todo!(),
            VdSemExprData::Sqrt {
                command_token_idx,
                radicand,
                ..
            } => todo!(),
        }
    }
}
