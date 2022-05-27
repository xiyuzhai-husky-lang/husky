use std::sync::Arc;

use crate::*;
use defn_head::{CallableDefnHead, GenericParameter, GenericPlaceholderVariant, InputParameter};
use entity_route::*;
use token::SemanticTokenKind;
use vm::{InputLiason, OutputLiason};
use word::{IdentDict, Paradigm};

use super::*;

// inner ops
impl<'a> AtomParser<'a> {
    pub fn routine_defn_head(mut self, paradigm: Paradigm) -> AtomResult<CallableDefnHead> {
        let routine_ident = get!(self, custom_ident);
        self.push_abs_semantic_token(AbsSemanticToken::new(
            SemanticTokenKind::Entity(EntityKind::Function {
                is_lazy: paradigm.is_lazy(),
            }),
            routine_ident.range,
        ));
        let generic_parameters = self.parameters()?;
        let parameters = self.call_parameters()?;
        let output_ty = self.func_output_type()?;
        match paradigm {
            Paradigm::Procedural => (),
            Paradigm::EagerFunctional => {
                for input_placeholder in parameters.iter() {
                    match input_placeholder.contract {
                        InputLiason::Pure | InputLiason::GlobalRef | InputLiason::Move => (),
                        InputLiason::BorrowMut | InputLiason::MoveMut => {
                            todo!("report invalid input contract")
                        }
                        InputLiason::MemberAccess => todo!(),
                    }
                }
            }
            Paradigm::LazyFunctional => todo!(),
        }
        Ok(CallableDefnHead {
            ident: routine_ident,
            paradigm,
            generic_parameters: generic_parameters,
            parameters: parameters,
            output_ty,
            output_liason: OutputLiason::Transfer,
            opt_this_contract: None,
        })
    }

    pub fn method_defn_head(
        mut self,
        this: InputLiason,
        paradigm: Paradigm,
    ) -> AtomResult<CallableDefnHead> {
        let routine_ident = get!(self, custom_ident);
        self.push_abs_semantic_token(AbsSemanticToken::new(
            SemanticTokenKind::Entity(EntityKind::Function {
                is_lazy: paradigm.is_lazy(),
            }),
            routine_ident.range,
        ));
        let generics = self.parameters()?;
        let parameters = self.call_parameters()?;
        let output_ty = self.func_output_type()?;
        Ok(CallableDefnHead {
            opt_this_contract: Some(this),
            paradigm,
            ident: routine_ident,
            generic_parameters: generics,
            parameters,
            output_ty,
            output_liason: OutputLiason::Transfer,
        })
    }

    fn parameters(&mut self) -> AtomResult<IdentDict<GenericParameter>> {
        if next_matches!(self, "<") {
            match IdentDict::from_vec(comma_list![self, parameter!+, ">"]) {
                Ok(generic_parameters) => Ok(generic_parameters),
                Err(repeat) => todo!(),
            }
        } else {
            Ok(Default::default())
        }
    }

    fn parameter(&mut self) -> AtomResult<GenericParameter> {
        let ranged_ident = get!(self, custom_ident);
        let mut traits = Vec::new();
        if next_matches!(self, ":") {
            traits.push(RangedEntityRoute {
                route: get!(self, ty?),
                range: self.stream.pop_text_range(),
            });
            if next_matches!(self, "+") {
                todo!()
            }
        }
        self.push_abs_semantic_token(AbsSemanticToken::new(
            SemanticTokenKind::GenericPlaceholder,
            ranged_ident.range,
        ));
        Ok(GenericParameter {
            ident: ranged_ident.ident,
            variant: GenericPlaceholderVariant::Type { traits },
        })
    }

    fn call_parameters(&mut self) -> AtomResultArc<Vec<InputParameter>> {
        no_look_pass!(self, "(");
        Ok(Arc::new(comma_list!(self, call_input_placeholder!, ")")))
    }

    fn call_input_placeholder(&mut self) -> AtomResult<InputParameter> {
        let ident = get!(self, custom_ident);
        self.push_abs_semantic_token(AbsSemanticToken::new(
            SemanticTokenKind::Parameter,
            ident.range,
        ));
        no_look_pass!(self, ":");
        self.stream.pop_text_range();
        let ty = RangedEntityRoute {
            route: get!(self, ty?),
            range: self.stream.pop_text_range(),
        };
        Ok(InputParameter {
            ranged_ident: ident,
            contract: InputLiason::Pure,
            ranged_ty: ty,
        })
    }

    fn func_output_type(&mut self) -> AtomResult<RangedEntityRoute> {
        Ok(if next_matches!(self, "->") {
            RangedEntityRoute {
                route: get!(self, ty?),
                range: self.stream.pop_text_range(),
            }
        } else {
            RangedEntityRoute {
                route: EntityRoutePtr::Root(RootIdentifier::Void),
                range: self.stream.pop_text_range(),
            }
        })
    }
}
