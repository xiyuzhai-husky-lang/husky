def hello := "world"

structure BaseRepr where
  base: Nat
  digits: List Nat

namespace BaseRepr
def new (base: Nat) (n: Nat): BaseRepr :=
  if n < base then
    { base, digits := [n] : BaseRepr }
  else
    let a := n / base
    let b := n % base
    let a_base_repr := (new base a)
    { base, digits := sorry }

end BaseRepr
