use super::*;

impl<'a> AstParser<'a> {
    pub(super) fn parse_uses(&mut self, token_group_idx: TokenGroupIdx, indent: u32) -> Ast {
        let mut aux_parser = self.aux_parser(token_group_idx);
        Ast::Use {
            token_group_idx,
            accessibility: match aux_parser.parse_accessibility() {
                Ok(accessibility) => accessibility,
                Err(error) => {
                    return Ast::Err {
                        token_group_idx,
                        error,
                    }
                }
            },
        }
    }
}
