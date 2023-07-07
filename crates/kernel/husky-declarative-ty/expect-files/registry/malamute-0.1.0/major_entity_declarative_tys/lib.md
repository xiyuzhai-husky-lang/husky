[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`malamute::OneVsAll`, `Enum`),
            ),
        ),
        Ok(
            DeclarativeTerm(`(independent v0: Type) -> independent v0 -> Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`malamute::OneVsAllResult`, `Enum`),
            ),
        ),
        Ok(
            DeclarativeTerm(`(independent v0: Type) -> independent v0 -> Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`malamute::narrow_down`, `Gn`),
            ),
        ),
        Ok(
            DeclarativeTerm(`(independent v0: Type) -> (independent v0: v0) -> fn(core::num::f32, core::num::i32) -> malamute::OneVsAllResult v0 v0`),
        ),
    ),
]