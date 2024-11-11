use expr::{VdSynExprData, VdSynExprIdxRange, VdSynSeparator};

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum LetStmtResolution {
    Todo,
}

impl<'db> VdSynExprBuilder<'db> {
    pub(crate) fn build_let_stmt_resolution(&self, formula: VdSynExprIdx) -> LetStmtResolution {
        match self.expr_arena()[formula] {
            VdSynExprData::Literal {
                token_idx_range,
                literal,
            } => todo!(),
            VdSynExprData::Letter {
                token_idx_range,
                letter,
            } => todo!(),
            VdSynExprData::BaseOpr { opr } => todo!(),
            VdSynExprData::Binary { lopd, opr, ropd } => todo!(),
            VdSynExprData::Prefix { opr, opd } => todo!(),
            VdSynExprData::Suffix { opd, opr } => todo!(),
            VdSynExprData::SeparatedList {
                separator_class,
                items,
                ref separators,
            } => self.build_let_stmt_resolution_from_separated_list(items, separators),
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
        }
    }

    fn build_let_stmt_resolution_from_separated_list(
        &self,
        items: VdSynExprIdxRange,
        separators: &[VdSynSeparator],
    ) -> LetStmtResolution {
        LetStmtResolution::Todo
        // match fragments.len() {
        //     3 => todo!(),
        //     _ => todo!(),
        // }
    }
}
