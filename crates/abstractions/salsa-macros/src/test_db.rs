use proc_macro2::Literal;
use syn::Token;

pub(crate) fn test_db(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(args as Args);
    let input = syn::parse_macro_input!(input as syn::ItemStruct);
    let ident = &input.ident;
    let vis = &input.vis;

    quote! {
        #[cfg(test)]
        #vis struct #ident(salsa::test_utils::TestDb);

        #[cfg(test)]
        impl std::ops::Deref for #ident {
            type Target = salsa::test_utils::TestDb;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[cfg(test)]
        impl std::ops::DerefMut for #ident {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        #[cfg(test)]
        impl Default for #ident {
            fn default() -> Self {
                todo!()
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
