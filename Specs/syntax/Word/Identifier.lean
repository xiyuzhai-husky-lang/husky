import Specs.syntax.Text

inductive RootPrimitiveTyIdentifier
  | I32
  | I64
  | F32
  | F64
  deriving DecidableEq

namespace RootPrimitiveTyIdentifier

  def toRustVersion : RootPrimitiveTyIdentifier -> String
    | I32 => "RootPrimitiveTyIdentifier::I32"
    | I64 => "RootPrimitiveTyIdentifier::I64"
    | F32 => "RootPrimitiveTyIdentifier::F32"
    | F64 => "RootPrimitiveTyIdentifier::F64"
end RootPrimitiveTyIdentifier

inductive RootContainerTyIdentifier
  | Vec
  | Array
  deriving DecidableEq

namespace RootContainerTyIdentifier
def toRustVersion : RootContainerTyIdentifier -> String
  | Vec => "RootContainerTyIdentifier::Vec"
  | Array => "RootContainerTyIdentifier::Array"
end RootContainerTyIdentifier

--

inductive RootHigherTyIdentifier
  | TypeType
  | TraitType
  deriving DecidableEq

namespace RootHigherTyIdentifier
def toRustVersion : RootHigherTyIdentifier -> String
  | TypeType => "RootHigherTyIdentifier::TypeType"
  | TraitType=> "RootContainerTyIdentifier::TraitType"
end RootHigherTyIdentifier

namespace CustomIdentifier
def is_valid : String -> Bool := sorry
end CustomIdentifier

structure CustomIdentifier where
  value : String
  hvalid : CustomIdentifier.is_valid value
  deriving DecidableEq

namespace CustomIdentifier
  def toRustVersion : CustomIdentifier -> String := sorry
end CustomIdentifier

structure RangedCustomIdentifier where
  range : TextRange
  ident: CustomIdentifier
  deriving DecidableEq

inductive Identifier
  | RootPrimitiveTy : RootPrimitiveTyIdentifier -> Identifier
  | RootContainerTy : RootContainerTyIdentifier -> Identifier
  | RootHigherTy : RootHigherTyIdentifier -> Identifier
  | Custom : CustomIdentifier -> Identifier
  deriving DecidableEq

namespace Identifier
  def toRustVersion : Identifier -> String
    | RootPrimitiveTy ident => s!"Identifier::RootPrimitiveTy({ident.toRustVersion})"
    | RootContainerTy ident => s!"Identifier::ContainerTy({ident.toRustVersion})"
    | RootHigherTy ident => s!"Identifier::RootHigherTy({ident.toRustVersion})"
    | Custom ident => s!"Identifier::Custom({ident.toRustVersion})"
end Identifier