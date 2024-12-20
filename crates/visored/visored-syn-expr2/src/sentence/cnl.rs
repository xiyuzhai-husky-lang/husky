use super::*;

impl<'db> VdSynExprBuilder<'db> {
    pub(super) fn parse_cnl_sentence(
        &mut self,
        token_idx: LxRoseTokenIdx,
        word: BaseCoword,
        asts: &mut Peekable<impl Iterator<Item = LxRoseAstIdx>>,
        vibe: VdSynExprVibe,
    ) -> VdSynSentenceEntry {
        let clauses = vec![self.parse_clause(token_idx, word, asts, vibe)];
        let end = loop {
            if self.peek_new_division(asts).is_some() {
                break VdSynSentenceEnd::Void;
            }
            if let Some(ast_idx) = asts.next() {
                match self.ast_arena()[ast_idx] {
                    LxRoseAstData::TextEdit { .. } => todo!(),
                    LxRoseAstData::Word(token_idx, coword) => {
                        self.emit_message_over_token_to_stdout(
                            *token_idx,
                            format!("coword: {}", coword),
                        );
                        todo!("coword: {}", coword)
                    }
                    LxRoseAstData::Punctuation(pucntuation_token_idx, punctuation) => {
                        match punctuation {
                            LxRosePunctuation::Comma => todo!(),
                            LxRosePunctuation::Period => {
                                break VdSynSentenceEnd::Period(pucntuation_token_idx)
                            }
                            LxRosePunctuation::Colon => todo!(),
                            LxRosePunctuation::Semicolon => todo!(),
                            LxRosePunctuation::Exclamation => todo!(),
                            LxRosePunctuation::Question => todo!(),
                            LxRosePunctuation::LeftCurl => todo!(),
                            LxRosePunctuation::RightCurl => todo!(),
                            LxRosePunctuation::LeftBox => todo!(),
                            LxRosePunctuation::RightBox => todo!(),
                            LxRosePunctuation::EscapedBackslash => todo!(),
                            LxRosePunctuation::EscapedLcurl => todo!(),
                            LxRosePunctuation::EscapedRcurl => todo!(),
                        }
                    }
                    LxRoseAstData::Math { .. } => todo!(),
                    LxRoseAstData::Delimited {
                        left_delimiter_token_idx,
                        left_delimiter,
                        asts,
                        right_delimiter_token_idx,
                        right_delimiter,
                    } => todo!(),
                    LxRoseAstData::CompleteCommand {
                        command_token_idx,
                        command_path,
                        options,
                        ref arguments,
                    } => todo!(),
                    LxRoseAstData::Environment { .. } => todo!(),
                    LxRoseAstData::NewParagraph(_) => todo!(),
                }
            } else {
                break VdSynSentenceEnd::Void;
            }
        };
        let clauses = self.alloc_clauses(clauses);
        let data = VdSynSentenceData::Clauses { clauses, end };
        // TODO: match snl mode
        VdSynSentenceEntry::Cnl { data }
    }
}
