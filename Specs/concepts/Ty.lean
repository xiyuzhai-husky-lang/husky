inductive PrimitiveTy

structure f32 where
  raw : Float
 

structure i32

structure i64

structure b32

structure b64 where
  raw : UInt64

structure OrderedFloat (α : Type) where
  data : α
  valid : not nan