use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_quote;
use syn::{parse_macro_input, Arm, Expr, ExprMatch};

pub(crate) fn unify_elabm(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input as a match expression
    let mut expr_match = parse_macro_input!(input as ExprMatch);

    // Transform each arm's expression to add .eval(elaborator, heuristic)
    for arm in &mut expr_match.arms {
        let body = &arm.body;
        arm.body = Box::new(parse_quote!(#body.eval(elaborator, heuristic)));
    }

    // Wrap the transformed match in a closure
    let result = quote! {
        move |elaborator: &mut VdBsqElaboratorInner<'db, 'sess>,
          heuristic: &Heuristic<'_, 'db, 'sess, VdBsqSumBuilder<'sess>>| #expr_match
    };

    result.into()
}
