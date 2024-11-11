use super::*;
use either::*;
use error::OriginalVdSynExprError;
use expr::{
    list_item::VdSynSeparatedListItem, VdSynBinaryOpr, VdSynExprClass, VdSynExprData,
    VdSynLeftDelimiter, VdSynPrefixOpr, VdSynRightDelimiter, VdSynSeparator, VdSynSuffixOpr,
};
use expr_stack::TopVdSynExpr;
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
                VdSynExprClass::Binary => todo!(),
            },
            ResolvedToken::Opr(opr) => {
                self.accept_opr(preceding_space_annotation, token_idx_range, opr)
            }
            ResolvedToken::Separator(sep) => self.accept_separator(
                preceding_space_annotation,
                VdSynSeparator::Base(token_idx_range, sep),
            ),
            ResolvedToken::LeftDelimiter(left_delimiter) => self.accept_left_delimiter(
                preceding_space_annotation,
                token_idx_range,
                VdSynLeftDelimiter::Base(token_idx_range, left_delimiter),
            ),
            ResolvedToken::RightDelimiter(right_delimiter) => self.accept_right_delimiter(
                preceding_space_annotation,
                VdSynRightDelimiter::Base(token_idx_range, right_delimiter),
            ),
        }
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
                    Some(IncompleteVdSynExprData::SeparatedList {
                        separator: separator0,
                        fragments,
                    }) if separator.separator() == *separator0 => {
                        match fragments.last().unwrap() {
                            Left(_) => fragments.push(Right(separator)),
                            // `,,`
                            Right(_) => todo!("repeated separator"),
                        }
                    }
                    _ => self.push_top_syn_expr(
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
                    IncompleteVdSynExprData::Delimited {
                        left_delimiter: bra,
                    } => todo!(),
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

    fn accept_left_delimiter(
        &mut self,
        preceding_space_annotation: Option<VdSpaceAnnotation>,
        token_idx_range: LxTokenIdxRange,
        left_delimiter: VdSynLeftDelimiter,
    ) {
        self.push_top_syn_expr(
            preceding_space_annotation,
            IncompleteVdSynExprData::Delimited { left_delimiter }.into(),
        );
    }

    fn accept_right_delimiter(
        &mut self,
        preceding_space_annotation: Option<VdSpaceAnnotation>,
        right_delimiter: VdSynRightDelimiter,
    ) {
        if let Some(annotation) = preceding_space_annotation {
            todo!()
        }
        self.reduce(VdPrecedenceRange::RIGHT_DELIMITER_LEFT, None);
        let Some(item) = self.take_complete_expr() else {
            todo!()
        };
        let Some(IncompleteVdSynExprData::Delimited { left_delimiter }) =
            self.take_last_incomplete_expr()
        else {
            todo!()
        };
        let item = self.builder.alloc_expr(item);
        let expr = VdSynExprData::Delimited {
            left_delimiter,
            item,
            right_delimiter,
        };
        self.set_complete_expr(expr);
    }
}
