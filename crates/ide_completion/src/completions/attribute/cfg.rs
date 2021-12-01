//! Completion for cfg

use std::iter;

use syntax::SyntaxKind;

use crate::{
    completions::Completions, context::CompletionContext, CompletionItem, CompletionItemKind,
};

const KNOWN_ARCH: [&str; 19] = [
    "aarch64",
    "arm",
    "avr",
    "hexagon",
    "mips",
    "mips64",
    "msp430",
    "nvptx64",
    "powerpc",
    "powerpc64",
    "riscv32",
    "riscv64",
    "s390x",
    "sparc",
    "sparc64",
    "wasm32",
    "wasm64",
    "x86",
    "x86_64",
];

const KNOWN_ENV: [&str; 7] = [
    "eabihf",
    "gnu",
    "gnueabihf",
    "msvc",
    "relibc",
    "sgx",
    "uclibc",
];

const KNOWN_OS: [&str; 20] = [
    "cuda",
    "dragonfly",
    "emscripten",
    "freebsd",
    "fuchsia",
    "haiku",
    "hermit",
    "illumos",
    "l4re",
    "linux",
    "netbsd",
    "none",
    "openbsd",
    "psp",
    "redox",
    "solaris",
    "uefi",
    "unknown",
    "vxworks",
    "windows",
];

const KNOWN_VENDOR: [&str; 8] = [
    "apple", "fortanix", "nvidia", "pc", "sony", "unknown", "wrs", "uwp",
];
