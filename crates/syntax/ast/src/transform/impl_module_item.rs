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
            return err_derived!();
        };
        match keyword {
            Keyword::Routine(routine_keyword) => {
                enter_block(self);
                self.parse_routine_decl(routine_keyword, token_group)
            }
            Keyword::Type(ty_kw) => {
                enter_block(self);
                self.parse_ty_defn(ty_kw, token_group)
            }
            Keyword::Use => self.parse_use(token_group),
            Keyword::Mod => todo!(),
            Keyword::Stmt(_) => todo!("no stmt in module level"),
            Keyword::Config(cfg) => {
                enter_block(self);
                Ok(match cfg {
                    ConfigKeyword::Dataset => {
                        self.env.set_value(AstContext::DatasetConfig);
                        self.use_all(RootIdentifier::Datasets.into(), token_group[0].text_range())?;
                        AstKind::DatasetConfigDefnHead
                    }
                })
            }
            Keyword::Def => {
                enter_block(self);
                self.parse_morphism_decl(token_group)
            }
            Keyword::Main => {
                enter_block(self);
                self.env.set_value(AstContext::Main);
                Ok(AstKind::MainDefn)
            }
        }
    }

    fn parse_use(&mut self, token_group: &[Token]) -> AstResult<AstKind> {
        let atoms = self.parse_atoms(&token_group[1..], |parser| parser.parse_all())?;
        msg_once!("todo: use all");
        let route = if atoms.len() != 1 {
            todo!("expect one atom for entity route")
        } else {
            match atoms[0].kind {
                AtomKind::EntityRoute { route, .. } => {
                    if route.generic_arguments.len() != 0 {
                        todo!("expect no generics")
                    }
                    route
                }
                _ => todo!(),
            }
        };
        self.use_route(route)?;
        Ok(AstKind::Use {
            use_variant: UseVariant::Route { route },
        })
    }
}
