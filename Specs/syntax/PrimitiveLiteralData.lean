inductive PrimitiveLiteralData 
   | Void
   | Integer(value : i64)
   | I32(value : i32)
   | I64(value : i64)
   | Float(value : OrderedFloat f64)
   | F32(value : OrderedFloat f32)
   | F64(value : OrderedFloat f64)
   | Bits(value : u64)
   | B32(value : u32)
   | B64(value : u64)
   | Bool(value : Bool)
