# crates

## abstraction

abstraction of data structure and traits.

## apps

applications for language users.

## benchmarks

benchmarks.

## comptime

everything about compilation.

## devsoul

The specification of husky devtime implementation.

## devtime

The backend, functioning as compiler + debugger.

## fs

Everything about file system, such as toolchain, workspace, package, crate, module, etc. A safe wrapper that supports incremental computation.

## gadgets

simple tools for husky language develepment

## gui

Graphics user interface libraries for husky debugger.

## hir

High-level intermediate representation. It includes only runtime constructs, and excludes those compterms that don't affect runtime behavior. Everything is carefully stored as interned values for version stamp purposes.

## ide

IDE support, such as hover info, auto completion, diagnostics et al.

## kernel

The kernel of the language, fundamental things of the language design including the definition of entity paths, terms, etc.

## ki

`Ki` is a node in the generalized computation graph.

This crate group is about its definition, representation, etc.

## lex

Lexical analysis.

Decompose source code to tokens.

There are two kinds of tokens, absolute and regional.

## linket

Linket is the way that runtime or interpreter is able to run compiled code.

## linktime

Linktime is the struct that stores dynamic libraries and provides up-to-date linkages.

## protocols

Protocols are communication standards between husky debugger server and client.

## runtime

Runtime.

## semantics

everything about semantics, such as type inference, implicit augment inference.

## syntax

## toml

Everything about toml. Husky implements its own toml parser to provide the best error messages.

## utils

## vm
