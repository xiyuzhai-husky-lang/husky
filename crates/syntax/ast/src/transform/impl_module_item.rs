use crate::*;
use text::TextRanged;
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
                enter_block(self);
                match token_group[2].kind {
                    TokenKind::Special(Special::LightArrow) => todo!(),
                    TokenKind::Special(Special::LPar) => {
                        self.parse_function_defn_head(paradigm, token_group)
                    }
                    _ => todo!(),
                }
                // Keyword::Def => {
                //     enter_block(self);
                //     self.parse_morphism_decl(token_group)
                // }
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
                    ConfigKeyword::Dataset => {
                        self.context
                            .set(AstContext::Stmt(Paradigm::EagerFunctional));
                        self.use_all(RootIdentifier::Datasets.into(), token_group[0].text_range())?;
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
            source_file: derived_not_none!(self.db.get_submodule_file(&self.file, ident.ident))?,
        })
    }
}
