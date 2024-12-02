import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Data.Real.Basic

namespace Example8

-- Let $x\in\mathbb{R}$
variable (x : ℝ)

-- Assume $x > 0$
variable (h : x > 0)

-- Then $x + 1 - 2\sqrt{x} = {(\sqrt{x}-1)}^2$
def h1 : x + 1 - 2 * Real.sqrt x = (Real.sqrt x - 1) ^ 2 :=
by
  have h_sqrtx_sq : (Real.sqrt x) ^ 2 = x := by exact Real.sqrt_mul_self h.le
  rw [←h_sqrtx_sq]
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

end Example8
