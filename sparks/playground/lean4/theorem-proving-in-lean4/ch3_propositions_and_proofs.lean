#check And
#check Or
#check Not

variable (p q: Prop)

example (hp: p) (hq: q) : p ∧ q := And.intro hp hq

#check fun (hp : p) (hq : q) => And.intro hp hq

example (hpq : p ∧ q) : p := And.left hpq

example (hpq : p ∧ q) : q := And.right hpq

example (hpq : p ∧ q) : q ∧ p :=
  And.intro (And.right hpq) (And.left hpq)