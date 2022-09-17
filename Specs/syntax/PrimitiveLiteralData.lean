import Specs.concepts

inductive PrimitiveLiteralData 
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


-- def PrimitiveLiteralData.decEq : DecidableEq PrimitiveLiteralData :=
deriving instance DecidableEq for PrimitiveLiteralData

def PrimitiveLiteralData.decEq2 : DecidableEq PrimitiveLiteralData := fun (a b) => decEq a b

namespace PrimitiveLiteralData
  def huskyCode : PrimitiveLiteralData -> String := sorry
end PrimitiveLiteralData