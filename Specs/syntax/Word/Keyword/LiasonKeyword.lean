
inductive LiasonKeyword
  | Haha
  deriving BEq

namespace LiasonKeyword
def toRustCode : LiasonKeyword -> String := sorry
instance : ToString LiasonKeyword where
  toString := sorry
end LiasonKeyword