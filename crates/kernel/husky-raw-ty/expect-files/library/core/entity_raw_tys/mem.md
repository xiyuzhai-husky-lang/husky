[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`core::mem::Ref`, `Extern`),
            ),
        ),
        Ok(
            RawTerm(`covariant core::basic::Lifetime -> covariant Type -> Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`core::mem::RefMut`, `Extern`),
            ),
        ),
        Ok(
            RawTerm(`covariant core::basic::Lifetime -> invariant Type -> Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`core::mem::Leash`, `Extern`),
            ),
        ),
        Ok(
            RawTerm(`covariant Type -> Type`),
        ),
    ),
]