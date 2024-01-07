use syn::Token;

pub(crate) fn db(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(args as Args);
    let input = syn::parse_macro_input!(input as syn::ItemStruct);
    let ident = &input.ident;
    let vis = &input.vis;
    let initialization: proc_macro2::TokenStream = args
        .jar_paths
        .iter()
        .map(|jar_path| {
            quote! {
                jars.initialize_jar::<#jar_path>(routes);
            }
        })
        .collect();

    quote! {
        #vis struct #ident(::salsa::Db);

        impl std::ops::Deref for #ident {
            type Target = ::salsa::Db;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for #ident {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl Default for #ident {
            fn default() -> Self {
                Self(::salsa::Db::new(|jars, routes| {
                    *jars = ::salsa::jar::Jars::default();
                    #initialization
                }))
            }
        }

        impl ::salsa::snapshot::SnapshotClone for #ident {
            fn snapshot_clone(&self) -> Self {
                Self(self.0.snapshot_clone())
            }
        }
    }
    .into()
}

pub struct Args {
    jar_paths: syn::punctuated::Punctuated<syn::Path, Token![,]>,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            jar_paths: syn::punctuated::Punctuated::parse_terminated(input)?,
        })
    }
}
