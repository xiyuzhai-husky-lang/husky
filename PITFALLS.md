don't `println!()` in released code for vscode extension;
this will cause wierd behaviors

user may use `lib.rs` instead of `lib.hsy`, there should be errors shown on `Corgi.toml` for that

## no libtorch found

It might be fish environment variable set flags (use -gx)

## error: undefined symbol: c10::detail::torchCheckFail

If this error occurs when building tch-rs, this might be you are using the libtorch.so from the python package which doesn't include all dependencies.

## Vscode environment variables not updated for rust-analyzer actions

Restart vscode

## LD_LIBRARY_PATH missing paths outside target

<https://doc.rust-lang.org/cargo/reference/environment-variables.html#dynamic-library-paths>

## Unresolved import everywhere

Mismatched edition key in Cargo.toml

## Vscode Extension not working, without error message

making sure there is no type error in typescript code.

Last resort is to compare with rust-analyzer.

Looking out for infinite loops.

## Couldn't find Vscode Publisher

https://marketplace.visualstudio.com/manage/publishers/husky-lang

## lsp-type version

94.1 sucks, so many bugs it caused, use 94.0

## debug_with_db not working

check that the db trait extends the other db trait

## introduce a dependency and things break down

may be the feature flags the dependency introduces into other dependencies. Say smallvec/union for instance.