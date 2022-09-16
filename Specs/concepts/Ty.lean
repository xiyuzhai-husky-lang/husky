inductive PrimitiveTy

structure f32 where
  raw : Float
 
structure f64 where
  raw : Float

structure i32
  deriving DecidableEq

structure i64
  deriving DecidableEq

structure b32
  deriving DecidableEq

structure b64 where
  raw : UInt64
  deriving DecidableEq

structure OrderedFloat (α : Type) where
  -- data : α
  -- valid : not nan
  deriving DecidableEq

structure OrderedF32
  deriving DecidableEq
structure OrderedF64
  deriving DecidableEq