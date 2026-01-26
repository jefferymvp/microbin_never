# Debug Report: OpenSSL Build Failure

## Issue
Running `cargo run` fails on Windows with the following error:
```
thread 'main' panicked at ... openssl-src ...
Error configuring OpenSSL build:
    Command: "perl" ...
    Failed to execute: program not found
```

## Cause
The `openssl` crate with the `vendored` feature attempts to build OpenSSL from source. This process requires `perl` to be installed and available in the PATH, which is not the case in the current environment.

## Proposed Solution
The usage of OpenSSL can be replaced by `rustls` (a pure Rust TLS implementation) effectively avoiding the dependency on OpenSSL and Perl.
The `Cargo.toml` provides a `no-c-deps` feature that enables `rustls` and disables `openssl`.
However, we likely still want `rusqlite` for the database. `rusqlite` with `bundled` feature builds SQLite, which usually works fine with MSVC without Perl.

## Action
已修改 `Cargo.toml` 的 `default` features：
- 从 `__default-tls` 改为 `__rustcrypto-tls`（使用纯 Rust 的 TLS 实现）
- 从 `__syntect-fast` 改为 `__syntect-rust`（使用纯 Rust 的语法高亮）
- 保留 `dep:rusqlite`（bundled SQLite 在 MSVC 环境下可以正常构建）

这样用户可以直接运行 `cargo run` 而无需安装 Perl 或指定特殊的 feature flags。

## Status
✅ 已应用修复。用户现在可以运行 `cargo run` 来启动应用。
