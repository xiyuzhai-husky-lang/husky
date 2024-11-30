import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Data.Real.Basic
import Mathlib.Tactic.Explode

namespace Example1
-- Then $1+1=2$
def h : 1 + 1 = 2 := sorry
end Example1

namespace Example2
-- Let $x\in\mathbb{R}$
variable (x : ℝ)

-- Then $x^2\ge 0$
def h : x ^ 2 ≥ 0 := sorry
end Example2

namespace Example3
-- Let $a\in\mathbb{R}$
variable (a : ℝ)

-- Let $b\in\mathbb{R}$
variable (b : ℝ)

-- Then $a+b=b+a$
def h : a + b = b + a := sorry
end Example3

namespace Example4
-- Let $x\in\mathbb{R}$
variable (x : ℝ)

-- Then $x^2 \ge 0$
def h : x ^ 2 ≥ 0 := sorry
end Example4

namespace Example5
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
end Example5

namespace Example6
-- Let $x\in\mathbb{R}$
variable (x : ℝ)

-- Then $x + \frac{1}{x} - 2 = \frac{x^2 + 1 - 2x}{x}$
def h : x + 1 / x - 2 = (x ^ 2 + 1 - 2 * x) / x := sorry

-- Then $\frac{x^2 + 1 - 2x}{x} = \frac{{(x-1)}^2}{x}$
def h1 : (x ^ 2 + 1 - 2 * x) / x = (x - 1) ^ 2 / x := sorry

-- Then $x + \frac{1}{x} - 2 = \frac{{(x-1)}^2}{x}$
def h2 : x + 1 / x - 2 = (x - 1) ^ 2 / x := sorry

-- Then $\frac{{(x-1)}^2}{x} \ge 0$
def h3 : (x - 1) ^ 2 / x ≥ 0 := sorry

-- Then $x + \frac{1}{x} - 2 \ge 0$
def h4 : x + 1 / x - 2 ≥ 0 := sorry

-- Then $x + \frac{1}{x} \ge 2$
def h5 : x + 1 / x ≥ 2 := sorry
end Example6

namespace Example7
-- Let $x\in\mathbb{R}$
variable (x : ℝ)

-- Then $x^2 + 1 - 2x = {(x-1)}^2$
def h : x ^ 2 + 1 - 2 * x = (x - 1) ^ 2 := sorry

-- Then ${(x-1)}^2 \ge 0$
def h1 : (x - 1) ^ 2 ≥ 0 := sorry

-- Then $x^2 + 1 - 2x \ge 0$
def h2 : x ^ 2 + 1 - 2 * x ≥ 0 := sorry

-- Then $x^2 + 1 \ge 2x$
def h3 : x ^ 2 + 1 ≥ 2 * x := sorry
end Example7

namespace Example8
-- Let $x\in\mathbb{R}$
variable (x : ℝ)

-- Assume $x > 0$
variable (h : x > 0)

-- Then $x + 1 - 2\sqrt{x} = {(\sqrt{x}-1)}^2$
def h1 : x + 1 - 2 * (√ x) = ((√ x) - 1) ^ 2 := sorry

-- Then ${(\sqrt{x}-1)}^2 \ge 0$
def h2 : ((√ x) - 1) ^ 2 ≥ 0 := sorry

-- Then $x + 1 - 2\sqrt{x} \ge 0$
def h3 : x + 1 - 2 * (√ x) ≥ 0 := sorry

-- Then $x + 1 \ge 2\sqrt{x}$
def h4 : x + 1 ≥ 2 * (√ x) := sorry
end Example8

namespace Example9
-- Let $x\in\mathbb{R}$
variable (x : ℝ)

-- Let $y\in\mathbb{R}$
variable (y : ℝ)

