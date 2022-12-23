import Specs.concepts

inductive LiteralToken 
   | Void
  --  | Integer(value : i64)
  --  | I32(value : i32)
  --  | I64(value : i64)
  --  | Float(value : OrderedF64)
  --  | F32(value : OrderedF32)
  --  | F64(value : OrderedF64)
  --  | Bits(value : b64)
  --  | B32(value : b32)
  --  | B64(value : b64)
   | Bool(value : Bool)


-- def LiteralToken.decEq : DecidableEq LiteralToken :=
deriving instance DecidableEq for LiteralToken

def LiteralToken.decEq2 : DecidableEq LiteralToken := fun (a b) => decEq a b

namespace LiteralToken
  def huskyCode : LiteralToken -> String := sorry
end LiteralToken