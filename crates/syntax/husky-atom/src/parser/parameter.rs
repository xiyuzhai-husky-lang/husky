use std::sync::Arc;

use crate::*;
use husky_defn_head::{Parameter, SpatialParameter, SpatialParameterVariant};
use husky_entity_route::*;
use husky_print_utils::msg_once;
use husky_token::SemanticTokenKind;
use husky_word::{IdentDict, Paradigm};

use super::*;

// inner ops
impl<'a, 'b, 'c> AtomParser<'a, 'b, 'c> {
    pub fn spatial_parameters(&mut self) -> AtomResult<IdentDict<SpatialParameter>> {
        if try_eat_special!(self, "<") {
            let spatial_parameters = get_patt!(
                self,
                CommaListPattern {
                    item: SpatialParameterPattern,
                    terminator: be_special_token_patt!(">")
                }
            );
            if spatial_parameters.len() == 0 {
                todo!()
            }
            match IdentDict::from_vec(spatial_parameters) {
                Ok(spatial_parameters) => Ok(spatial_parameters),
                Err(repeat) => todo!(),
            }
        } else {
            Ok(Default::default())
        }
    }

    pub fn parameter(&mut self) -> AtomResult<Parameter> {
        let ranged_parameter_liason = self.ranged_parameter_liason();
        let ident = deprecated_get!(self, custom_ident);
        self.atom_context
            .push_abs_semantic_token(AbsSemanticToken::new(
                SemanticTokenKind::Parameter,
                ident.range,
            ));
        eat_special!(self, ":");
        let ranged_ty = deprecated_get!(self, ranged_ty?);
        Ok(Parameter::new(
            self.atom_context.entity_syntax_db(),
            ident,
            ranged_parameter_liason,
            ranged_ty,
        ))
    }

    pub fn ranged_parameter_liason(&mut self) -> RangedParameterLiason {
        let text_start = self.token_stream.text_start();
        let liason = if deprecated_try_eat!(self, "&") {
            todo!()
        } else if deprecated_try_eat!(self, "mut") {
            if deprecated_try_eat!(self, "!!") {
                ParameterModifier::OwnedMut
            } else {
                ParameterModifier::TempRefMut
            }
        } else if deprecated_try_eat!(self, "!!") {
            ParameterModifier::Owned
        } else {
            return ParameterModifier::None.into();
        };
        RangedParameterLiason {
            liason,
            opt_range: Some(self.token_stream.text_range(text_start)),
        }
    }

    pub fn func_output_ty(&mut self) -> AtomResult<RangedEntityRoute> {
        Ok(if deprecated_try_eat!(self, "->") {
            deprecated_get!(self, ranged_ty?)
        } else {
            RangedEntityRoute {
                route: EntityRoutePtr::Root(RootIdentifier::Void),
                range: self.token_stream.next_range(),
            }
        })
    }
}

pub struct SpatialParameterPattern;
impl std::fmt::Display for SpatialParameterPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "spatial parameter".fmt(f)
    }
}
impl AtomParserPattern for SpatialParameterPattern {
    type Output = SpatialParameter;

    fn get_parsed(&self, parser: &mut AtomParser) -> AtomResult<Option<Self::Output>> {
        let ranged_ident = deprecated_get!(parser, custom_ident);
        let mut traits = Vec::new();
        if try_eat_special!(parser, ":") {
            traits.push(deprecated_get!(parser, ranged_ty?));
            if try_eat_special!(parser, "+") {
                todo!()
            }
        }
        parser
            .atom_context
            .push_abs_semantic_token(AbsSemanticToken::new(
                SemanticTokenKind::GenericPlaceholder,
                ranged_ident.range,
            ));
        Ok(Some(SpatialParameter {
            ident: ranged_ident,
            variant: SpatialParameterVariant::Type { traits },
            file: parser.atom_context.file(),
            range: ranged_ident.range,
        }))
    }
}

pub struct BracketedParametersPattern;
impl std::fmt::Display for BracketedParametersPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "bracketed parameters".fmt(f)
    }
}
impl AtomParserPattern for BracketedParametersPattern {
    type Output = Arc<Vec<Parameter>>;

    fn get_parsed(&self, parser: &mut AtomParser) -> AtomResult<Option<Self::Output>> {
        eat_special!(parser, "(");
        Ok(Some(Arc::new(
            CommaListPattern {
                item: ParameterPattern,
                terminator: be_special_token_patt!(")"),
            }
            .get_parsed(parser)?
            .unwrap(),
        )))
    }
}

pub struct ParameterPattern;
impl std::fmt::Display for ParameterPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "parameter".fmt(f)
    }
}
impl AtomParserPattern for ParameterPattern {
    type Output = Parameter;

    fn get_parsed(&self, parser: &mut AtomParser) -> AtomResult<Option<Self::Output>> {
        let ranged_parameter_liason = parser.ranged_parameter_liason();
        let ident = deprecated_get!(parser, custom_ident);
        parser
            .atom_context
            .push_abs_semantic_token(AbsSemanticToken::new(
                SemanticTokenKind::Parameter,
                ident.range,
            ));
        eat_special!(parser, ":");
        let ranged_ty = deprecated_get!(parser, ranged_ty?);
        Ok(Some(Parameter::new(
            parser.db(),
            ident,
            ranged_parameter_liason,
            ranged_ty,
        )))
    }
}
