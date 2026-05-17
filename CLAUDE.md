# cake_bear

A Rust-implemented compiler for **TypeScript**, AOT-compiled to a single native binary.

> **Note:** `README.md` is from late 2023 and is out of date. Trust this file and the in-flight plan in `/Users/zob456/.claude/plans/` over the README until the README is rewritten.

## Locked architectural decisions

- **Source language:** full TypeScript, phased in. The current phase's language subset is the only thing supported at any given commit — see "Current phase" below.
- **Codegen backend:** Cranelift (`cranelift-codegen`).
- **Concurrency surface:** JS `async`/`await`, backed by an M:N work-stealing scheduler in `cakec-runtime`. Not Go-style `go`/`chan` keywords.
- **Repo layout:** Cargo workspace with 8 crates under `crates/`.

## Current phase: **Phase 1 — primitives + functions + `console.log`**

A `.ts` source file may use:
- Primitives: `number`, `string`, `boolean`, `null`, `undefined`.
- `const` / `let` with explicit type annotations.
- Function declarations with typed params and typed return.
- Arithmetic, string concatenation with `+`, boolean operators.
- `if` / `else` / `while` / `return`.
- `console.log` on a single primitive argument.

Out of scope until later phases: imports, modules, classes, generics, `any`/`unknown`, unions, literals, arrays, object types, async, GC, full stdlib.

## Workspace layout

```
cake_bear/
├── Cargo.toml                 # workspace
├── rust-toolchain.toml
├── crates/
│   ├── cakec-cli/             # `cakec build foo.ts` binary; wires the pipeline
│   ├── cakec-ast/             # AST nodes + spans (pure data)
│   ├── cakec-lexer/           # &str → Token stream
│   ├── cakec-parser/          # Tokens → ast::Program
│   ├── cakec-sema/            # name resolution + type checking
│   ├── cakec-ir/              # SSA-ish IR (the load-bearing contract)
│   ├── cakec-codegen/         # IR → object file via Cranelift → linker → binary
│   └── cakec-runtime/         # entry glue, intrinsics; later: GC, scheduler, stdlib
```

The IR in `cakec-ir` is the load-bearing contract between frontend and backend. Spans and types live on every node so later passes (generics monomorphization, async lowering, GC barriers) have somewhere to attach.

## Pipeline

```
source (&str)
  → cakec-lexer    → Token stream
  → cakec-parser   → ast::Program
  → cakec-sema     → typed AST + diagnostics
  → cakec-ir       → ir::Module
  → cakec-codegen  → object file → system linker → binary  (links cakec-runtime)
```

## Build / test / run

```sh
# One-time: pick up the toolchain pinned in rust-toolchain.toml
rustup update

# Build everything
cargo build --workspace

# Run the CLI
cargo run -p cakec-cli -- --help

# Tests (once they exist)
cargo test --workspace
```

## Per-phase planning

cake_bear is multi-year and will involve many contributors. Plan only the **current** phase in detail; defer later phases until they're the active phase. Don't write speculative roadmaps for things that aren't being built yet.
