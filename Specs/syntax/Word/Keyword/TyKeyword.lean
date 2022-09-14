inductive TyKeyword
  | Struct
  | Enum
  | Record
  deriving DecidableEq

namespace TyKeyword
  def kindName : TyKeyword -> String
    | Struct => "Struct"
    | Enum => "Enum"
    | Record => "Record"

  def huskyCode : TyKeyword -> String
    | Struct => "struct"
    | Enum => "enum"
    | Record => "record"

  instance : ToString TyKeyword where
    toString : TyKeyword -> String
    | kw => s!"TyKeyword::{kw.kindName}"
end TyKeyword
