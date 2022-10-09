import Specs.concepts

inductive RawLiteralData 
   | Void
   | Integer(value : i64)
   | I32(value : i32)
   | I64(value : i64)
   | Float(value : OrderedF64)
   | F32(value : OrderedF32)
   | F64(value : OrderedF64)
   | Bits(value : b64)
   | B32(value : b32)
   | B64(value : b64)
   | Bool(value : Bool)


-- def RawLiteralData.decEq : DecidableEq RawLiteralData :=
deriving instance DecidableEq for RawLiteralData

def RawLiteralData.decEq2 : DecidableEq RawLiteralData := fun (a b) => decEq a b

namespace RawLiteralData
  def huskyCode : RawLiteralData -> String := sorry
end RawLiteralData