use husky_word::Paradigm;

use super::*;

impl<'a> AstTransformer<'a> {
    pub fn call_defn_head(
        &mut self,
        token_group: &[HuskyToken],
        opt_this_liason: Option<ParameterLiason>,
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstVariant> {
        expect_at_least!(token_group, token_group.text_range(), 5);
        expect_block_head!(token_group);
        let mut token_stream: TokenStream = token_group.into();
        let mut parser = AtomParser::new(self, &mut token_stream);
        let paradigm = get!(parser, paradigm);
        enter_block(self);
        let mut parser = AtomParser::new(self, &mut token_stream);
        let ranged_ident = get!(parser, custom_ident);
        parser
            .atom_context
            .push_abs_semantic_token(AbsSemanticToken::new(
                SemanticTokenKind::Entity(EntityKind::Function {
                    requires_lazy: paradigm.is_lazy(),
                }),
                ranged_ident.range,
            ));
        let generic_parameters = parser.generic_parameters()?;
        let parameters = parser.parameters()?;
        let return_ty = parser.func_return_ty()?;
        if let Some(route) = self
            .context
            .value()
            .opt_subroute(self.db.upcast(), ranged_ident.ident)
        {
            match self.push_new_symbol(Symbol {
                init_ident: ranged_ident,
                kind: match opt_this_liason {
                    Some(_) => SymbolKind::ThisMethod,
                    None => SymbolKind::EntityRoute(route),
                },
            }) {
                Some(old) => {
                    return err!(
                        format!("a symbol with the same name already exists"),
                        ranged_ident.range
                    )
                }
                _ => (),
            }
        };
        self.context.set(AstContext::Stmt {
            paradigm,
            return_context: Some(RawReturnContext {
                return_ty,
                kind: ReturnContextKind::Normal,
            }),
        });
        self.opt_this_liason.set(opt_this_liason);
        self.symbols.extend(
            parameters
                .iter()
                .map(|parameter| Symbol::variable(parameter.ranged_ident)),
        );
        match paradigm {
            Paradigm::EagerProcedural => (),
            Paradigm::EagerFunctional | Paradigm::LazyFunctional => {
                for parameter in parameters.iter() {
                    match parameter.ranged_liason.liason {
                        ParameterLiason::Pure
                        | ParameterLiason::Move
                        | ParameterLiason::EvalRef
                        | ParameterLiason::TempRef => (),
                        ParameterLiason::TempRefMut | ParameterLiason::MoveMut => {
                            return err!(
                                "invalid  parameter liason",
                                parameter.ranged_liason.opt_range.unwrap()
                            )
                        }
                        ParameterLiason::MemberAccess => todo!(),
                    }
                }
            }
        }
        Ok(AstVariant::CallFormDefnHead {
            ident: ranged_ident,
            paradigm,
            spatial_parameters: generic_parameters,
            parameters,
            return_ty,
            output_liason: OutputLiason::Transfer,
            opt_this_liason,
        })
    }
}
