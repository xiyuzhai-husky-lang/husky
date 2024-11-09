use super::{
    expr::{VdSynExprClass, VdSynExprData, VdSynExprIdx},
    VdSynExprParser,
};
use latex_ast::ast::math::{
    LxMathAstData, LxMathAstIdx, LxMathCommandArgument, LxMathCommandArgumentData,
};
use latex_command::path::LxCommandPath;
use latex_math_letter::LxMathLetter;
use latex_token::{
    data::math::digit::LxMathDigit,
    idx::{LxMathTokenIdx, LxTokenIdxRange},
};
use salsa::DebugWithDb;
use visored_annotation::annotation::{space::VdSpaceAnnotation, token::VdTokenAnnotation};
use visored_opr::{
    delimiter::{VdBaseLeftDelimiter, VdBaseRightDelimiter},
    opr::VdBaseOpr,
    precedence::VdPrecedence,
    separator::VdBaseSeparator,
};
use visored_resolution::resolution::{
    command::VdCompleteCommandResolution, punctuation::VdPunctuationResolution,
};
use visored_zfc_ty::term::literal::{VdZfcLiteral, VdZfcLiteralData};

pub struct DisambiguatedMathAst {
    ast: LxMathAstIdx,
    preceding_space_annotation: Option<VdSpaceAnnotation>,
}

#[derive(Debug)]
pub enum ResolvedToken {
    Expr(VdSynExprData, VdSynExprClass),
    Opr(LxMathTokenIdx, VdBaseOpr),
    Separator(VdBaseSeparator),
    LeftDelimiter(VdBaseLeftDelimiter),
    RightDelimiter(VdBaseRightDelimiter),
}

impl<'a, 'db> VdSynExprParser<'a, 'db> {
    pub fn resolve_token(&mut self, next: &mut LxMathAstIdx, end: LxMathAstIdx) -> ResolvedToken {
        use crate::builder::ToVdSyn;

        let ast_data = &self.builder.ast_arena()[*next];
        *next += 1;
        match *ast_data {
            LxMathAstData::Letter(token_idx, letter) => {
                if let Some(token_annotation) =
                    self.builder.annotations().token_annotation(*token_idx)
                {
                    match token_annotation {
                        VdTokenAnnotation::Integral(lx_integral_annotation) => todo!(),
                        VdTokenAnnotation::Variable(lx_variable_annotation) => todo!(),
                        VdTokenAnnotation::Differential => {
                            return ResolvedToken::Opr(token_idx, VdBaseOpr::DIFFERENTIAL)
                        }
                    }
                }
                ResolvedToken::Expr(
                    VdSynExprData::Letter {
                        token_idx_range: LxTokenIdxRange::new_single(*token_idx),
                        letter,
                    },
                    VdSynExprClass::ATOM,
                )
            }
            LxMathAstData::Punctuation(token_idx, punctuation) => {
                if let Some(token_annotation) =
                    self.builder.annotations().token_annotation(*token_idx)
                {
                    return todo!();
                }
                match self.builder.default_resolution_table()[punctuation] {
                    Some(resolution) => match resolution {
                        VdPunctuationResolution::Opr(opr) => ResolvedToken::Opr(token_idx, opr),
                        VdPunctuationResolution::Separator(separator) => {
                            ResolvedToken::Separator(separator)
                        }
                        VdPunctuationResolution::LeftDelimiter(left_delimiter) => {
                            ResolvedToken::LeftDelimiter(left_delimiter)
                        }
                        VdPunctuationResolution::RightDelimiter(right_delimiter) => {
                            ResolvedToken::RightDelimiter(right_delimiter)
                        }
                        VdPunctuationResolution::Todo => todo!("punctuation = {:?}", punctuation),
                    },
                    None => todo!(),
                }
            }
            LxMathAstData::Digit(first_token_idx, digit) => {
                let mut last_token_idx = first_token_idx;
                let mut last_offset_end = self.builder.token_storage()[*first_token_idx].0.end();
                let mut s = String::from(digit.char());
                // TODO: handle real number by using a kind variable, literal number kind
                while *next < end {
                    match self.builder.ast_arena()[*next] {
                        LxMathAstData::Digit(token_idx, digit) => {
                            let offset_range = self.builder.token_storage()[*token_idx].0;
                            if offset_range.start() != last_offset_end {
                                break;
                            }
                            if let Some(space_annotation) = self
                                .builder
                                .annotations()
                                .preceding_space_annotation(*token_idx)
                            {
                                match space_annotation {
                                    VdSpaceAnnotation::Apply(apply_annotation) => todo!(),
                                    VdSpaceAnnotation::Sever(sever_annotation) => todo!(),
                                }
                            }
                            last_token_idx = token_idx;
                            last_offset_end = offset_range.end();
                            s.push(digit.char())
                        }
                        // TODO: handle real number
                        _ => break,
                    }
                    *next += 1;
                }
                let expr_data = VdSynExprData::Literal {
                    token_idx_range: LxTokenIdxRange::new_closed(*first_token_idx, *last_token_idx),
                    literal: VdZfcLiteral::new(
                        VdZfcLiteralData::NaturalNumber(s),
                        self.builder.db(),
                    ),
                };
                ResolvedToken::Expr(expr_data, VdSynExprClass::ATOM)
            }
            LxMathAstData::TextEdit { ref buffer } => todo!(),
            LxMathAstData::Attach { base, ref scripts } => {
                let base = base.to_vd_syn(self.builder);
                let scripts = scripts
                    .iter()
                    .copied()
                    .map(|(script_kind, script)| (script_kind, script.to_vd_syn(self.builder)))
                    .collect();
                ResolvedToken::Expr(
                    VdSynExprData::Attach { base, scripts },
                    VdSynExprClass::ATOM,
                )
            }
            LxMathAstData::Delimited {
                left_delimiter_token_idx,
                left_delimiter,
                asts,
                right_delimiter_token_idx,
                right_delimiter,
            } => ResolvedToken::Expr(
                VdSynExprData::LxDelimited {
                    left_delimiter_token_idx,
                    left_delimiter,
                    item: (
                        ((*left_delimiter_token_idx + 1)..(*right_delimiter_token_idx)).into(),
                        asts,
                    )
                        .to_vd_syn(self.builder),
                    right_delimiter_token_idx,
                    right_delimiter,
                },
                VdSynExprClass::ATOM,
            ),
            LxMathAstData::Command {
                command_token_idx,
                command_path,
                ref arguments,
            } => self.resolve_command(command_token_idx, command_path, arguments),
            LxMathAstData::Environment { .. } => todo!(),
        }
    }

