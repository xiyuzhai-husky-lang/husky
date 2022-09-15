
inductive LiasonKeyword
  | Haha
  deriving DecidableEq

namespace LiasonKeyword
def huskyCode : LiasonKeyword -> String := sorry
def toRustVersion : LiasonKeyword -> String := sorry
instance : ToString LiasonKeyword where
  toString := sorry
end LiasonKeyword