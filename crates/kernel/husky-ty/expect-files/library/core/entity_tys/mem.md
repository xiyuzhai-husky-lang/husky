[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`core::mem::Ref`, `Alien`),
            ),
        ),
        Ok(
            Term(`covariant Lifetime -> covariant Type -> Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`core::mem::RefMut`, `Alien`),
            ),
        ),
        Ok(
            Term(`covariant Lifetime -> invariant Type -> Type`),
        ),
    ),
]