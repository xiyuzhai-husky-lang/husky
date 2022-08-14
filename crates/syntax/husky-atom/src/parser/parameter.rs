use std::sync::Arc;

use crate::*;
use defn_head::{Parameter, SpatialParameter, SpatialParameterVariant};
use husky_entity_route::*;
use husky_print_utils::msg_once;
use husky_token::SemanticTokenKind;
use husky_word::{IdentDict, Paradigm};

use super::*;

// inner ops
impl<'a, 'b> AtomParser<'a, 'b> {
    // pub fn method_defn_head(
    //     mut self,
    //     this: ParameterLiason,
    //     paradigm: Paradigm,
    // ) -> AtomResult<CallableDefnHead> {
    //     let routine_ident = get!(self, custom_ident);
    //     self.atom_context
    //         .push_abs_semantic_token(AbsSemanticToken::new(
    //             SemanticTokenKind::Entity(EntityKind::Function {
    //                 is_lazy: paradigm.is_lazy(),
    //             }),
    //             routine_ident.range,
    //         ));
    //     let generics = self.generic_parameters()?;
    //     let parameters = self.parameters()?;
    //     let output_ty = self.func_output_type()?;
    //     Ok(CallableDefnHead {
    //         opt_this_liason: Some(this),
    //         paradigm,
    //         ident: routine_ident,
    //         generic_parameters: generics,
    //         parameters,
    //         output_ty,
    //         output_liason: OutputLiason::Transfer,
    //     })
    // }

    pub fn generic_parameters(&mut self) -> AtomResult<IdentDict<SpatialParameter>> {
        if try_eat!(self, "<") {
            match IdentDict::from_vec(comma_list![self, spatial_parameter!+, ">"]) {
                Ok(generic_parameters) => Ok(generic_parameters),
                Err(repeat) => todo!(),
            }
        } else {
            Ok(Default::default())
        }
    }

    pub fn spatial_parameter(&mut self) -> AtomResult<SpatialParameter> {
        let ranged_ident = get!(self, custom_ident);
        let mut traits = Vec::new();
        if try_eat!(self, ":") {
            traits.push(get!(self, ranged_ty?));
            if try_eat!(self, "+") {
                todo!()
            }
        }
        self.atom_context
            .push_abs_semantic_token(AbsSemanticToken::new(
                SemanticTokenKind::GenericPlaceholder,
                ranged_ident.range,
            ));
        Ok(SpatialParameter {
            ident: ranged_ident,
            variant: SpatialParameterVariant::Type { traits },
            file: self.atom_context.file(),
            range: ranged_ident.range,
        })
    }

    // pub fn parameters(&mut self) -> AtomResultArc<Vec<Parameter>> {
    //     eat_special!(self, "(");
    //     Ok(Arc::new(comma_list!(self, parameter!, ")")))
    // }

    pub fn parameter(&mut self) -> AtomResult<Parameter> {
        let ident = get!(self, custom_ident);
        self.atom_context
            .push_abs_semantic_token(AbsSemanticToken::new(
                SemanticTokenKind::Parameter,
                ident.range,
            ));
        eat_special!(self, ":");
        let ranged_parameter_liason = self.ranged_parameter_liason();
        let ranged_ty = get!(self, ranged_ty?);
        Ok(Parameter::new(ident, ranged_parameter_liason, ranged_ty))
    }

    pub fn ranged_parameter_liason(&mut self) -> RangedParameterLiason {
        let text_start = self.token_stream.text_start();
        let liason = if try_eat!(self, "&") {
            msg_once!("todo: temporal parameter");
            ParameterLiason::EvalRef
        } else if try_eat!(self, "mut") {
            if try_eat!(self, "!!") {
                ParameterLiason::MoveMut
            } else {
                ParameterLiason::TempRefMut
            }
        } else if try_eat!(self, "!!") {
            ParameterLiason::Move
        } else {
            return ParameterLiason::Pure.into();
        };
        RangedParameterLiason {
            liason,
            opt_range: Some(self.token_stream.text_range(text_start)),
        }
    }

    pub fn func_output_ty(&mut self) -> AtomResult<RangedEntityRoute> {
        Ok(if try_eat!(self, "->") {
            get!(self, ranged_ty?)
        } else {
            RangedEntityRoute {
                route: EntityRoutePtr::Root(RootIdentifier::Void),
                range: self.token_stream.next_range(),
            }
        })
    }
}

pub struct BracketedParametersPattern;
impl AtomParserPattern for BracketedParametersPattern {
    type Output = Arc<Vec<Parameter>>;

    fn get_parsed(&self, parser: &mut AtomParser) -> AtomResult<Option<Self::Output>> {
        eat_special!(parser, "(");
        Ok(Some(Arc::new(comma_list!(parser, parameter!, ")"))))
    }
}
