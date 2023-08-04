[
    (
        ItemPath::MajorItem(
            MajarItemPath::Type(
                TypePath(`core::logic::Prop`, `Extern`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Type`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Type(
                TypePath(`core::logic::LogicAnd`, `Structure`),
            ),
        ),
        Ok(
            DeclarativeTerm(`independent Type -> independent Type -> Type`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Type(
                TypePath(`core::logic::LogicOr`, `Inductive`),
            ),
        ),
        Ok(
            DeclarativeTerm(`independent Type -> independent Type -> Type`),
        ),
    ),
]