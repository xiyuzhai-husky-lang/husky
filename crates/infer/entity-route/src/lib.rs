mod builder;
mod query;
mod sheet;

pub use query::*;
pub use sheet::*;

use ast::*;
use check_utils::*;
use defn_head::*;
use entity_route::*;
use entity_syntax::{EntityRouteQueryGroup, EntitySyntaxResultArc};
use file::FilePtr;
use infer_decl::{CallDecl, DeclQueryGroup, MethodDecl, TyDecl};
use infer_error::*;
use print_utils::*;
use word::RootIdentifier;

pub trait InferEntityRoute {
    fn decl_db(&self) -> &dyn DeclQueryGroup;
    fn entity_route_sheet(&self) -> &EntityRouteSheet;
    fn raw_expr_ty(&self, raw_expr_idx: RawExprIdx) -> InferResult<EntityRoutePtr> {
        emsg_once!("deprecated");
        self.entity_route_sheet().expr_ty_result(raw_expr_idx)
    }
    fn raw_expr_ty_decl(&self, raw_expr_idx: RawExprIdx) -> InferResultArc<TyDecl> {
        let ty = self.raw_expr_ty(raw_expr_idx)?;
        Ok(derived_unwrap!(self.decl_db().ty_decl(ty)))
    }

    fn call_route_result(&self, raw_expr_idx: RawExprIdx) -> InferResult<EntityRoutePtr> {
        self.entity_route_sheet().call_route(raw_expr_idx)
    }

    fn call_decl(&self, raw_expr_idx: RawExprIdx) -> InferResultArc<CallDecl> {
        Ok(derived_unwrap!(self
            .decl_db()
            .call_decl(self.call_route_result(raw_expr_idx)?)))
    }

    fn method_decl(&self, raw_expr_idx: RawExprIdx) -> InferResultArc<MethodDecl> {
        self.decl_db()
            .method_decl(self.call_route_result(raw_expr_idx)?)
    }
}
