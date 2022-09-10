inductive TyKeyword
  | Struct
  | Enum
  | Record
  deriving BEq

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