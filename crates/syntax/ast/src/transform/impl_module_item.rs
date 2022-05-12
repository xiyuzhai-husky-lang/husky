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
            return derived_err!();
        };
        match keyword {
            Keyword::Routine(routine_keyword) => {
                enter_block(self);
                self.parse_routine_defn_head(routine_keyword, token_group)
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
        if token_group.len() <= 1 {
            return err!("expect route after keyword `use`", token_group.text_range());
        }
        let atoms = self.parse_atoms(&token_group[1..], |parser| parser.parse_all())?;
        emsg_once!("todo: use all");
        let route = if atoms.len() != 1 {
            todo!("expect one atom for entity route")
        } else {
            match atoms[0].kind {
                AtomVariant::EntityRoute { route, .. } => {
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

    fn parse_submodule(&mut self, token_group: &[Token]) -> AstResult<AstKind> {
        if token_group.len() < 2 {
            return derived_err!();
        }
        if token_group.len() > 2 {
            todo!()
        }
        let ident = identify!(
            self,
            token_group[1],
            SemanticTokenKind::Entity(EntityKind::Module)
        );
        Ok(AstKind::Submodule {
            ident,
            source_file: derived_not_none!(self.db.get_submodule_file(&self.file, ident.ident))?,
        })
    }
}
