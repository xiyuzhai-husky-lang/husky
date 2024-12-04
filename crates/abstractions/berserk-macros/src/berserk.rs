use crate::*;
use convert_case::{Case, Casing};

pub(crate) fn berserk(_attr: TokenStream, item: TokenStream) -> TokenStream {
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
            if let Some(ref_ty) = field_attr.return_ref_ty {
                quote! {
                    pub fn #field_ident(self) -> &'db #ref_ty {
                        &self.0.0.value.#field_ident
                    }
                }
            } else {
                quote! {
                    pub fn #field_ident(self) -> &'db #field_ty {
                        &self.0.0.value.#field_ident
                    }
                }
            }
        } else {
            quote! {
                pub fn #field_ident(self) -> #field_ty {
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
                impl<Q: ?Sized> std::borrow::Borrow<Q> for #data_ty_ident<'db>
                where
                    #field_ty: std::borrow::Borrow<Q>,
                {
                    fn borrow(&self) -> &Q {
                        self.#field_ident.borrow()
                    }
                }

                impl<'a, Q: ?Sized> From<&'a Q> for #data_ty_ident<'db> where #field_ty: From<&'a Q> {
                    fn from(q: &'a Q) -> Self {
                        Self { #field_ident: q.into() }
                    }
                }

                impl<'db> #ty_ident<'db> {
                    #vis fn from_ref<Q: Eq + std::hash::Hash + ?Sized>(q: &Q, db: &'db ::berserk::db::BerserkerDb) -> Self
                    where
                        #field_ty: std::borrow::Borrow<Q> + for<'a> From<&'a Q>,
                    {
                        #ty_ident(db.berserk_ref::<#data_ty_ident<'db>, Q>(q))
                    }
                }
            }
        }
        _ => quote! {},
    };

    let expanded = quote! {
        #[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
        #vis struct #data_ty_ident<'db> {
            #(#field_defs),*
        }

        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #vis struct #ty_ident<'db>(::berserk::Berserk<'db, #data_ty_ident<'db>>);

        impl<'db> ::berserk::as_id::AsBerserkId<'db> for #ty_ident<'db> {
            fn as_id(self) -> u32 {
                self.0.as_id()
            }

            fn from_id(id: u32, db: &'db ::berserk::db::BerserkerDb) -> Self {
                Self(berserk::Berserk::from_id(id, db))
            }
        }

        impl<'db> #ty_ident<'db> {
            #vis fn new(#(#ctor_params),*, db: &'db ::berserk::db::BerserkerDb) -> Self {
                use berserk::Berserk;

                let data = #data_ty_ident {
                    #(#field_inits),*
                };

                #ty_ident(db.berserk(data))
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
    return_ref_ty: Option<syn::Type>,
}

impl FieldAttr {
    fn new(attrs: &[Attribute]) -> Self {
        let mut slf = Self::default();
        for attr in attrs {
            if let Some(ident) = attr.path().get_ident() {
                if ident == "return_ref" {
                    slf.return_ref = true;
                    match attr.meta {
                        Meta::Path(ref path) => (),
                        Meta::List(ref meta_list) => {
                            match syn::parse::<syn::Type>(meta_list.tokens.clone().into()) {
                                Ok(ty) => slf.return_ref_ty = Some(ty),
                                Err(_) => todo!(),
                            }
                        }
                        Meta::NameValue(ref meta_name_value) => todo!(),
                    }
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
