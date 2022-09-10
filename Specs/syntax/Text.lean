structure Row where
  raw : Nat

structure Column where
  raw : Nat

structure TextPosition where
  row : Row
  col : Column

structure TextRange where
  start : TextPosition
  end_ : TextPosition