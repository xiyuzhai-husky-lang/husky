use super::*;
use disambiguate_token::DisambiguatedToken;
use either::*;
use expr::{list_item::VdSynSeparatedListItem, VdSynExprClass, VdSynExprData};
use incomplete_expr::{IncompleteCallListOpr, IncompleteSeparatedListOpr, IncompleteVdSynExprData};
use latex_token::idx::LxTokenIdx;
use smallvec::smallvec;
use visored_opr::{
    delimiter::{VdLeftDelimiter, VdRightDelimiter},
    opr::{binary::VdBaseBinaryOpr, prefix::VdBasePrefixOpr, suffix::VdBaseSuffixOpr, VdBaseOpr},
    precedence::VdPrecedence,
    separator::VdSeparator,
};

impl<'a, 'db> VdSynExprParser<'a, 'db> {
    pub(crate) fn accept_token(&mut self, token: DisambiguatedToken) {
        match token {
            DisambiguatedToken::Expr(expr, class) => match class {
                VdSynExprClass::Atom => self.accept_atom(expr),
                VdSynExprClass::Prefix => todo!(),
                VdSynExprClass::Suffix => todo!(),
                VdSynExprClass::Separator => todo!(),
            },
            DisambiguatedToken::Opr(opr) => self.accept_opr(opr),
            DisambiguatedToken::Separator(sep) => self.accept_separator(sep),
        }
    }

    fn accept_list_end(&mut self, ket: VdRightDelimiter, ket_token_idx: LxTokenIdx) {
        todo!()
        // self.reduce(VdPrecedence::LIST_ITEM);
        // let last_incomplete_expr = self.take_last_incomplete_expr().unwrap();
        // match last_incomplete_expr {
        //     IncompleteVdSynExprData::SeparatedList {
        //         opr,
        //         bra,
        //         mut items,
        //     } => {
        //         if bra != ket {
        //             todo!()
        //         }
        //         self.take_complete_and_push_to_top(|slf, finished_expr| {
        //             if let Some(expr) = finished_expr {
        //                 items.push(VdSynSeparatedListItem::new(
        //                     slf.builder.alloc_expr(expr, todo!()),
        //                     None,
        //                 ))
        //             }
        //             match opr {
        //                 IncompleteSeparatedListOpr::UnitOrDelimiteredOrNewTuple => {
        //                     match items.last() {
        //                         None => VdSynExprData::Unit {
        //                             lpar_token_idx: bra_token_idx,
        //                             rpar_token_idx: ket_token_idx,
        //                         },
        //                         Some(last_item) => {
        //                             if items.len() == 1 && last_item.comma_token_idx().is_none() {
        //                                 VdSynExprData::Delimitered {
        //                                     lpar_token_idx: bra_token_idx,
        //                                     item: last_item.syn_expr_idx(),
        //                                     rpar_token_idx: ket_token_idx,
        //                                 }
        //                             } else {
        //                                 VdSynExprData::NewTuple {
        //                                     lpar_token_idx: bra_token_idx,
        //                                     items,
        //                                     rpar_token_idx: ket_token_idx,
        //                                 }
        //                             }
        //                         }
        //                     }
        //                     .into()
        //                 }
        //             }
        //         })
        //     }
        //     IncompleteVdSynExprData::CallList { opr, items } => match opr {
        //         IncompleteCallListOpr::FunctionCall {
        //             function,
        //             generic_arguments,
        //         } => self.set_complete_expr(VdSynExprData::FunctionCall {
        //             function,
        //             template_arguments: generic_arguments,
        //             lpar_token_idx: bra_token_idx,
        //             items,
        //             rpar_token_idx: ket_token_idx,
        //         }),
        //         IncompleteCallListOpr::MethodCall { .. } => todo!(),
        //     },
        //     _ => {
        //         p!(last_incomplete_expr);
        //         // p!(self.context.path.debug(self.db()));
        //         p!(ket_token_idx);
        //         todo!()
        //     }
        // }
    }

    fn accept_atom(&mut self, atom: VdSynExprData) {
        self.push_top_syn_expr(atom.into())
    }

