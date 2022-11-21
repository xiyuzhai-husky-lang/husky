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

namespace Identifier
def is_valid : String -> Bool := sorry
end Identifier

structure Identifier where
  value : String
  hvalid : Identifier.is_valid value
  deriving DecidableEq

namespace Identifier
  def toRustVersion : Identifier -> String := sorry
  def huskyCode : Identifier -> String
    | ident => ident.value
end Identifier

structure RangedIdentifier where
  range : TextRange
  ident: Identifier
  deriving DecidableEq

inductive Identifier
  | PrimitiveTy : PrimitiveTyIdentifier -> Identifier
  | RootContainerTy : RootContainerTyIdentifier -> Identifier
  | RootHigherTy : RootHigherTyIdentifier -> Identifier
  | Custom : Identifier -> Identifier
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