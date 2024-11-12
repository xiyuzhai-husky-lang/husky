pub mod attach;
pub mod binary;
pub mod delimited;
pub mod letter;
pub mod literal;
pub mod prefix;
pub mod separated_list;
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
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use latex_math_letter::letter::LxMathLetter;
use latex_prelude::script::LxScriptKind;
use latex_token::idx::{LxMathTokenIdx, LxTokenIdx, LxTokenIdxRange};
use letter::VdSemLetterDispatch;
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
use visored_syn_expr::expr::{VdSynExprData, VdSynSeparator};
use visored_zfc_ty::{term::literal::VdZfcLiteral, ty::VdZfcType};

/// It's a tree of both form and meaning
#[derive(Debug, PartialEq, Eq)]
pub enum VdSemExprData {
    Literal {
        token_idx_range: LxTokenIdxRange,
        literal: VdZfcLiteral,
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
        dispatch: (),
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
        dispatch: AttachDispatch,
    },
    SeparatedList {
        separator_class: VdSeparatorClass,
        items: VdSemExprIdxRange,
        dispatch: VdSemSeparatedListDispatch,
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
    Fraction {
        command_token_idx: LxMathTokenIdx,
        numerator_lcurl_token_idx: LxMathTokenIdx,
        numerator: VdSemExprIdx,
        numerator_rcurl_token_idx: LxMathTokenIdx,
        denominator_lcurl_token_idx: LxMathTokenIdx,
        denominator: VdSemExprIdx,
        denominator_rcurl_token_idx: LxMathTokenIdx,
    },
    Sqrt {
        command_token_idx: LxMathTokenIdx,
        radicand_lcurl_token_idx: LxMathTokenIdx,
        radicand: VdSemExprIdx,
        radicand_rcurl_token_idx: LxMathTokenIdx,
    },
}

pub struct VdSemExprEntry {
    data: VdSemExprData,
    ty: VdZfcType,
    // todo: var deps
}

pub type VdSemExprIdx = ArenaIdx<VdSemExprEntry>;
pub type VdSemExprIdxRange = ArenaIdxRange<VdSemExprEntry>;
pub type VdSemExprArena = Arena<VdSemExprEntry>;
pub type VdSemExprArenaRef<'a> = ArenaRef<'a, VdSemExprEntry>;
pub type VdSemExprMap<T> = ArenaMap<VdSemExprEntry, T>;

impl VdSemExprEntry {
    pub fn new(data: VdSemExprData, ty: VdZfcType) -> Self {
        Self { data, ty }
    }

    pub fn data(&self) -> &VdSemExprData {
        &self.data
    }

    pub fn ty(&self) -> VdZfcType {
        self.ty
    }
}

impl<I> ToVdSem<VdSemExprIdxRange> for I
where
    I: IntoIterator<Item = VdSynExprIdx> + Clone,
{
    fn to_vd_sem(self, builder: &mut VdSemExprBuilder) -> VdSemExprIdxRange {
        let mut exprs: Vec<VdSemExprEntry> = vec![];
        for expr in self.clone() {
            exprs.push(builder.build_expr(expr));
        }
        builder.alloc_exprs(exprs, self)
    }
}

impl ToVdSem<VdSemExprIdx> for VdSynExprIdx {
    fn to_vd_sem(self, builder: &mut VdSemExprBuilder) -> VdSemExprIdx {
        if let Some(&idx) = builder.syn_to_sem_expr_map().get(self) {
            return idx;
        }
        let entry = builder.build_expr(self);
        builder.alloc_expr(self, entry)
    }
}

impl<'a> VdSemExprBuilder<'a> {
    fn build_expr(&mut self, syn_expr: VdSynExprIdx) -> VdSemExprEntry {
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
            VdSynExprData::BaseOpr { opr } => todo!(),
            VdSynExprData::Binary { lopd, opr, ropd } => todo!(),
            VdSynExprData::Prefix { opr, opd } => todo!(),
            VdSynExprData::Suffix { opd, opr } => todo!(),
            VdSynExprData::SeparatedList {
                separator_class,
                items,
                ref separators,
            } => self.build_separated_list(separator_class, items, separators),
            VdSynExprData::LxDelimited {
                left_delimiter_token_idx,
                left_delimiter,
                item,
                right_delimiter_token_idx,
                right_delimiter,
            } => todo!(),
            VdSynExprData::Delimited {
                left_delimiter,
                item,
                right_delimiter,
            } => todo!(),
            VdSynExprData::Attach { base, ref scripts } => todo!(),
            VdSynExprData::Fraction {
                command_token_idx,
                numerator,
                denominator,
                denominator_rcurl_token_idx,
            } => todo!(),
            VdSynExprData::Sqrt {
                command_token_idx,
                radicand,
                radicand_rcurl_token_idx,
            } => todo!(),
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
            VdSemExprData::SeparatedList { ref items, .. } => items.into_iter().collect(),
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
            VdSemExprData::Fraction {
                numerator,
                denominator,
                ..
            } => vec![numerator, denominator],
            VdSemExprData::Sqrt { radicand, .. } => vec![radicand],
        }
    }
}
