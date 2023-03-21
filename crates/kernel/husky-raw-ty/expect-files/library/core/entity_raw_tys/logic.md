[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`core::logic::Prop`, `Extern`),
            ),
        ),
        Ok(
            RawTerm(`Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`core::logic::LogicAnd`, `Structure`),
            ),
        ),
        Ok(
            RawTerm(`independent Type -> independent Type -> Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`core::logic::LogicOr`, `Inductive`),
            ),
        ),
        Ok(
            RawTerm(`independent Type -> independent Type -> Type`),
        ),
    ),
]