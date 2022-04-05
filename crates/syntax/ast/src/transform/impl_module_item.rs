use crate::*;
use text::TextRanged;
use token::*;
use word::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_module_item(
        &mut self,
        token_group: &[Token],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstKind> {
        let keyword = if let TokenKind::Keyword(keyword) = token_group[0].kind {
            keyword
        } else {
            todo!()
        };
        enter_block(self);
        match keyword {
            Keyword::Routine(routine_keyword) => {
                self.parse_routine_decl(routine_keyword, token_group)
            }
            Keyword::Type(ty_kw) => self.parse_ty_defn(ty_kw, token_group),
            Keyword::Use | Keyword::Mod => todo!(),
            Keyword::Stmt(_) => todo!("no stmt in module level"),
            Keyword::Config(cfg) => Ok(match cfg {
                ConfigKeyword::Dataset => {
                    self.env.set_value(Env::DatasetConfig);
                    self.use_all(
                        BuiltinIdentifier::Datasets.into(),
                        token_group[0].text_range(),
                    )?;
                    AstKind::DatasetConfigDefnHead
                }
            }),
            Keyword::Def => self.parse_morphism_decl(token_group),
            Keyword::Main => {
                self.env.set_value(Env::Main);
                Ok(AstKind::MainDefn)
            }
        }
    }
}
