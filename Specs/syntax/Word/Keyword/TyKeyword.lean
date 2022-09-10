inductive TyKeyword
  | Struct
  | Enum
  | Record
  deriving BEq

namespace TyKeyword
def toRustCode : TyKeyword -> String := sorry

instance : ToString TyKeyword where
  toString : TyKeyword -> String
  | Struct => "struct"
  | Enum => "enum"
  | Record => "record"
end TyKeyword

-- namespace TyKeyword
-- def TyKeywordEnumeration := [
--   Struct,
--   Enum,
--   Record
-- ]
-- def as_str (kw : TyKeyword) : String :=
--   match kw with
--   | Struct => "struct"
--   | Enum => "enum"
--   | Record => "record"
-- end TyKeyword