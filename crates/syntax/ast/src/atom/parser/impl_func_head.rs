use std::sync::Arc;

use crate::*;
use scope::*;
use syntax_types::*;
use vm::{EagerContract, InputContract};
use word::IdentMap;

use super::*;

// inner ops
impl<'a> AtomLRParser<'a> {
    pub(crate) fn routine_decl(mut self) -> AstResult<RoutineHead> {
        let routine_name = get!(self, custom_ident);
        let space_params = self.placeholders()?;
        let input_contracts = self.func_input_placeholders()?;
        let output = self.func_output_type()?;
        Ok(RoutineHead {
            routine_name,
            generic_placeholders: space_params,
            input_placeholders: input_contracts,
            output,
        })
    }

    pub(crate) fn memb_routine_decl(
        mut self,
        this: InputContract,
        kind: RawMembRoutineKind,
    ) -> AstResult<MembRoutineHead> {
        let routine_name = get!(self, custom_ident);
        let space_params = self.placeholders()?;
        let input_contracts = self.func_input_placeholders()?;
        let output = self.func_output_type()?;
        Ok(MembRoutineHead {
            this_contract: this,
            routine_name,
            generics: space_params,
            input_placeholders: input_contracts,
            output,
            kind,
        })
    }

    fn placeholders(&mut self) -> AstResult<IdentMap<GenericPlaceholderKind>> {
        if next_matches!(self, "<") {
            match IdentMap::from_vec(comma_list![self, placeholder!+, ">"]) {
                Ok(generic_placeholders) => Ok(generic_placeholders),
                Err(repeat) => todo!(),
            }
        } else {
            Ok(Default::default())
        }
    }

    fn placeholder(&mut self) -> AstResult<(CustomIdentifier, GenericPlaceholderKind)> {
        let ident = get!(self, custom_ident);
        let mut traits = Vec::new();
        if next_matches!(self, ":") {
            traits.push(RangedScope {
                scope: get!(self, ty?),
                range: self.stream.pop_range(),
            });
            if next_matches!(self, "+") {
                todo!()
            }
        }
        Ok((ident, GenericPlaceholderKind::Type { traits }))
    }

    fn func_input_placeholders(&mut self) -> AstResultArc<Vec<InputPlaceholder>> {
        no_look_pass!(self, "(");
        Ok(Arc::new(comma_list!(self, func_input_placeholder!, ")")))
    }

    fn func_input_placeholder(&mut self) -> AstResult<InputPlaceholder> {
        let ident = get!(self, custom_ident);
        no_look_pass!(self, ":");
        let ty = RangedScope {
            scope: get!(self, ty?),
            range: self.stream.pop_range(),
        };
        Ok(InputPlaceholder {
            ident,
            contract: InputContract::Pure,
            ranged_ty: ty,
        })
    }

    fn func_output_type(&mut self) -> AstResult<RangedScope> {
        Ok(if next_matches!(self, "->") {
            RangedScope {
                scope: get!(self, ty?),
                range: self.stream.pop_range(),
            }
        } else {
            RangedScope {
                scope: ScopePtr::Builtin(BuiltinIdentifier::Void),
                range: self.stream.pop_range(),
            }
        })
    }
}
