[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`core::mem::Ref`, `Extern`),
            ),
        ),
        Ok(
            DeclarativeTerm(`covariant core::basic::Lifetime -> covariant Type -> Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`core::mem::RefMut`, `Extern`),
            ),
        ),
        Ok(
            DeclarativeTerm(`covariant core::basic::Lifetime -> invariant Type -> Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`core::mem::Leash`, `Extern`),
            ),
        ),
        Ok(
            DeclarativeTerm(`covariant Type -> Type`),
        ),
    ),
]