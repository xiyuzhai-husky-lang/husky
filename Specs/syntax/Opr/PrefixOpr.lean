inductive PrefixOpr
  | Minus   -- -
  | Not     -- !$0
  | BitNot  -- ~
  | Shared  -- &
  | Move    -- !$0 after WithType or Vec or Array
  deriving DecidableEq