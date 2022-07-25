mod builder;
mod query;
mod sheet;

pub use query::*;
pub use sheet::*;

use defn_head::*;
use husky_ast::*;
use husky_check_utils::*;
use husky_entity_route::*;
use husky_entity_syntax::{EntitySyntaxQueryGroup, EntitySyntaxResultArc};
use husky_file::FilePtr;
use husky_print_utils::*;
use husky_word::RootIdentifier;
use infer_decl::{CallFormDecl, DeclQueryGroup, TyDecl};
use infer_error::*;
use vm::{ListOpr, RawOpnVariant};

pub trait InferEntityRoute {
    fn decl_db(&self) -> &dyn DeclQueryGroup;
    fn entity_route_sheet(&self) -> &EntityRouteSheet;
    fn raw_expr_ty(&self, idx: RawExprIdx) -> InferResult<EntityRoutePtr> {
        self.entity_route_sheet().expr_ty_result(idx)
    }
    fn raw_expr_intrinsic_ty(&self, idx: RawExprIdx) -> InferResult<EntityRoutePtr> {
        self.entity_route_sheet()
            .expr_ty_result(idx)
            .map(|ty| ty.intrinsic())
    }
    fn raw_expr_deref_ty_decl(&self, idx: RawExprIdx) -> InferResultArc<TyDecl> {
        let ty = self.raw_expr_intrinsic_ty(idx)?;
        Ok(derived_unwrap!(self.decl_db().ty_decl(ty)))
    }

    // fn call_route_result(&self, idx: RawExprIdx) -> InferResult<EntityRoutePtr> {
    //     self.entity_route_sheet().call_route(idx)
    // }

    fn function_call_form_decl(&self, function_idx: RawExprIdx) -> InferResultArc<CallFormDecl> {
        if let Some(call_route_result) = self
            .entity_route_sheet()
            .opt_function_call_route(function_idx)
        {
            Ok(derived_unwrap!(self
                .decl_db()
                .entity_call_form_decl(call_route_result?)))
        } else {
            Ok(derived_unwrap!(self
                .decl_db()
                .value_call_form_decl(self.raw_expr_ty(function_idx)?)))
        }
    }

    fn method_call_form_decl(&self, this_idx: RawExprIdx) -> InferResultArc<CallFormDecl> {
        let call_route_result =
            derived_not_none!(self.entity_route_sheet().opt_method_call_route(this_idx))?;
        Ok(derived_unwrap!(self
            .decl_db()
            .entity_call_form_decl(call_route_result?)))
    }
}
