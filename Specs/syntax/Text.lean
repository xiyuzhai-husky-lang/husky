structure Row where
  raw : Nat
  deriving DecidableEq

structure Column where
  raw : Nat
  deriving DecidableEq

structure TextPosition where
  row : Row
  col : Column
  deriving DecidableEq

structure TextRange where
  start : TextPosition
  end_ : TextPosition
  deriving DecidableEq