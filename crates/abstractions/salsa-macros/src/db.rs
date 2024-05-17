use syn::Token;

pub(crate) fn db(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(args as Args);
    let input = syn::parse_macro_input!(input as syn::ItemStruct);
    let ident = &input.ident;
    let vis = &input.vis;
    check_jar_paths(args.jar_paths.iter());
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

fn check_jar_paths<'a>(jar_paths: impl Iterator<Item = &'a syn::Path>) {
    let jar_idents: Vec<String> = jar_paths
        .map(|jar_path| jar_path.segments.last().unwrap().ident.to_string())
        .collect();
    for jar_ident in &jar_idents {
        for &dep in jar_dependencies(jar_ident) {
            assert!(
                jar_idents.contains(&dep.to_string()),
                "expect `{dep}` to be included as a dependency for `{jar_ident}`"
            )
        }
    }

    fn jar_dependencies(jar_ident: &str) -> &[&str] {
        // todo: update this list
        match jar_ident {
            "Jar" => &[],
            // comptime
            "RustTranspilationJar" => &[],
            // devtime
            "TraceJar" => &["TextJar"],
            // fs
            "CorgiConfigJar" => &[],
            "ManifestJar" => &[],
            "ToolchainConfigJar" => &[],
            "VfsJar" => &[],
            // hir
            "HirDeclJar" => &[],
            "HirDefnJar" => &[],
            "HirEagerExprJar" => &["TextJar"],
            "HirExprJar" => &[],
            "HirLazyExprJar" => &[],
            "HirPreludeJar" => &[],
            "HirTypeJar" => &[],
            // ide
            "CompletionJar" => &[],
            "DiagnosticsJar" => &[],
            "DocumentationJar" => &[],
            "FoldingRangeJar" => &[],
            "HoverJar" => &[],
            "SemanticTokenJar" => &[],
            "IdeFmtJar" => &[],
            "TokenInfoJar" => &[],
            // kernel
            "CowordJar" => &[],
            "DecSignatureJar" => &[],
            "DecTermJar" => &[],
            "DeclarativeTypeJar" => &[],
            "EntityPathJar" => &[],
            "EthSignatureJar" => &[],
            "EthTermJar" => &["DecTermJar"],
            "FlyTermJar" => &["EthTermJar"],
            "TermPreludeJar" => &[],
            "PlaceJar" => &[],
            // lex
            "TextJar" => &[],
            "TokenDataJar" => &[],
            "TokenJar" => &["TokenDataJar"],
            "TomlTokenJar" => &[],
            // linkage
            "JavelinJar" => &[],
            "LinkageJar" => &[],
            // semantics
            "SemExprJar" => &["TextJar"],
            "SemPlaceContractJar" => &[],
            // super
            "SuperNodeJar" => &[],
            // syntax
            "AstJar" => &["TokenJar"],
            "EntityTreeJar" => &["AstJar"],
            "ManifestAstJar" => &[],
            "SynDeclJar" => &[],
            "SynDefnJar" => &[],
            "SynExprJar" => &[],
            "TomlAstJar" => &[],
            "CorgiConfigAstJar" => &[],
            // val
            "KiJar" => &[],
            "KiReprJar" => &["KiJar"],
            // vm
            "VmirJar" => &[],
            other => panic!("unknown jar ident `{other}`"),
        }
    }
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
