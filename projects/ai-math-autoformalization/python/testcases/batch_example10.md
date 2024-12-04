Problem:
For $a, b \in \mathbb{R}$, $a, b > 0$, prove that $a / b + b / a \ge 2$.

LaTeX Proof:
Let $a\in\mathbb{R}$. Assume $a > 0$. Let $b\in\mathbb{R}$. Assume $b > 0$. Then $\frac{a}{b} + \frac{b}{a} - 2 = \frac{a^2 + b^2 - 2ab}{ab}$. Then $\frac{a^2 + b^2 - 2ab}{ab} = \frac{{(a-b)}^2}{ab}$. Then $\frac{{(a-b)}^2}{ab} \ge 0$. Then $\frac{a}{b} + \frac{b}{a} - 2 \ge 0$. Then $\frac{a}{b} + \frac{b}{a} \ge 2$.

Lean Proof:
``` Lean
import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Data.Real.Basic
import Mathlib.Data.Real.Sqrt
import Mathlib.Tactic.Explode

namespace Example
-- Let $a\in\mathbb{R}$
variable (a : ℝ)

-- Assume $a > 0$
variable (h : a > 0)

-- Let $b\in\mathbb{R}$
variable (b : ℝ)

-- Assume $b > 0$
variable (h1 : b > 0)

-- Then $\frac{a}{b} + \frac{b}{a} - 2 = \frac{a^2 + b^2 - 2ab}{ab}$
def h2 : a / b + b / a - 2 = (a ^ 2 + b ^ 2 - 2 * a * b) / (a * b) := sorry

-- Then $\frac{a^2 + b^2 - 2ab}{ab} = \frac{{(a-b)}^2}{ab}$
def h3 : (a ^ 2 + b ^ 2 - 2 * a * b) / (a * b) = (a - b) ^ 2 / (a * b) := sorry

-- Then $\frac{{(a-b)}^2}{ab} \ge 0$
def h4 : (a - b) ^ 2 / (a * b) ≥ 0 := sorry

-- Then $\frac{a}{b} + \frac{b}{a} - 2 \ge 0$
def h5 : a / b + b / a - 2 ≥ 0 := sorry

-- Then $\frac{a}{b} + \frac{b}{a} \ge 2$
def h6 : a / b + b / a ≥ 2 := sorry
end Example
```
