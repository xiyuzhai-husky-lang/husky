import Specs.syntax.Text

inductive RootPrimitiveTyIdentifier
  | Bool
  | I32
  | I64
  | B32
  | B64
  | F32
  | F64
  deriving DecidableEq

namespace RootPrimitiveTyIdentifier

  def toRustVersion : RootPrimitiveTyIdentifier -> String
    | Bool => "RootPrimitiveTyIdentifier::Bool"
    | I32 => "RootPrimitiveTyIdentifier::I32"
    | I64 => "RootPrimitiveTyIdentifier::I64"
    | B32 => "RootPrimitiveTyIdentifier::B32"
    | B64 => "RootPrimitiveTyIdentifier::B64"
    | F32 => "RootPrimitiveTyIdentifier::F32"
    | F64 => "RootPrimitiveTyIdentifier::F64"
  
  def huskyCode : RootPrimitiveTyIdentifier -> String
    | Bool => "bool"
    | I32 => "i32"
    | I64 => "i64"
    | B32 => "b32"
    | B64 => "b64"
    | F32 => "f32"
    | F64 => "f64"
end RootPrimitiveTyIdentifier

inductive RootContainerTyIdentifier
  | Vec
  | Array
  deriving DecidableEq

namespace RootContainerTyIdentifier
def toRustVersion : RootContainerTyIdentifier -> String
  | Vec => "RootContainerTyIdentifier::Vec"
  | Array => "RootContainerTyIdentifier::Array"
  def huskyCode : RootContainerTyIdentifier -> String
  | Vec => "Vec"
  | Array => "Array"
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

  def huskyCode : RootHigherTyIdentifier -> String
    | TypeType => "Type"
    | TraitType => "Trait"
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
  def huskyCode : CustomIdentifier -> String
    | ident => ident.value
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
  
  def huskyCode : Identifier -> String
    | RootPrimitiveTy ident => ident.huskyCode
    | RootContainerTy ident => ident.huskyCode
    | RootHigherTy ident => ident.huskyCode
    | Custom ident => ident.huskyCode
end Identifier