
inductive TYPE_NAME
  | VARIANT1
  | VARIANT2


inductive NaturalNumber
  | Zero -- 0
  | Succ : NaturalNumber -> NaturalNumber

namespace NaturalNumber
def add : NaturalNumber -> NaturalNumber -> NaturalNumber
  | x, Zero => x
  | x, Succ a => Succ (add x a)

theorem example3 (a b : NaturalNumber) (h : Succ a = b) : Succ (Succ a) = Succ b := by
  rw [h]

  
end NaturalNumber