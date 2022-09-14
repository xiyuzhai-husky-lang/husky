inductive PrefixOpr
  | Minus   -- -
  | Not     -- !$0
  | BitNot  -- ~
  | Shared  -- &
  | Move    -- !$0 after WithType or Vec or Array
  deriving DecidableEq


namespace PrefixOpr
def kindName : PrefixOpr -> String
  | Minus => "Minus"
  | Not => "Not"
  | BitNot => "BitNot" 
  | Shared => "Shared"
  | Move => "Move"
end PrefixOpr