use std::sync::Arc;

use crate::*;
use defn_head::{
    GenericPlaceholder, GenericPlaceholderVariant, InputPlaceholder, RoutineDefnHead,
    TypeMethodDefnHead,
};
use entity_route::*;
use token::SemanticTokenKind;
use vm::{InputContract, OutputContract};
use word::IdentDict;

use super::*;

// inner ops
impl<'a> AtomLRParser<'a> {
    pub fn routine_defn_head(
        mut self,
        routine_kind: RoutineContextKind,
    ) -> AtomResult<RoutineDefnHead> {
        let routine_ident = get!(self, custom_ident);
        self.push_abs_semantic_token(AbsSemanticToken::new(
            SemanticTokenKind::Entity(EntityKind::Routine),
            routine_ident.range,
        ));
        let generic_placeholders = self.placeholders()?;
        let input_placeholders = self.call_input_placeholders()?;
        let output_ty = self.func_output_type()?;
        match routine_kind {
            RoutineContextKind::Proc => (),
            RoutineContextKind::Test => {
                todo!()
            }
            RoutineContextKind::Func => {
                for input_placeholder in input_placeholders.iter() {
                    match input_placeholder.contract {
                        InputContract::Pure | InputContract::GlobalRef | InputContract::Move => (),
                        InputContract::BorrowMut | InputContract::MoveMut => {
                            todo!("report invalid input contract")
                        }
                        InputContract::Exec => todo!(),
                        InputContract::MemberAccess => todo!(),
                    }
                }
            }
        }
        Ok(RoutineDefnHead {
            ident: routine_ident,
            routine_kind,
            generic_placeholders,
            input_placeholders,
            output_ty,
            output_contract: OutputContract::Transitive,
        })
    }

    pub fn method_decl(
        mut self,
        this: InputContract,
        routine_kind: RoutineContextKind,
    ) -> AtomResult<TypeMethodDefnHead> {
        let routine_name = get!(self, custom_ident);
        let generics = self.placeholders()?;
        let input_placeholders = self.call_input_placeholders()?;
        let output_ty = self.func_output_type()?;
        Ok(TypeMethodDefnHead {
            this_contract: this,
            routine_kind,
            ident: routine_name,
            generic_placeholders: generics,
            input_placeholders,
            output_ty,
            output_contract: OutputContract::Transitive,
        })
    }

    fn placeholders(&mut self) -> AtomResult<IdentDict<GenericPlaceholder>> {
        if next_matches!(self, "<") {
            match IdentDict::from_vec(comma_list![self, placeholder!+, ">"]) {
                Ok(generic_placeholders) => Ok(generic_placeholders),
                Err(repeat) => todo!(),
            }
        } else {
            Ok(Default::default())
        }
    }

    fn placeholder(&mut self) -> AtomResult<GenericPlaceholder> {
        let ranged_ident = get!(self, custom_ident);
        let mut traits = Vec::new();
        if next_matches!(self, ":") {
            traits.push(RangedEntityRoute {
                route: get!(self, ty?),
                range: self.stream.pop_range(),
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

    fn call_input_placeholders(&mut self) -> AtomResultArc<Vec<InputPlaceholder>> {
        no_look_pass!(self, "(");
        Ok(Arc::new(comma_list!(self, call_input_placeholder!, ")")))
    }

    fn call_input_placeholder(&mut self) -> AtomResult<InputPlaceholder> {
        let ident = get!(self, custom_ident);
        self.push_abs_semantic_token(AbsSemanticToken::new(
            SemanticTokenKind::Parameter,
            ident.range,
        ));
        no_look_pass!(self, ":");
        self.stream.pop_range();
        let ty = RangedEntityRoute {
            route: get!(self, ty?),
            range: self.stream.pop_range(),
        };
        Ok(InputPlaceholder {
            ident,
            contract: InputContract::Pure,
            ranged_ty: ty,
        })
    }

    fn func_output_type(&mut self) -> AtomResult<RangedEntityRoute> {
        Ok(if next_matches!(self, "->") {
            RangedEntityRoute {
                route: get!(self, ty?),
                range: self.stream.pop_range(),
            }
        } else {
            RangedEntityRoute {
                route: EntityRoutePtr::Root(RootIdentifier::Void),
                range: self.stream.pop_range(),
            }
        })
    }
}
