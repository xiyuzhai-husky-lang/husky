use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_quote;
use syn::{parse_macro_input, Arm, Expr, ExprIf, ExprMatch};

pub(crate) fn unify_elabm(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input: Expr = parse_macro_input!(input);

    let transformed = match input {
        Expr::Match(expr_match) => unify_elabm_match(expr_match),
        Expr::If(expr_if) => unify_elabm_if_else(expr_if),
        other => parse_quote!(#other.eval(elaborator, heuristic)),
    };

    // Wrap the transformed expression in a closure
    let result = quote! {
        move |elaborator: &mut VdBsqElaboratorInner<'db, 'sess>,
          heuristic: &Heuristic<'_, 'db, 'sess, _>| #transformed
    };

    result.into()
}

fn unify_elabm_match(mut expr_match: ExprMatch) -> Expr {
    // Transform each arm's expression to add .eval(elaborator, heuristic)
    for arm in &mut expr_match.arms {
        let body = &arm.body;
        arm.body = Box::new(parse_quote!(#body.eval(elaborator, heuristic)));
    }
    Expr::Match(expr_match)
}

fn unify_elabm_if_else(mut expr_if: ExprIf) -> Expr {
    // Transform then branch
    let then_branch = &expr_if.then_branch;
    expr_if.then_branch = parse_quote!({ #then_branch.eval(elaborator, heuristic) });

    // Transform else branch if it exists
    if let Some((_, else_branch)) = &mut expr_if.else_branch {
        let else_expr = &else_branch;
        *else_branch = Box::new(parse_quote!(#else_expr.eval(elaborator, heuristic)));
    }
    Expr::If(expr_if)
}
