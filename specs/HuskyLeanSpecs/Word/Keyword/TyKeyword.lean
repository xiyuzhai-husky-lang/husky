inductive TyKeyword
  | Struct
  | Enum
  | Record


namespace TyKeyword
def TyKeywordEnumeration := [
  Struct,
  Enum,
  Record
]
deriving instance BEq for TyKeyword
def as_str (kw : TyKeyword) : String :=
  match kw with
  | Struct => "struct"
  | Enum => "enum"
  | Record => "record"
end TyKeyword