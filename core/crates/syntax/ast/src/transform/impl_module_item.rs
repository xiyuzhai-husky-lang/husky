use crate::*;
use husky_text::TextRanged;
use token::*;
use word::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_module_item(
        &mut self,
        token_group: &[Token],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstVariant> {
        let keyword = if let TokenKind::Keyword(keyword) = token_group[0].kind {
            self.abs_semantic_tokens.push(AbsSemanticToken::new(
                SemanticTokenKind::Keyword,
                token_group[0].range,
            ));
            keyword
        } else {
            return err!(
                format!("expect keyword at the beginning of module item"),
                token_group[0].range
            );
        };
        match keyword {
            Keyword::Paradigm(paradigm) => {
                if token_group.len() < 3 {
                    return err!(format!("expect more tokens"), token_group.text_range());
                }
                match token_group[2].kind {
                    TokenKind::Special(SpecialToken::LightArrow) => {
                        enter_block(self);
                        self.parse_feature_defn_head(token_group)
                    }
                    TokenKind::Special(SpecialToken::LPar) => {
                        self.call_defn_head(token_group, None, enter_block)
                    }
                    _ => {
                        enter_block(self);
                        self.context.set(AstContext::Stmt(paradigm));
                        return err!(format!("expect `->` or `(`"), token_group[2].range);
                    }
                }
            }
            Keyword::Type(ty_kw) => {
                enter_block(self);
                self.parse_ty_defn(ty_kw, token_group)
            }
            Keyword::Use => self.parse_use(token_group),
            Keyword::Mod => self.parse_submodule(token_group),
            Keyword::Stmt(_) => err!("no stmt in module level", token_group.text_range()),
            Keyword::Config(cfg) => {
                enter_block(self);
                Ok(match cfg {
                    ConfigKeyword::Context => {
                        self.context
                            .set(AstContext::Stmt(Paradigm::EagerFunctional));
                        self.use_all(RootIdentifier::Domains.into(), token_group[0].text_range())?;
                        AstVariant::DatasetConfigDefnHead
                    }
                })
            }
            Keyword::Main => {
                enter_block(self);
                self.context.set(AstContext::Stmt(Paradigm::LazyFunctional));
                Ok(AstVariant::MainDefn)
            }
            Keyword::Visual => todo!(),
            Keyword::Liason(_) => todo!(),
        }
    }

    fn parse_submodule(&mut self, token_group: &[Token]) -> AstResult<AstVariant> {
        if token_group.len() < 2 {
            return err!(format!("expect mod <identifier>"), token_group.text_range());
        }
        if token_group.len() > 2 {
            todo!()
        }
        let ident = identify_token!(
            self,
            token_group[1],
            SemanticTokenKind::Entity(EntityKind::Module)
        );
        Ok(AstVariant::Submodule {
            ident,
            source_file: derived_not_none!(self.db.submodule_file(self.file, ident.ident))?,
        })
    }
}
