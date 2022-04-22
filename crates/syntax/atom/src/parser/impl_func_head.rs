use std::sync::Arc;

use crate::*;
use defn_head::{
    GenericPlaceholder, GenericPlaceholderVariant, InputPlaceholder, RoutineDefnHead,
    TypeMethodDefnHead,
};
use entity_route::*;
use print_utils::msg_once;
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
        let generic_placeholders = self.placeholders()?;
        let input_placeholders = self.func_input_placeholders()?;
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
        msg_once!("output contract");
        Ok(RoutineDefnHead {
            ident: routine_ident,
            routine_kind,
            generic_placeholders,
            input_placeholders,
            output_ty,
            output_contract: OutputContract::Pure,
        })
    }

    pub fn method_decl(
        mut self,
        this: InputContract,
        routine_kind: RoutineContextKind,
    ) -> AtomResult<TypeMethodDefnHead> {
        let routine_name = get!(self, custom_ident);
        let generics = self.placeholders()?;
        let input_placeholders = self.func_input_placeholders()?;
        let output_ty = self.func_output_type()?;
        Ok(TypeMethodDefnHead {
            this_contract: this,
            routine_kind,
            ident: routine_name,
            generic_placeholders: generics,
            input_placeholders,
            output_ty,
            output_contract: OutputContract::Pure,
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
        let ident = get!(self, custom_ident);
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
        Ok(GenericPlaceholder {
            ident,
            variant: GenericPlaceholderVariant::Type { traits },
        })
    }

    fn func_input_placeholders(&mut self) -> AtomResultArc<Vec<InputPlaceholder>> {
        no_look_pass!(self, "(");
        Ok(Arc::new(comma_list!(self, func_input_placeholder!, ")")))
    }

    fn func_input_placeholder(&mut self) -> AtomResult<InputPlaceholder> {
        let ident = get!(self, custom_ident);
        no_look_pass!(self, ":");
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
