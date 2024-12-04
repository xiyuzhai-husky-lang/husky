use crate::*;
use convert_case::{Case, Casing};

pub(crate) fn memo(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = match syn::parse::<MemoAttr>(attr) {
        Ok(attr) => attr,
        Err(err) => return err.into_compile_error().into(),
    };
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
    let args_with_lifetimes = args
        .iter()
        .map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                let pat = &pat_type.pat;
                let ty = &*pat_type.ty;
                match ty {
                    Type::Path(path) => {
                        let last_segment = path.path.segments.last().unwrap();

                        // Check if it already has a lifetime parameter
                        match &last_segment.arguments {
                            syn::PathArguments::AngleBracketed(_) => quote!(#pat: #ty),
                            _ => quote!(#pat: #ty<'db>),
                        }
                    }
                    _ => panic!("expect path type"),
                }
            } else {
                panic!("Self arguments not supported")
            }
        })
        .collect::<Vec<_>>();
    let arg_tys_with_static_lifetimes = args
        .iter()
        .map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                let pat = &pat_type.pat;
                let ty = &*pat_type.ty;
                match ty {
                    Type::Path(path) => {
                        let last_segment = path.path.segments.last().unwrap();

                        // Check if it already has a lifetime parameter
                        match &last_segment.arguments {
                            syn::PathArguments::AngleBracketed(args) => {
                                // Replace existing lifetime parameters with 'static
                                let mut new_args = args.clone();
                                for arg in new_args.args.iter_mut() {
                                    if let syn::GenericArgument::Lifetime(_) = arg {
                                        *arg = syn::parse_quote!('static);
                                    }
                                }
                                let new_ty = Type::Path(TypePath {
                                    qself: path.qself.clone(),
                                    path: path.path.clone(),
                                });
                                quote!(#new_ty)
                            }
                            _ => quote!(#ty<'static>),
                        }
                    }
                    _ => panic!("expect path type"),
                }
            } else {
                panic!("Self arguments not supported")
            }
        })
        .collect::<Vec<_>>();
    let db_ty = if let FnArg::Typed(pat_type) = db_arg {
        if let Type::Path(type_path) = &*pat_type.ty {
            if let Some(last_segment) = type_path.path.segments.last() {
                if last_segment.ident.to_string() != "BerserkerDb" {
                    panic!("expect last arg to be db:BerserkerDb");
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
        0 => quote!(::berserk::memo::jar0::Jar0<#ret_type>),
        _ => quote!(::berserk::memo::jar::Jar<(#(#arg_tys_with_static_lifetimes),*), #ret_type>),
    };

    let output = if attr.return_ref {
        quote! {
                #[allow(non_camel_case_types)]
                struct #fn_name {}

            impl ::berserk::memo::IsMemo for #fn_name {
                type Jar = #jar_ty;
            }

            #vis fn #fn_name<'db>(#(#args,)* db: &'db ::berserk::db::BerserkerDb) -> &'db #ret_type  {
                fn #inner_fn_name<'db>(#(#args,)* db: &'db ::berserk::db::BerserkerDb) -> #ret_type #body

                db.memo_jar::<#fn_name>().get_or_alloc((#(#arg_names),*), || #inner_fn_name(#(#arg_names,)* db))
            }
        }
    } else {
        quote! {
                #[allow(non_camel_case_types)]
                struct #fn_name {}

            impl ::berserk::memo::IsMemo for #fn_name {
                type Jar = #jar_ty;
            }


            #vis fn #fn_name<'db>(#(#args_with_lifetimes,)* db: &'db ::berserk::db::BerserkerDb) -> #ret_type {
                fn #inner_fn_name<'db>(#(#args_with_lifetimes,)* db: &::berserk::db::BerserkerDb) -> #ret_type #body

                let key = unsafe { std::mem::transmute::<_, (#(#arg_tys_with_static_lifetimes),*)>((#(#arg_names),*)) };
                *db.memo_jar::<#fn_name>().get_or_alloc(key, || #inner_fn_name(#(#arg_names,)* db))
            }
        }
    };

    output.into()
}

#[derive(Default)]
struct MemoAttr {
    return_ref: bool,
}

impl syn::parse::Parse for MemoAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::ext::IdentExt;

        let mut slf = Self::default();
        while !input.is_empty() {
            let ident: syn::Ident = syn::Ident::parse_any(input)?;
            if ident == "return_ref" {
                slf.return_ref = true;
            } else {
                todo!()
            }
        }
        Ok(slf)
    }
}
