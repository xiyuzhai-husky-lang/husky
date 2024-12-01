use crate::*;
use convert_case::{Case, Casing};

pub(crate) fn memo(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _attr = attr;
    let input = parse_macro_input!(item as ItemFn);
    let vis = input.vis;
    let sig = input.sig;
    let body = input.block;

    let fn_name = &sig.ident;
    let storage_name = format_ident!("{}_STORAGE", fn_name.to_string().to_uppercase());
    let inner_fn_name = format_ident!("__{}", fn_name);
    let ret_type = match &sig.output {
        ReturnType::Default => quote!(()),
        ReturnType::Type(_, ty) => quote!(#ty),
    };
    let args = sig
        .inputs
        .iter()
        .take(sig.inputs.len() - 1)
        .collect::<Vec<_>>();
    let db_arg = sig.inputs.last().unwrap();
    let arg_tys = args
        .iter()
        .map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                &*pat_type.ty
            } else {
                panic!("Self arguments not supported")
            }
        })
        .collect::<Vec<_>>();
    let db_ty = if let FnArg::Typed(pat_type) = db_arg {
        if let Type::Path(type_path) = &*pat_type.ty {
            if let Some(last_segment) = type_path.path.segments.last() {
                if last_segment.ident.to_string() != "EternerDb" {
                    panic!("expect last arg to be db:EternerDb");
                }
            }
        }
        &*pat_type.ty
    } else {
        panic!("DB argument must be typed")
    };
    let arg_names = args
        .iter()
        .map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    &pat_ident.ident
                } else {
                    panic!("Unsupported argument pattern")
                }
            } else {
                panic!("Self arguments not supported")
            }
        })
        .collect::<Vec<_>>();

    let jar_ty = match args.len() {
        0 => quote!(::eterned::memo::jar0::Jar0<#ret_type>),
        _ => quote!(::eterned::memo::jar::Jar<(#(#arg_tys),*), #ret_type>),
    };

    let output = quote! {
        #[allow(non_camel_case_types)]
        struct #fn_name {}

        impl ::eterned::memo::IsMemo for #fn_name {
            type Jar = #jar_ty;
        }

        #vis fn #fn_name<'db>(#(#args,)* db: &'db ::eterned::db::EternerDb) -> &'db #ret_type  {
            fn #inner_fn_name(#(#args,)* db: &::eterned::db::EternerDb) -> #ret_type #body

            let __jar = db.memo_jar::<#fn_name>();

            if let Some(result) = __jar.get(&(#(#arg_names),*)) {
                return result;
            }

            let result = #inner_fn_name(#(#arg_names,)* db);
            __jar.alloc((#(#arg_names),*), result)
        }
    };

    output.into()
}
