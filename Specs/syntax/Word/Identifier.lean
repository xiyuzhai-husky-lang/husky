import Specs.syntax.Text

inductive PrimitiveTyIdentifier
  | Bool
  | I32
  | I64
  | B32
  | B64
  | F32
  | F64
  deriving DecidableEq

namespace PrimitiveTyIdentifier

  def toRustVersion : PrimitiveTyIdentifier -> String
    | Bool => "PrimitiveTyIdentifier::Bool"
    | I32 => "PrimitiveTyIdentifier::I32"
    | I64 => "PrimitiveTyIdentifier::I64"
    | B32 => "PrimitiveTyIdentifier::B32"
    | B64 => "PrimitiveTyIdentifier::B64"
    | F32 => "PrimitiveTyIdentifier::F32"
    | F64 => "PrimitiveTyIdentifier::F64"
  
  def huskyCode : PrimitiveTyIdentifier -> String
    | Bool => "bool"
    | I32 => "i32"
    | I64 => "i64"
    | B32 => "b32"
    | B64 => "b64"
    | F32 => "f32"
    | F64 => "f64"
end PrimitiveTyIdentifier

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
  | PrimitiveTy : PrimitiveTyIdentifier -> Identifier
  | RootContainerTy : RootContainerTyIdentifier -> Identifier
  | RootHigherTy : RootHigherTyIdentifier -> Identifier
  | Custom : CustomIdentifier -> Identifier
  deriving DecidableEq

namespace Identifier
  def toRustVersion : Identifier -> String
    | PrimitiveTy ident => s!"Identifier::PrimitiveTy({ident.toRustVersion})"
    | RootContainerTy ident => s!"Identifier::ContainerTy({ident.toRustVersion})"
    | RootHigherTy ident => s!"Identifier::RootHigherTy({ident.toRustVersion})"
    | Custom ident => s!"Identifier::Custom({ident.toRustVersion})"
  
  def huskyCode : Identifier -> String
    | PrimitiveTy ident => ident.huskyCode
    | RootContainerTy ident => ident.huskyCode
    | RootHigherTy ident => ident.huskyCode
    | Custom ident => ident.huskyCode
end Identifier