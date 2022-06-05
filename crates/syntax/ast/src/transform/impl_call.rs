use word::Paradigm;

use super::*;

impl<'a> AstTransformer<'a> {
    pub fn call_defn_head(
        &mut self,
        token_group: &[Token],
        opt_this_liason: Option<ParameterLiason>,
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstVariant> {
        expect_at_least!(token_group, token_group.text_range(), 5);
        expect_block_head!(token_group);
        let mut token_stream: TokenStream = token_group.into();
        let mut parser = AtomParser::new(self, &mut token_stream);
        let paradigm = get!(parser, paradigm);
        let ranged_ident = get!(parser, custom_ident);
        parser
            .atom_context
            .push_abs_semantic_token(AbsSemanticToken::new(
                SemanticTokenKind::Entity(EntityKind::Function {
                    is_lazy: paradigm.is_lazy(),
                }),
                ranged_ident.range,
            ));
        let generic_parameters = parser.generic_parameters()?;
        let parameters = parser.parameters()?;
        let output_ty = parser.func_output_ty()?;
        match paradigm {
            Paradigm::EagerProcedural => (),
            Paradigm::EagerFunctional | Paradigm::LazyFunctional => {
                for parameter in parameters.iter() {
                    match parameter.liason {
                        ParameterLiason::Pure
                        | ParameterLiason::Move
                        | ParameterLiason::EvalRef
                        | ParameterLiason::TempRef => (),
                        ParameterLiason::TempRefMut | ParameterLiason::MoveMut => {
                            todo!("invalid  parameter liason")
                        }
                        ParameterLiason::MemberAccess => todo!(),
                    }
                }
            }
        }
        match self.push_new_symbol(Symbol {
            ident: ranged_ident.ident,
            kind: match opt_this_liason {
                Some(_) => SymbolKind::ThisMethod,
                None => SymbolKind::EntityRoute(
                    self.context.value().subroute(self.db, ranged_ident.ident),
                ),
            },
        }) {
            Some(old) => {
                return err!(
                    format!("a symbol with the same name already exists"),
                    ranged_ident.range
                )
            }
            _ => (),
        };
        enter_block(self);
        self.opt_this_liason.set(Some(opt_this_liason.unwrap()));
        self.symbols.extend(
            parameters
                .iter()
                .map(|parameter| Symbol::variable(parameter.ranged_ident)),
        );
        self.context.set(AstContext::Stmt(paradigm));
        Ok(AstVariant::CallFormDefnHead {
            ident: ranged_ident,
            paradigm,
            spatial_parameters: generic_parameters,
            parameters,
            output_ty,
            output_liason: OutputLiason::Transfer,
            opt_this_liason,
        })
    }
}
