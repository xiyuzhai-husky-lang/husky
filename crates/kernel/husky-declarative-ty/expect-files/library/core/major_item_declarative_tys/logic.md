[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::logic::Prop`, `Extern`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Type`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::logic::LogicAnd`, `Structure`),
            ),
        ),
        Ok(
            DeclarativeTerm(`independent Type -> independent Type -> Type`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::logic::LogicOr`, `Inductive`),
            ),
        ),
        Ok(
            DeclarativeTerm(`independent Type -> independent Type -> Type`),
        ),
    ),
]