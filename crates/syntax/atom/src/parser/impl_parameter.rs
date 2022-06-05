use std::sync::Arc;

use crate::*;
use defn_head::{CallableDefnHead, GenericPlaceholderVariant, Parameter, SpatialParameter};
use entity_route::*;
use token::SemanticTokenKind;
use word::{IdentDict, Paradigm};

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
            ident: ranged_ident.ident,
            variant: GenericPlaceholderVariant::Type { traits },
        })
    }

    pub fn parameters(&mut self) -> AtomResultArc<Vec<Parameter>> {
        eat!(self, "(");
        Ok(Arc::new(comma_list!(self, parameter!, ")")))
    }

    pub fn parameter(&mut self) -> AtomResult<Parameter> {
        let ident = get!(self, custom_ident);
        self.atom_context
            .push_abs_semantic_token(AbsSemanticToken::new(
                SemanticTokenKind::Parameter,
                ident.range,
            ));
        eat!(self, ":");
        let ty = get!(self, ranged_ty?);
        Ok(Parameter::new(ident, ty))
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
