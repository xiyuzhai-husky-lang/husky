# HuskyTex

In the normal language, text -> token -> ast -> syn expr -> sema expr.

However, here text <--> token <--> ast -> syn expr -> sema expr.

Further text <--> token <--> ast is totally handled automatically.

Basically, the user modify directly the asts.

HuskyTex asts is centered around displayability, what looks the same is processed the same way.

HuskyTex differs from LaTex in that
- greek letters like `α`, `β`, ... and special symbols like `∀`, `∃` are allowed for better readability;
- `{...}` in math mode can be interpreted directly as set notation if the content inside doesn't begin with `_` or `^`;
- all `/` is interpreted as fraction, and thus no fraction command is needed for faster typing;
- ...

However, HuskyTex is designed to be as backward-compatible with LaTex as possible for easier data acquirement.