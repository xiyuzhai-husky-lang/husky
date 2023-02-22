[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`core::logic::Prop`, `Alien`),
            ),
        ),
        Ok(
            Term(`Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`core::logic::LogicAnd`, `Structure`),
            ),
        ),
        Ok(
            Term(`independent Type -> independent Type -> Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`core::logic::LogicOr`, `Inductive`),
            ),
        ),
        Ok(
            Term(`independent Type -> independent Type -> Type`),
        ),
    ),
]