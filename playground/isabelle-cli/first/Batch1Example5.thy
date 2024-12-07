theory Batch1Example5
  imports Complex_Main
begin

(* Let $x \in \mathbb{R}$. Assume $x > 0$. *)
lemma x_plus_one_over_x_ge_2:
  fixes x :: real
  assumes "x > 0"
  shows "x + 1/x >= 2"
proof -
  (* Then $x > 0$, so $\frac{1}{x}$ is defined and $\frac{1}{x} > 0$. *)
  from assms have "x > 0" by blast
  
  (* Then consider $x + \frac{1}{x} - 2$. *)
  (* Then $x + \frac{1}{x} - 2 = \frac{x}{1} + \frac{1}{x} - \frac{2}{1}$. *)
  have "x + 1/x - 2 = x + (1/x) - 2" by blast
  
  (* Then multiply and divide terms to get common denominator $x$. *)
  have "x + 1/x - 2 = (x^2 / x) + (1 / x) - (2 * x / x)" by blast

  (* Then combine terms under the denominator $x$. *)
  have "x + 1/x - 2 = (x^2 + 1 - 2 * x) / x" by blast

  (* Then rewrite the numerator $x^2 + 1 - 2x$ as $(x-1)^2$. *)
  have "x^2 + 1 - 2 * x = (x - 1)^2" by blast
  
  (* Then substitute back $x^2 + 1 - 2x = (x-1)^2$. *)
  have "x + 1/x - 2 = (x - 1)^2 / x" by blast

  (* Then $(x-1)^2 \geq 0$ because it is a square. *)
  have "(x - 1)^2 >= 0" by blast
  
  (* Then $x > 0$, so dividing $(x-1)^2$ by $x$ preserves the inequality. *)
  from `(x - 1)^2 >= 0` and `x > 0`
  have "(x - 1)^2 / x >= 0" by blast

  (* Then $x + \frac{1}{x} - 2 = \frac{(x-1)^2}{x}$. *)
  have "x + 1/x - 2 >= 0" by blast

  (* Then add 2 to both sides of $x + \frac{1}{x} - 2 \geq 0$. *)
  thus "x + 1/x >= 2" by blast
qed


end

