# HuskyTex

In the normal language, text -> token -> ast -> syn expr -> sema expr.

However, here text <--> token <--> ast -> syn expr -> sema expr.

Further text <--> token <--> ast is totally handled automatically.

Basically, the user modify directly the asts.

Math asts is centered around displayability, basically the same as typst ast.

HuskyTex is a little different from LaTex in that
- greek letters like `α`, `β`, ... and special symbols like `∀`, `∃` are included for better readability;
- `{...}` in math mode can be interpreted directly as set notation if the content inside doesn't begin with `_` or `^`;
- all `/` is interpreted as fraction, and thus no fraction command;
- ...