-- Then $\frac{1}{x} + \frac{1}{y} - \frac{4}{x+y} = \frac{y(x+y) + x(x+y) - 4xy}{xy(x+y)}$
def h : 1 / x + 1 / y - 4 / (x + y) = (y * (x + y) + x * (x + y) - 4 * x * y) / (x * y * (x + y)) := sorry

-- Then $\frac{y(x+y) + x(x+y) - 4xy}{xy(x+y)} = \frac{yx + x^2 + x^2 + yx - 4xy}{xy(x+y)}$
def h1 : (y * (x + y) + x * (x + y) - 4 * x * y) / (x * y * (x + y)) = (y * x + x ^ 2 + x ^ 2 + y * x - 4 * x * y) / (x * y * (x + y)) := sorry

-- Then $\frac{yx + x^2 + x^2 + yx - 4xy}{xy(x+y)} = \frac{x^2 + x^2 -2xy}{xy(x+y)}$
def h2 : (y * x + x ^ 2 + x ^ 2 + y * x - 4 * x * y) / (x * y * (x + y)) = (x ^ 2 + x ^ 2 - 2 * x * y) / (x * y * (x + y)) := sorry

-- Then $\frac{x^2 + x^2 -2xy}{xy(x+y)} = \frac{{(x-y)}^2}{xy(x+y)}$
def h3 : (x ^ 2 + x ^ 2 - 2 * x * y) / (x * y * (x + y)) = (x - y) ^ 2 / (x * y * (x + y)) := sorry

-- Then $\frac{{(x-y)}^2}{xy(x+y)} \ge 0$
def h4 : (x - y) ^ 2 / (x * y * (x + y)) ≥ 0 := sorry

-- Then $\frac{1}{x} + \frac{1}{y} - \frac{4}{x+y} \ge 0$
def h5 : 1 / x + 1 / y - 4 / (x + y) ≥ 0 := sorry

-- Then $\frac{1}{x} + \frac{1}{y} \ge \frac{4}{x+y}$
def h6 : 1 / x + 1 / y ≥ 4 / (x + y) := sorry
end Example9

namespace Example10
-- Let $a\in\mathbb{R}$
variable (a : ℝ)

-- Let $b\in\mathbb{R}$
variable (b : ℝ)

-- Then $\frac{a}{b} + \frac{b}{a} - 2 = \frac{a^2 + b^2 - 2ab}{ab}$
def h : a / b + b / a - 2 = (a ^ 2 + b ^ 2 - 2 * a * b) / (a * b) := sorry

-- Then $\frac{a^2 + b^2 - 2ab}{ab} = \frac{{(a-b)}^2}{ab}$
def h1 : (a ^ 2 + b ^ 2 - 2 * a * b) / (a * b) = (a - b) ^ 2 / (a * b) := sorry

-- Then $\frac{{(a-b)}^2}{ab} \ge 0$
def h2 : (a - b) ^ 2 / (a * b) ≥ 0 := sorry

-- Then $\frac{a}{b} + \frac{b}{a} - 2 \ge 0$
def h3 : a / b + b / a - 2 ≥ 0 := sorry

-- Then $\frac{a}{b} + \frac{b}{a} \ge 2$
def h4 : a / b + b / a ≥ 2 := sorry
end Example10

namespace Example11
-- Let $x\in\mathbb{R}$
variable (x : ℝ)

-- Let $y\in\mathbb{R}$
variable (y : ℝ)

-- Then $x^2 + y^2 - 2xy = {(x-y)}^2$
def h : x ^ 2 + y ^ 2 - 2 * x * y = (x - y) ^ 2 := sorry

-- Then ${(x-y)}^2 \ge 0$
def h1 : (x - y) ^ 2 ≥ 0 := sorry

-- Then $x^2 + y^2 - 2xy \ge 0$
def h2 : x ^ 2 + y ^ 2 - 2 * x * y ≥ 0 := sorry

-- Then $x^2 + y^2 \ge 2xy$
def h3 : x ^ 2 + y ^ 2 ≥ 2 * x * y := sorry
end Example11
