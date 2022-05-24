use std::sync::Arc;

use crate::*;
use defn_head::{
    GenericPlaceholder, GenericPlaceholderVariant, InputParameter, RoutineDefnHead,
    TypeMethodDefnHead,
};
use entity_route::*;
use token::SemanticTokenKind;
use vm::{InputLiason, OutputLiason};
use word::{IdentDict, RoutineKeyword};

use super::*;

// inner ops
impl<'a> AtomParser<'a> {
    pub fn routine_defn_head(
        mut self,
        routine_keyword: RoutineKeyword,
    ) -> AtomResult<RoutineDefnHead> {
        let routine_ident = get!(self, custom_ident);
        self.push_abs_semantic_token(AbsSemanticToken::new(
            SemanticTokenKind::Entity(EntityKind::Routine),
            routine_ident.range,
        ));
        let generic_placeholders = self.parameters()?;
        let input_placeholders = self.call_input_placeholders()?;
        let output_ty = self.func_output_type()?;
        match routine_keyword {
            RoutineKeyword::Proc => (),
            RoutineKeyword::Func => {
                for input_placeholder in input_placeholders.iter() {
                    match input_placeholder.contract {
                        InputLiason::Pure | InputLiason::GlobalRef | InputLiason::Move => (),
                        InputLiason::BorrowMut | InputLiason::MoveMut => {
                            todo!("report invalid input contract")
                        }
                        InputLiason::Exec => todo!(),
                        InputLiason::MemberAccess => todo!(),
                    }
                }
            }
        }
        Ok(RoutineDefnHead {
            ident: routine_ident,
            routine_kind: routine_keyword,
            generic_placeholders,
            parameters: input_placeholders,
            output_ty,
            output_liason: OutputLiason::Transfer,
        })
    }

    pub fn method_decl(
        mut self,
        this: InputLiason,
        routine_kind: RoutineKeyword,
    ) -> AtomResult<TypeMethodDefnHead> {
        let routine_ident = get!(self, custom_ident);
        self.push_abs_semantic_token(AbsSemanticToken::new(
            SemanticTokenKind::Entity(EntityKind::Routine),
            routine_ident.range,
        ));
        let generics = self.parameters()?;
        let input_placeholders = self.call_input_placeholders()?;
        let output_ty = self.func_output_type()?;
        Ok(TypeMethodDefnHead {
            this_contract: this,
            routine_kind,
            ident: routine_ident,
            generic_placeholders: generics,
            input_placeholders,
            output_ty,
            output_liason: OutputLiason::Transfer,
        })
    }

    fn parameters(&mut self) -> AtomResult<IdentDict<GenericPlaceholder>> {
        if next_matches!(self, "<") {
            match IdentDict::from_vec(comma_list![self, parameter!+, ">"]) {
                Ok(generic_placeholders) => Ok(generic_placeholders),
                Err(repeat) => todo!(),
            }
        } else {
            Ok(Default::default())
        }
    }

    fn parameter(&mut self) -> AtomResult<GenericPlaceholder> {
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
        Ok(GenericPlaceholder {
            ident: ranged_ident.ident,
            variant: GenericPlaceholderVariant::Type { traits },
        })
    }

    fn call_input_placeholders(&mut self) -> AtomResultArc<Vec<InputParameter>> {
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
