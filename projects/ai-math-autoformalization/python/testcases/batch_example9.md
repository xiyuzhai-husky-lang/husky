Problem:
For $x, y \in \mathbb{R}$, $x, y > 0$, prove that $1 / x + 1 / y \ge 4 / (x + y)$.

LaTeX Proof:
Let $x\in\mathbb{R}$. Assume $x>0$. Let $y\in\mathbb{R}$. Assume $y>0$. Then $\frac{1}{x} + \frac{1}{y} - \frac{4}{x+y} = \frac{y(x+y) + x(x+y) - 4xy}{xy(x+y)}$. Then $\frac{y(x+y) + x(x+y) - 4xy}{xy(x+y)} = \frac{yx + x^2 + x^2 + yx - 4xy}{xy(x+y)}$. Then $\frac{yx + x^2 + x^2 + yx - 4xy}{xy(x+y)} = \frac{x^2 + x^2 -2xy}{xy(x+y)}$. Then $\frac{x^2 + x^2 -2xy}{xy(x+y)} = \frac{{(x-y)}^2}{xy(x+y)}$. Then $\frac{{(x-y)}^2}{xy(x+y)} \ge 0$. Then $\frac{1}{x} + \frac{1}{y} - \frac{4}{x+y} \ge 0$. Then $\frac{1}{x} + \frac{1}{y} \ge \frac{4}{x+y}$.

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

-- Let $y\in\mathbb{R}$
variable (y : ℝ)

-- Assume $y>0$
variable (h1 : y > 0)

-- Then $\frac{1}{x} + \frac{1}{y} - \frac{4}{x+y} = \frac{y(x+y) + x(x+y) - 4xy}{xy(x+y)}$
def h2 : 1 / x + 1 / y - 4 / (x + y) = (y * (x + y) + x * (x + y) - 4 * x * y) / (x * y * (x + y)) := sorry

-- Then $\frac{y(x+y) + x(x+y) - 4xy}{xy(x+y)} = \frac{yx + x^2 + x^2 + yx - 4xy}{xy(x+y)}$
def h3 : (y * (x + y) + x * (x + y) - 4 * x * y) / (x * y * (x + y)) = (y * x + x ^ 2 + x ^ 2 + y * x - 4 * x * y) / (x * y * (x + y)) := sorry

-- Then $\frac{yx + x^2 + x^2 + yx - 4xy}{xy(x+y)} = \frac{x^2 + x^2 -2xy}{xy(x+y)}$
def h4 : (y * x + x ^ 2 + x ^ 2 + y * x - 4 * x * y) / (x * y * (x + y)) = (x ^ 2 + x ^ 2 - 2 * x * y) / (x * y * (x + y)) := sorry

-- Then $\frac{x^2 + x^2 -2xy}{xy(x+y)} = \frac{{(x-y)}^2}{xy(x+y)}$
def h5 : (x ^ 2 + x ^ 2 - 2 * x * y) / (x * y * (x + y)) = (x - y) ^ 2 / (x * y * (x + y)) := sorry

-- Then $\frac{{(x-y)}^2}{xy(x+y)} \ge 0$
def h6 : (x - y) ^ 2 / (x * y * (x + y)) ≥ 0 := sorry

-- Then $\frac{1}{x} + \frac{1}{y} - \frac{4}{x+y} \ge 0$
def h7 : 1 / x + 1 / y - 4 / (x + y) ≥ 0 := sorry

-- Then $\frac{1}{x} + \frac{1}{y} \ge \frac{4}{x+y}$
def h8 : 1 / x + 1 / y ≥ 4 / (x + y) := sorry
end Example
```