    fn resolve_command(
        &mut self,
        command_token_idx: LxMathTokenIdx,
        command_path: LxCommandPath,
        arguments: &[LxMathCommandArgument],
    ) -> ResolvedToken {
        use crate::builder::ToVdSyn;

        match self.builder.default_resolution_table()[command_path] {
            VdCompleteCommandResolution::Letter(letter) => {
                let token_idx_range = match arguments.last() {
                    Some(argument) => {
                        LxTokenIdxRange::new_closed(*command_token_idx, *argument.rcurl_token_idx())
                    }
                    None => LxTokenIdxRange::new_single(*command_token_idx),
                };
                ResolvedToken::Expr(
                    VdSynExprData::Letter {
                        token_idx_range,
                        letter,
                    },
                    VdSynExprClass::ATOM,
                )
            }
            VdCompleteCommandResolution::Todo => {
                todo!("command_path = {:?}", command_path.debug(self.builder.db()))
            }
            VdCompleteCommandResolution::Item(_) => todo!(),
            VdCompleteCommandResolution::Frac => {
                debug_assert!(arguments.len() == 2);
                let [numerator_arg, denominator_arg] = arguments else {
                    unreachable!()
                };
                let LxMathCommandArgumentData::Math(numerator_asts) = *numerator_arg.data() else {
                    unreachable!()
                };
                let numerator =
                    (numerator_arg.asts_token_idx_range(), numerator_asts).to_vd_syn(self.builder);
                let LxMathCommandArgumentData::Math(denominator_asts) = *denominator_arg.data()
                else {
                    unreachable!()
                };
                let denominator = (denominator_arg.asts_token_idx_range(), denominator_asts)
                    .to_vd_syn(self.builder);
                ResolvedToken::Expr(
                    VdSynExprData::Fraction {
                        command_token_idx,
                        numerator,
                        denominator,
                        denominator_rcurl_token_idx: denominator_arg.rcurl_token_idx(),
                    },
                    VdSynExprClass::ATOM,
                )
            }
            VdCompleteCommandResolution::Sqrt => {
                debug_assert!(arguments.len() == 1);
                let [radicand_arg] = arguments else {
                    unreachable!()
                };
                let LxMathCommandArgumentData::Math(radicand_asts) = *radicand_arg.data() else {
                    unreachable!()
                };
                let radicand =
                    (radicand_arg.asts_token_idx_range(), radicand_asts).to_vd_syn(self.builder);
                ResolvedToken::Expr(
                    VdSynExprData::Sqrt {
                        command_token_idx,
                        radicand,
                        radicand_rcurl_token_idx: radicand_arg.rcurl_token_idx(),
                    },
                    VdSynExprClass::ATOM,
                )
            }
            VdCompleteCommandResolution::Text => todo!(),
            VdCompleteCommandResolution::Opr(vd_base_opr) => {
                ResolvedToken::Opr(command_token_idx, vd_base_opr)
            }
        }
    }
}
