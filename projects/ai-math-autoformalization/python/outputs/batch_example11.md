Here is the completed Lean 4 proof:

``` Lean
import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Data.Real.Basic
import Mathlib.Data.Real.Sqrt
import Mathlib.Tactic.Explode

namespace Example

-- Let $x\in\mathbb{R}$
variable (x : ℝ)

-- Let $y\in\mathbb{R}$
variable (y : ℝ)

-- Then $x^2 + y^2 - 2xy = {(x-y)}^2$
def h : x ^ 2 + y ^ 2 - 2 * x * y = (x - y) ^ 2 :=
by
  ring

-- Then ${(x-y)}^2 \ge 0$
def h1 : (x - y) ^ 2 ≥ 0 :=
by
  apply sq_nonneg

-- Then $x^2 + y^2 - 2xy \ge 0$
def h2 : x ^ 2 + y ^ 2 - 2 * x * y ≥ 0 :=
by
  rw [h x y]
  exact h1 x y

-- Then $x^2 + y^2 \ge 2xy$
def h3 : x ^ 2 + y ^ 2 ≥ 2 * x * y :=
by
  linarith [h2 x y]

end Example
```

This proof closely follows the LaTeX proof you provided. The `ring` tactic is used to prove the first statement by expanding and simplifying the expression. The `sq_nonneg` theorem is used to prove the second statement, which states that the square of any real number is non-negative. The third statement is proved by rewriting the expression using the first statement and then applying the second statement. Finally, the fourth statement is proved by using the `linarith` tactic to combine the previous statements.