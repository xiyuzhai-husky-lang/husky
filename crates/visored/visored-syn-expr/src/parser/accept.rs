use super::*;
use either::*;
use error::OriginalVdSynExprError;
use expr::{
    list_item::VdSynSeparatedListItem, VdSynBinaryOpr, VdSynExprClass, VdSynExprData,
    VdSynPrefixOpr, VdSynSeparator, VdSynSuffixOpr,
};
use incomplete_expr::{IncompleteCallListOpr, IncompleteSeparatedListOpr, IncompleteVdSynExprData};
use latex_token::idx::{LxTokenIdx, LxTokenIdxRange};
use resolve::ResolvedToken;
use smallvec::smallvec;
use visored_annotation::annotation::space::VdSpaceAnnotation;
use visored_opr::{
    delimiter::{VdBaseLeftDelimiter, VdBaseRightDelimiter},
    opr::{binary::VdBaseBinaryOpr, prefix::VdBasePrefixOpr, suffix::VdBaseSuffixOpr, VdBaseOpr},
    precedence::VdPrecedence,
    separator::{VdBaseSeparator, VdSeparator},
};

impl<'a, 'db> VdSynExprParser<'a, 'db> {
    pub(crate) fn accept_token(
        &mut self,
        preceding_space_annotation: Option<VdSpaceAnnotation>,
        token_idx_range: LxTokenIdxRange,
        token: ResolvedToken,
    ) {
        match token {
            ResolvedToken::Expr(expr, class) => match class {
                VdSynExprClass::Complete(_) => {
                    self.accept_complete_expr(preceding_space_annotation, expr)
                }
                VdSynExprClass::Prefix => todo!(),
                VdSynExprClass::Suffix => todo!(),
                VdSynExprClass::Separator => todo!(),
            },
            ResolvedToken::Opr(lx_math_token_idx, opr) => {
                self.accept_opr(preceding_space_annotation, token_idx_range, opr)
            }
            ResolvedToken::Separator(sep) => self.accept_separator(
                preceding_space_annotation,
                VdSynSeparator::Base(token_idx_range, sep),
            ),
            ResolvedToken::LeftDelimiter(vd_left_delimiter) => todo!(),
            ResolvedToken::RightDelimiter(vd_right_delimiter) => todo!(),
        }
    }

    fn accept_list_end(&mut self, ket: VdBaseRightDelimiter, ket_token_idx: LxTokenIdx) {
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

    fn accept_complete_expr(
        &mut self,
        preceding_space_annotation: Option<VdSpaceAnnotation>,
        expr: VdSynExprData,
    ) {
        self.push_top_syn_expr(preceding_space_annotation, expr.into())
    }

    fn accept_opr(
        &mut self,
        preceding_space_annotation: Option<VdSpaceAnnotation>,
        token_idx_range: LxTokenIdxRange,
        opr: VdBaseOpr,
    ) {
        match opr {
            VdBaseOpr::Binary(opr) => self.accept_binary_opr(
                preceding_space_annotation,
                VdSynBinaryOpr::Base(token_idx_range, opr),
            ),
            VdBaseOpr::Prefix(opr) => self.accept_prefix_opr(
                preceding_space_annotation,
                VdSynPrefixOpr::Base(token_idx_range, opr),
            ),
            VdBaseOpr::Suffix(opr) => self.accept_suffix_opr(
                preceding_space_annotation,
                VdSynSuffixOpr::Base(token_idx_range, opr),
            ),
        }
    }

    fn accept_prefix_opr(
        &mut self,
        preceding_space_annotation: Option<VdSpaceAnnotation>,
        opr: VdSynPrefixOpr,
    ) {
        if let Some(annotation) = preceding_space_annotation {
            todo!()
        }
        self.push_top_syn_expr(
            preceding_space_annotation,
            IncompleteVdSynExprData::Prefix { opr }.into(),
        )
    }

    fn accept_suffix_opr(
        &mut self,
        preceding_space_annotation: Option<VdSpaceAnnotation>,
        opr: VdSynSuffixOpr,
    ) {
        if let Some(annotation) = preceding_space_annotation {
            todo!()
        }
        self.take_complete_and_push_to_top(|slf, top_expr| match top_expr {
            Some(expr) => VdSynExprData::Suffix {
                opd: slf.builder.alloc_expr(expr),
                opr,
            }
            .into(),
            None => todo!(),
        })
    }

    fn accept_separator(
        &mut self,
        preceding_space_annotation: Option<VdSpaceAnnotation>,
        separator: VdSynSeparator,
    ) {
        if let Some(annotation) = preceding_space_annotation {
            todo!()
        }
        self.reduce(
            separator.left_precedence_range(),
            Some(separator.separator()),
        );
        if separator.separator() == VdSeparator::SPACE {
            todo!()
        }
        match self.take_complete_expr() {
            Some(item) => {
                let item = self.builder.alloc_expr(item);
                match self.last_incomplete_expr_mut() {
                    Some(expr) => match expr {
                        IncompleteVdSynExprData::SeparatedList {
                            separator: separator0,
                            fragments,
                        } => {
                            if separator.separator() == *separator0 {
                                match fragments.last().unwrap() {
                                    Left(_) => fragments.push(Right(separator)),
                                    // `,,`
                                    Right(_) => todo!("repeated separator"),
                                }
                            } else {
                                todo!()
                            }
                        }
                        _ => todo!(),
                    },
                    None => self.push_top_syn_expr(
                        preceding_space_annotation,
                        IncompleteVdSynExprData::SeparatedList {
                            separator: separator.separator(),
                            fragments: smallvec![Left(item), Right(separator)],
                        }
                        .into(),
                    ),
                }
            }
            None => match self.last_incomplete_expr_mut() {
                Some(expr) => match expr {
                    IncompleteVdSynExprData::Binary { lopd, opr } => todo!(),
                    IncompleteVdSynExprData::Prefix { opr } => todo!(),
                    IncompleteVdSynExprData::SeparatedList {
                        separator: separator0,
                        fragments,
                    } => match fragments.last().unwrap() {
                        Left(_) => {
                            if *separator0 == separator.separator() {
                                fragments.push(Right(separator));
                            } else {
                                todo!()
                            }
                        }
                        Right(_) => todo!(),
                    },
                    IncompleteVdSynExprData::Delimited { bra } => todo!(),
                },
                None => todo!(),
            },
        }
    }

    fn accept_binary_opr(
        &mut self,
        preceding_space_annotation: Option<VdSpaceAnnotation>,
        opr: VdSynBinaryOpr,
    ) {
        if let Some(annotation) = preceding_space_annotation {
            todo!()
        }
        self.reduce(opr.left_precedence_range(), None);
        let lopd = self.take_complete_expr().unwrap_or(VdSynExprData::Err(
            OriginalVdSynExprError::NoLeftOperandForBinaryOperator { opr }.into(),
        ));
        let lopd = self.builder.alloc_expr(lopd);
        let incomplete_expr = IncompleteVdSynExprData::Binary { lopd, opr };
        self.push_top_syn_expr(None, incomplete_expr.into());
    }

    fn accept_list_start(&mut self, bra: VdBaseLeftDelimiter, bra_token_idx: LxTokenIdx) {
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
