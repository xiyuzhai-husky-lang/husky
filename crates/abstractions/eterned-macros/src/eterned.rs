use crate::*;
use convert_case::{Case, Casing};

pub(crate) fn eterned(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let vis = input.vis;
    let ty_ident = input.ident;
    let data_ty_ident = format_ident!("__{}Data", ty_ident);
    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    // Generate the field definitions for both structs
    let field_defs = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { #name: #ty }
    });

    // Generate constructor parameters
    let ctor_params = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { #name: #ty }
    });

    // Generate field initializers
    let field_inits = fields.iter().map(|f| {
        let field_ident = &f.ident;
        quote! { #field_ident }
    });

    let field_accesses = fields.iter().map(|field| {
        let field_attr = FieldAttr::new(&field.attrs);
        let field_ident = &field.ident;
        let field_ty = &field.ty;
        if field_attr.return_ref {
            quote! {
                pub fn #field_ident(self, db: &::eterned::db::EternerDb) -> &#field_ty {
                    &self.0.0.value.#field_ident
                }
            }
        } else {
            quote! {
                pub fn #field_ident(self, db: &::eterned::db::EternerDb) -> #field_ty {
                    self.0.0.value.#field_ident
                }
            }
        }
    });

    let from_ref = match fields.len() {
        1 => {
            let field = &fields[0];
            let field_ident = &field.ident;
            let field_ty = &field.ty;
            quote! {
                impl<Q: ?Sized> std::borrow::Borrow<Q> for #data_ty_ident
                where
                    #field_ty: std::borrow::Borrow<Q>,
                {
                    fn borrow(&self) -> &Q {
                        self.#field_ident.borrow()
                    }
                }

                impl<'a, Q: ?Sized> From<&'a Q> for #data_ty_ident where #field_ty: From<&'a Q> {
                    fn from(q: &'a Q) -> Self {
                        Self { #field_ident: q.into() }
                    }
                }

                impl #ty_ident {
                    #vis fn from_ref<Q: Eq + std::hash::Hash + ?Sized>(q: &Q, db: &::eterned::db::EternerDb) -> Self
                    where
                        #field_ty: std::borrow::Borrow<Q> + for<'a> From<&'a Q>,
                    {
                        #ty_ident(db.etern_ref::<#data_ty_ident, Q>(q))
                    }
                }
            }
        }
        _ => quote! {},
    };

    let expanded = quote! {
        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        #vis struct #data_ty_ident {
            #(#field_defs),*
        }

        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #vis struct #ty_ident(eterned::Eterned<#data_ty_ident>);

        impl ::eterned::as_id::AsEternedId for #ty_ident {
            fn as_id(self) -> u32 {
                self.0.as_id()
            }

            fn from_id(id: u32, db: &::eterned::db::EternerDb) -> Self {
                Self(eterned::Eterned::from_id(id, db))
            }
        }

        impl #ty_ident {
            #vis fn new(#(#ctor_params),*, db: &::eterned::db::EternerDb) -> Self {
                use eterned::Eterned;

                let data = #data_ty_ident {
                    #(#field_inits),*
                };

                #ty_ident(db.etern(data))
            }

            #(#field_accesses)*
        }

        #from_ref
    };

    TokenStream::from(expanded)
}

#[derive(Default)]
struct FieldAttr {
    return_ref: bool,
}

impl FieldAttr {
    fn new(attrs: &[Attribute]) -> Self {
        let mut slf = Self::default();
        for attr in attrs {
            if let Some(ident) = attr.path().get_ident() {
                if ident == "return_ref" {
                    slf.return_ref = true;
                } else {
                    panic!("Invalid attribute: `{}`", ident);
                }
            } else {
                panic!("Invalid attribute: {:?}", attr.path());
            }
        }
        slf
    }
}
