inductive ImplicitConversion
  | None
  | WrapInSome (number_of_somes : Nat)
  | ConvertToBool
  deriving DecidableEq