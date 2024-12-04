Problem:
For any $x \in \mathbb{R}$, $x \ge 0$, prove that $x + 1 \ge 2 \sqrt x$.

LaTex Proof:
Let $x\in\mathbb{R}$. Assume $x > 0$. Then $x + 1 - 2\sqrt{x} = {(\sqrt{x}-1)}^2$. Then ${(\sqrt{x}-1)}^2 \ge 0$. Then $x + 1 - 2\sqrt{x} \ge 0$. Then $x + 1 \ge 2\sqrt{x}$.

Lean Proof:
``` Lean
import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Data.Real.Basic
import Mathlib.Data.Real.Sqrt

namespace Example

-- Let $x\in\mathbb{R}$
variable (x : ℝ)

-- Assume $x > 0$
variable (h : x > 0)

-- Then $x + 1 - 2\sqrt{x} = {(\sqrt{x}-1)}^2$
def h1 : x + 1 - 2 * Real.sqrt x = (Real.sqrt x - 1) ^ 2 :=
by
  -- Expand the right-hand side (a-b)²
  have : (√x - 1) ^ 2 = √x * √x - 2 * √x + 1 := by ring
  have h10 : x ≥ 0 := by linarith
  have h11 : √x * √x = x:= by
    calc
      √x * √x = (√x)^2 := by ring
      _ = x := by apply Real.sq_sqrt; exact h10;
  rw [this, h11]
  ring



-- Then ${(\sqrt{x}-1)}^2 \ge 0$
def h2 : (Real.sqrt x - 1) ^ 2 ≥ 0 :=
by
  nlinarith

-- Then $x + 1 - 2\sqrt{x} \ge 0$
def h3 : x + 1 - 2 * Real.sqrt x ≥ 0 :=
by
  rw [h1 x h]
  exact h2 x

-- Then $x + 1 \ge 2\sqrt{x}$
def h4 : x + 1 ≥ 2 * Real.sqrt x :=
by
  linarith [h3 x h]

end Example
```
