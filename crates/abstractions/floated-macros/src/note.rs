use crate::*;
use convert_case::{Case, Casing};

pub(crate) fn note(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = match syn::parse::<NoteAttr>(attr) {
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
    let ess_args = sig
        .inputs
        .iter()
        .take(sig.inputs.len() - 1)
        .collect::<Vec<_>>();
    let db_arg = sig.inputs.last().unwrap();
    let ess_arg_tys = ess_args
        .iter()
        .map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                &*pat_type.ty
            } else {
                panic!("Self arguments not supported")
            }
        })
        .collect::<Vec<_>>();
    let static_ess_arg_tys = ess_args
        .iter()
        .map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                match &*pat_type.ty {
                    Type::Path(type_path) => {
                        let mut type_path = type_path.clone();
                        for segment in &mut type_path.path.segments {
                            if let PathArguments::AngleBracketed(args) = &mut segment.arguments {
                                for arg in &mut args.args {
                                    if let GenericArgument::Lifetime(_) = arg {
                                        *arg = GenericArgument::Lifetime(Lifetime::new(
                                            "'static",
                                            proc_macro2::Span::call_site(),
                                        ));
                                    }
                                }
                            }
                        }
                        Type::Path(type_path)
                    }
                    _ => panic!("Type not supported"),
                }
            } else {
                panic!("Self arguments not supported")
            }
        })
        .collect::<Vec<_>>();
    let db_ty = if let FnArg::Typed(pat_type) = db_arg {
        if let Type::Path(type_path) = &*pat_type.ty {
            if let Some(last_segment) = type_path.path.segments.last() {
                if last_segment.ident.to_string() != "FloaterDb" {
                    panic!("expect last arg to be db:FloaterDb");
                }
            }
        }
        &*pat_type.ty
    } else {
        panic!("DB argument must be typed")
    };
    let ess_arg_names = ess_args
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

    let jar_ty = match ess_args.len() {
        0 => quote!(::floated::note::jar0::Jar0<#ret_type>),
        _ => quote!(::floated::note::jar::Jar<(#(#static_ess_arg_tys),*), #ret_type>),
    };

    let output = if attr.return_ref {
        quote! {
                #[allow(non_camel_case_types)]
                struct #fn_name {}

            impl ::floated::note::IsNote for #fn_name {
                type Jar = #jar_ty;
            }

            #vis fn #fn_name<'db>(#(#ess_args,)* db: &'db ::floated::db::FloaterDb) -> &'db #ret_type  {
                fn #inner_fn_name<'db>(#(#ess_args,)* db: &'db ::floated::db::FloaterDb) -> #ret_type #body

                db.note_jar::<#fn_name>().get_or_alloc((#(#ess_arg_names),*), || #inner_fn_name(#(#ess_arg_names,)* db))
            }
        }
    } else {
        quote! {
                #[allow(non_camel_case_types)]
                struct #fn_name {}

            impl ::floated::note::IsNote for #fn_name {
                type Jar = #jar_ty;
            }


            #vis fn #fn_name<'db>(#(#ess_args,)* db: &'db ::floated::db::FloaterDb) -> #ret_type {
                use floated::arb_ref;

                fn #inner_fn_name<'db>(#(#ess_args,)* db: &'db ::floated::db::FloaterDb) -> #ret_type #body

                unsafe {
                    *db.note_jar::<#fn_name>().get_or_alloc(
                        std::mem::transmute((#(#ess_arg_names),*)),
                        || #inner_fn_name(#(std::mem::transmute(#ess_arg_names)),*, arb_ref(db))
                    )
                }
            }
        }
    };

    output.into()
}

#[derive(Default)]
struct NoteAttr {
    return_ref: bool,
}

impl syn::parse::Parse for NoteAttr {
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