    fn accept_opr(&mut self, opr: VdBaseOpr) {
        match opr {
            VdBaseOpr::Binary(opr) => todo!(),
            VdBaseOpr::Prefix(opr) => self.accept_prefix_opr(Left(opr)),
            VdBaseOpr::Suffix(opr) => self.accept_suffix_opr(Left(opr)),
        }
    }

    fn accept_prefix_opr(&mut self, opr: Either<VdBasePrefixOpr, VdSynExprIdx>) {
        self.push_top_syn_expr(IncompleteVdSynExprData::Prefix { opr }.into())
    }

    fn accept_suffix_opr(&mut self, opr: Either<VdBaseSuffixOpr, VdSynExprIdx>) {
        self.take_complete_and_push_to_top(|slf, top_expr| match top_expr {
            Some(expr) => VdSynExprData::Suffix {
                opd: slf.builder.alloc_expr(expr, todo!()),
                opr,
            }
            .into(),
            None => todo!(),
        })
    }

    fn accept_separator(&mut self, separator: VdSeparator) {
        todo!()
        // match self.take_complete_expr() {
        //     Some(item) => {
        //         let item = self.context_mut().alloc_expr(item);
        //         match self.last_incomplete_expr_mut() {
        //             Some(expr) => match expr {
        //                 IncompleteSynExprData::CommaList {
        //                     opr: _,
        //                     bra: _,
        //                     bra_token_idx: _,
        //                     items,
        //                 } => items.push(SynCommaListItem::new(item, Some(comma_token_idx))),
        //                 IncompleteSynExprData::CallList { items, .. } => items.push(
        //                     SynSimpleOrVariadicCallListItem::new(
        //                         item,
        //                         CallListSeparator::Comma(comma_token_idx),
        //                     )
        //                     .into(),
        //                 ),
        //                 _ => unreachable!(),
        //             },
        //             None => unreachable!(),
        //         }
        //     }
        //     None => match self.last_incomplete_expr_mut() {
        //         Some(expr) => match expr {
        //             IncompleteSynExprData::CommaList {
        //                 opr: _,
        //                 bra: _,
        //                 bra_token_idx: _,
        //                 items: _,
        //             } => todo!(),
        //             IncompleteSynExprData::CallList { items, .. } => match items.last_mut() {
        //                 Some(last_item) => match last_item.separator() {
        //                     CallListSeparator::None => {
        //                         last_item.set_separator(CallListSeparator::Comma(comma_token_idx))
        //                     }
        //                     CallListSeparator::Comma(_) => todo!(),
        //                     CallListSeparator::Semicolon(_) => todo!(),
        //                 },
        //                 None => todo!(),
        //             },
        //             _ => unreachable!(),
        //         },
        //         None => unreachable!(),
        //     },
        // }
    }

    fn accept_binary_opr(&mut self, binary: Either<VdBaseBinaryOpr, VdSynExprIdx>) {
        // self.reduce(binary.precedence());
        // let lopd = self.take_complete_expr().unwrap_or(VdSynExprData::Err(
        //     OriginalSynExprError::NoLeftOperandForBinaryOperator { binary_token_idx }.into(),
        // ));
        // let lopd = self.builder.alloc_expr(lopd, todo!());
        // let unfinished_expr = IncompleteVdSynExprData::Binary {
        //     lopd,
        //     punctuation: binary,
        //     punctuation_token_idx: binary_token_idx,
        // };
        // self.push_top_syn_expr(unfinished_expr.into())
        todo!()
    }

    fn accept_list_start(&mut self, bra: VdLeftDelimiter, bra_token_idx: LxTokenIdx) {
        // self.reduce(Precedence::Application);
        // self.take_complete_and_push_to_top(|parser, finished_expr| -> TopSynExpr {
        //     let finished_expr = finished_expr.map(|expr| parser.context_mut().alloc_expr(expr));
        //     todo!()
        //     // match bra {
        //     //     VdLeftDelimiter::Par => todo!(),
        //     //     VdLeftDelimiter::Brace => todo!(),
        //     //     VdLeftDelimiter::Vert => todo!(),
        //     // }
        // })
        todo!()
    }
}
