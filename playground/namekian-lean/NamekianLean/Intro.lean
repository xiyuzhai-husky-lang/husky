inductive LogicIntroKind
| Arb | Exists | NotExists
  deriving Repr

structure LogicIntro where
  varid: Int
  kind: LogicIntroKind
  typename: String
  deriving Repr
