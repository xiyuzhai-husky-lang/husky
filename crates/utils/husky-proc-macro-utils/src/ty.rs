use syn::{GenericArgument, Lifetime, PathArguments, Type};

pub fn make_all_lifetimes_static(ty: &Type) -> Type {
    match ty {
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
        Type::Array(type_array) => unimplemented!(
            "make_all_lifetimes_static: Array type not implemented: {:?}",
            type_array
        ),
        Type::BareFn(type_bare_fn) => unimplemented!(
            "make_all_lifetimes_static: BareFn type not implemented: {:?}",
            type_bare_fn
        ),
        Type::Group(type_group) => unimplemented!(
            "make_all_lifetimes_static: Group type not implemented: {:?}",
            type_group
        ),
        Type::ImplTrait(type_impl_trait) => unimplemented!(
            "make_all_lifetimes_static: ImplTrait type not implemented: {:?}",
            type_impl_trait
        ),
        Type::Infer(type_infer) => unimplemented!(
            "make_all_lifetimes_static: Infer type not implemented: {:?}",
            type_infer
        ),
        Type::Macro(type_macro) => unimplemented!(
            "make_all_lifetimes_static: Macro type not implemented: {:?}",
            type_macro
        ),
        Type::Never(type_never) => unimplemented!(
            "make_all_lifetimes_static: Never type not implemented: {:?}",
            type_never
        ),
        Type::Paren(type_paren) => unimplemented!(
            "make_all_lifetimes_static: Paren type not implemented: {:?}",
            type_paren
        ),
        Type::Ptr(type_ptr) => unimplemented!(
            "make_all_lifetimes_static: Ptr type not implemented: {:?}",
            type_ptr
        ),
        Type::Reference(type_reference) => unimplemented!(
            "make_all_lifetimes_static: Reference type not implemented: {:?}",
            type_reference
        ),
        Type::Slice(type_slice) => unimplemented!(
            "make_all_lifetimes_static: Slice type not implemented: {:?}",
            type_slice
        ),
        Type::TraitObject(type_trait_object) => unimplemented!(
            "make_all_lifetimes_static: TraitObject type not implemented: {:?}",
            type_trait_object
        ),
        Type::Tuple(type_tuple) => unimplemented!(
            "make_all_lifetimes_static: Tuple type not implemented: {:?}",
            type_tuple
        ),
        Type::Verbatim(token_stream) => unimplemented!(
            "make_all_lifetimes_static: Verbatim type not implemented: {:?}",
            token_stream
        ),
        _ => todo!(),
    }
}
