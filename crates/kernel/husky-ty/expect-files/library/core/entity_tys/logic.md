[
    (
        TypePath(`core::logic::Prop`, `Alien`),
        Ok(
            Term(`Type`),
        ),
    ),
    (
        TypePath(`core::logic::LogicAnd`, `Structure`),
        Ok(
            Term(`TermCurry { variance: Independent, x: Term(`Type`), y: Term(`TermCurry { variance: Independent, x: Term(`Type`), y: Term(`Type`) }`) }`),
        ),
    ),
    (
        TypePath(`core::logic::LogicOr`, `Inductive`),
        Ok(
            Term(`TermCurry { variance: Independent, x: Term(`Type`), y: Term(`TermCurry { variance: Independent, x: Term(`Type`), y: Term(`Type`) }`) }`),
        ),
    ),
]