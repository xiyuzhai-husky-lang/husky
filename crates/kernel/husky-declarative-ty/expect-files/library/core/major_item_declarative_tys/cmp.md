[
    (
        ItemPath::MajorItem(
            MajorItemPath::Trait(
                TraitPath(`core::cmp::PartialEq`),
            ),
        ),
        Ok(
            DeclarativeTerm(`invariant Type -> core::basic::Trait`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Trait(
                TraitPath(`core::cmp::Eq`),
            ),
        ),
        Ok(
            DeclarativeTerm(`invariant Type -> core::basic::Trait`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Trait(
                TraitPath(`core::cmp::PartialOrd`),
            ),
        ),
        Ok(
            DeclarativeTerm(`invariant Type -> core::basic::Trait`),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Trait(
                TraitPath(`core::cmp::Ord`),
            ),
        ),
        Ok(
            DeclarativeTerm(`invariant Type -> core::basic::Trait`),
        ),
    ),
]