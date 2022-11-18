use proc_macro2::Literal;
use syn::Token;

// Source:
//
// #[timed_salsa::db(Jar0, Jar1, Jar2)]
// pub struct Database {
//    storage: timed_salsa::Storage<Self>,
// }

pub(crate) fn db(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(args as Args);
    let input = syn::parse_macro_input!(input as syn::ItemStruct);
    let storage = match find_storage_field(&input) {
        Ok(v) => v,
        Err(err) => {
            let err = Literal::string(err);
            let error = quote_spanned!(input.ident.span() => compile_error!(#err));
            return quote! {
                #input
                #error
            }
            .into();
        }
    };

    let as_salsa_database_impl = as_salsa_database_impl(&input);
    let has_jars_impl = has_jars_impl(&args, &input, &storage);
    let has_jars_dyn_impl = has_jars_dyn_impl(&input, &storage);
    let per_jar_impls = per_jar_impls(&args, &input, &storage);

    quote! {
        #input
        #as_salsa_database_impl
        #has_jars_impl
        #has_jars_dyn_impl
        #(#per_jar_impls)*
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

fn find_storage_field(input: &syn::ItemStruct) -> Result<syn::Ident, &'static str> {
    let storage = "storage";
    for field in input.fields.iter() {
        if let Some(i) = &field.ident {
            if i == storage {
                return Ok(i.clone());
            }
        } else {
            return Err(
                "database struct must be a braced struct (`{}`) with a field named storage",
            );
        }
    }

    Err("database has no field named `storage`")
}

fn as_salsa_database_impl(input: &syn::ItemStruct) -> syn::ItemImpl {
    let db = &input.ident;
    parse_quote! {
        impl timed_salsa::database::AsSalsaDatabase for #db {
            fn as_salsa_database(&self) -> &dyn timed_salsa::Database {
                self
            }
        }
    }
}

fn has_jars_impl(args: &Args, input: &syn::ItemStruct, storage: &syn::Ident) -> syn::ItemImpl {
    let jar_paths: Vec<&syn::Path> = args.jar_paths.iter().collect();
    let db = &input.ident;
    parse_quote! {
        // ANCHOR: HasJars
        impl timed_salsa::storage::HasJars for #db {
            type Jars = (#(#jar_paths,)*);
            // ANCHOR_END: HasJars

            fn jars(&self) -> (&Self::Jars, &timed_salsa::Runtime) {
                self.#storage.jars()
            }

            fn jars_mut(&mut self) -> (&mut Self::Jars, &mut timed_salsa::Runtime) {
                self.#storage.jars_mut()
            }

            // ANCHOR: create_jars
            fn create_jars(routes: &mut timed_salsa::routes::Routes<Self>) -> Self::Jars {
                (
                    #(
                        <#jar_paths as timed_salsa::jar::Jar>::create_jar(routes),
                    )*
                )
            }
            // ANCHOR_END: create_jars
        }
    }
}

fn has_jars_dyn_impl(input: &syn::ItemStruct, storage: &syn::Ident) -> syn::ItemImpl {
    let db = &input.ident;
    parse_quote! {
        impl timed_salsa::storage::HasJarsDyn for #db {
            fn runtime(&self) -> &timed_salsa::Runtime {
                self.#storage.runtime()
            }

            fn runtime_mut(&mut self) ->&mut timed_salsa::Runtime {
                self.#storage.runtime_mut()
            }

            fn maybe_changed_after(
                &self,
                input: timed_salsa::key::DependencyIndex,
                revision: timed_salsa::Revision,
            ) -> bool {
                let ingredient = self.#storage.ingredient(input.ingredient_index());
                ingredient.maybe_changed_after(self, input, revision)
            }

            fn cycle_recovery_strategy(
                &self,
                ingredient_index: timed_salsa::IngredientIndex,
            ) -> timed_salsa::cycle::CycleRecoveryStrategy {
                let ingredient = self.#storage.ingredient(ingredient_index);
                ingredient.cycle_recovery_strategy()
            }

            fn origin(
                &self,
                index: timed_salsa::DatabaseKeyIndex,
            ) -> Option<timed_salsa::runtime::local_state::QueryOrigin> {
                let ingredient = self.#storage.ingredient(index.ingredient_index());
                ingredient.origin(index.key_index())
            }

            fn mark_validated_output(&self, executor: timed_salsa::DatabaseKeyIndex, output: timed_salsa::key::DependencyIndex) {
                let ingredient = self.#storage.ingredient(output.ingredient_index());
                ingredient.mark_validated_output(self, executor, output.key_index());
            }

            fn remove_stale_output(&self, executor: timed_salsa::DatabaseKeyIndex, stale_output: timed_salsa::key::DependencyIndex) {
                let ingredient = self.#storage.ingredient(stale_output.ingredient_index());
                ingredient.remove_stale_output(self, executor, stale_output.key_index());
            }

            fn salsa_struct_deleted(&self, ingredient: timed_salsa::IngredientIndex, id: timed_salsa::Id) {
                let ingredient = self.#storage.ingredient(ingredient);
                ingredient.salsa_struct_deleted(self, id);
            }

            fn fmt_index(&self, index: timed_salsa::key::DependencyIndex, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let ingredient = self.#storage.ingredient(index.ingredient_index());
                ingredient.fmt_index(index.key_index(), fmt)
            }
        }
    }
}

fn per_jar_impls(args: &Args, input: &syn::ItemStruct, storage: &syn::Ident) -> Vec<syn::ItemImpl> {
    let db = &input.ident;
    args.jar_paths
        .iter()
        .zip(0..)
        .flat_map(|(jar_path, jar_index)| {
            let jar_index = Literal::u32_unsuffixed(jar_index);
            vec![
                parse_quote! {
                    impl timed_salsa::storage::DbWithJar<#jar_path> for #db {
                        fn as_jar_db<'db>(&'db self) -> &'db <#jar_path as timed_salsa::jar::Jar<'db>>::DynDb
                        where
                            'db: 'db,
                        {
                            self as &'db <#jar_path as timed_salsa::jar::Jar<'db>>::DynDb
                        }
                    }
                },

                parse_quote! {
                    impl timed_salsa::storage::HasJar<#jar_path> for #db {
                        fn jar(&self) -> (&#jar_path, &timed_salsa::Runtime) {
                            let (__jars, __runtime) = self.#storage.jars();
                            (&__jars.#jar_index, __runtime)
                        }

                        fn jar_mut(&mut self) -> (&mut #jar_path, &mut timed_salsa::Runtime) {
                            let (__jars, __runtime) = self.#storage.jars_mut();
                            (&mut __jars.#jar_index, __runtime)
                        }
                    }
                },

                parse_quote! {
                    impl timed_salsa::storage::JarFromJars<#jar_path> for #db {
                        fn jar_from_jars<'db>(jars: &Self::Jars) -> &#jar_path {
                            &jars.#jar_index
                        }

                        fn jar_from_jars_mut<'db>(jars: &mut Self::Jars) -> &mut #jar_path {
                            &mut jars.#jar_index
                        }
                    }
                }
            ]
        })
        .collect()
}
