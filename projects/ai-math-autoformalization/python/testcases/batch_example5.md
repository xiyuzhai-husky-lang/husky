Problem:
For $x \in \mathbb{R}$, $x > 0$, prove that $x + 1 / x \ge 2$.

LaTeX Proof:
Let $x\in\mathbb{R}$. Assume $x>0$. Then $x + \frac{1}{x} - 2 = \frac{x^2 + 1 - 2x}{x}$. Then $\frac{x^2 + 1 - 2x}{x} = \frac{{(x-1)}^2}{x}$. Then $x + \frac{1}{x} - 2 = \frac{{(x-1)}^2}{x}$. Then $\frac{{(x-1)}^2}{x} \ge 0$. Then $x + \frac{1}{x} - 2 \ge 0$. Then $x + \frac{1}{x} \ge 2$.

Lean Proof:
``` Lean
import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Data.Real.Basic
import Mathlib.Data.Real.Sqrt
import Mathlib.Tactic.Explode

namespace Example
-- Let $x\in\mathbb{R}$
variable (x : ℝ)

-- Assume $x>0$
variable (h : x > 0)

-- Then $x + \frac{1}{x} - 2 = \frac{x^2 + 1 - 2x}{x}$
def h1 : x + 1 / x - 2 = (x ^ 2 + 1 - 2 * x) / x := sorry

-- Then $\frac{x^2 + 1 - 2x}{x} = \frac{{(x-1)}^2}{x}$
def h2 : (x ^ 2 + 1 - 2 * x) / x = (x - 1) ^ 2 / x := sorry

-- Then $x + \frac{1}{x} - 2 = \frac{{(x-1)}^2}{x}$
def h3 : x + 1 / x - 2 = (x - 1) ^ 2 / x := sorry

-- Then $\frac{{(x-1)}^2}{x} \ge 0$
def h4 : (x - 1) ^ 2 / x ≥ 0 := sorry

-- Then $x + \frac{1}{x} - 2 \ge 0$
def h5 : x + 1 / x - 2 ≥ 0 := sorry

-- Then $x + \frac{1}{x} \ge 2$
def h6 : x + 1 / x ≥ 2 := sorry
end Example
```