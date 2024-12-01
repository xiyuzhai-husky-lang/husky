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

    let output = quote! {
        #vis fn #fn_name(#(#args,)* db: &::eterned::db::EternerDb) -> &'static #ret_type  {
            eterned::lazy_static! {
                static ref #storage_name: eterned::DashMap<(#(#arg_tys),*), Box<#ret_type>> = eterned::DashMap::new();
            }

            fn #inner_fn_name(#(#args,)* db: &::eterned::db::EternerDb) -> #ret_type #body

            if let Some(result) = #storage_name.get(&(#(#arg_names),*)) {
                return unsafe { &*(&**result as *const #ret_type)};
            }

            let result = #inner_fn_name(#(#arg_names,)* db);
            let result = Box::new(result);
            let result_ptr = &*result as *const #ret_type;
            #storage_name.insert((#(#arg_names),*), result);
            let result_ref:&'static #ret_type = unsafe { &*result_ptr };
            result_ref
        }
    };

    output.into()
}
