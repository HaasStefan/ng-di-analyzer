<p align="center">
  <img src="https://github.com/user-attachments/assets/04d90b69-9ad9-4300-94e2-f8fe7ec94b7b" alt="ng-di-analyzer logo" width="250" height="250" />
</p>

<h1 align="center">ng-di-analyzer</h1>

<p align="center">
  🧠 Rust-powered static analysis tool to detect <code>NullInjectorError</code> issues in Angular apps at compile-time.
</p>

---

## What is `ng-di-analyzer`?

`ng-di-analyzer` is a blazing-fast CLI tool written in Rust that statically analyzes Angular projects to construct a full **Dependency Injection (DI) tree** and identify all **possible paths** that could lead to runtime `NullInjectorError`'s.

Angular's DI system is powerful—but also brittle. In large applications, it’s easy to misconfigure providers or forget to add them in the right context. The result? A frustrating and often hard-to-reproduce `NullInjectorError` at runtime.

This tool shifts that pain to **compile time**.

## Why use it?

- ✅ Catch `NullInjectorError`s **before** they happen.
- ⚡ Rust-powered speed: Analyze large Angular codebases in seconds.
- 🧩 Understand your DI graph like never before.
- 🔍 Output all potential injection paths leading to errors.
