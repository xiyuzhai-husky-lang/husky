use std::sync::Arc;

use crate::*;
use syntax_types::*;
use vm::InputContract;

use super::*;

// inner ops
impl<'a> AtomLRParser<'a> {
    pub(crate) fn func_decl(mut self) -> AstResult<FuncDecl> {
        let funcname = get!(self, custom_ident);
        let space_params = self.placeholders()?;
        let input_contracts = self.func_input_contracts()?;
        let output = self.func_output_type()?;
        Ok(FuncDecl {
            funcname,
            generics: space_params,
            input_contracts,
            output,
        })
    }

    fn placeholders(&mut self) -> AstResult<Vec<GenericPlaceholder>> {
        if next_matches!(self, "<") {
            Ok(comma_list![self, placeholder!+, ">"])
        } else {
            Ok(Vec::new())
        }
    }

    fn placeholder(&mut self) -> AstResult<GenericPlaceholder> {
        let ident = get!(self, custom_ident);
        let mut traits = Vec::new();
        if next_matches!(self, ":") {
            traits.push(get!(self, ty?));
            if next_matches!(self, "+") {
                todo!()
            }
        }
        Ok(GenericPlaceholder {
            ident,
            kind: GenericPlaceholderKind::Type { traits },
        })
    }

    fn func_input_contracts(&mut self) -> AstResultArc<Vec<(CustomIdentifier, InputType)>> {
        no_look_pass!(self, "(");
        Ok(Arc::new(comma_list!(self, func_input_type!, ")")))
    }

    fn func_input_type(&mut self) -> AstResult<(CustomIdentifier, InputType)> {
        let ident = get!(self, custom_ident);
        no_look_pass!(self, ":");
        let ty = get!(self, ty?);
        Ok((
            ident,
            InputType {
                contract: InputContract::Intact,
                ty,
            },
        ))
    }

    fn func_output_type(&mut self) -> AstResult<ScopePtr> {
        Ok(if next_matches!(self, "->") {
            get!(self, ty?)
        } else {
            ScopePtr::Builtin(BuiltinIdentifier::Void)
        })
    }
}
