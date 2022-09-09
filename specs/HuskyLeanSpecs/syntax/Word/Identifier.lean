inductive RootPrimitiveTyIdentifier
  | I32
  | I64
  | F32
  | F64

inductive RootContainerTyIdentifier
  | Vec
  | Array

inductive RootHigherTyIdentifier
  | TypeType
  | TraitType

namespace CustomIdentifier
def is_valid : String -> Bool := sorry
end CustomIdentifier

structure CustomIdentifier where
  value : String
  hvalid : CustomIdentifier.is_valid value

inductive Identifier
  | RootPrimitiveTy : RootPrimitiveTyIdentifier -> Identifier
  | RootContainerTy : RootContainerTyIdentifier -> Identifier
  | RootHigherTy : RootHigherTyIdentifier -> Identifier
  | Custom : CustomIdentifier -> Identifier