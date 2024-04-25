def hello := "world"

inductive Literal where
| Nat (nat: Nat)
| Fin (nat: Nat) (fin: Fin nat)

inductive Pattern where
| Or (choices: List Pattern)
| Product (fields: List Pattern)
| Literal (literal: Literal)

inductive Animal
| Dog
| Cat
| Bird

structure A where
  a1 : Animal
  a2 : Animal
  a3 : Animal
  a4 : Animal
  a5 : Animal
  a6 : Animal
  a7 : Animal
  a8 : Animal
  a9 : Animal
  a10 : Animal

namespace Animal
def f: A -> Bool
  | A (Dog | Cat) (Dog | Cat) (Dog | Cat) (Dog | Cat) (Dog | Cat) (Dog | Cat) => sorry
  | _ => sorry
end namespace
