# cake_bear

A Rust-implemented compiler for **TypeScript** that produces a single native binary.

> **Status:** Phase 1 (alpha). The compiler pipeline is being built phase by phase. The `cakec` CLI prints help today; `cakec build` is being wired in. See [`CLAUDE.md`](CLAUDE.md) for the language subset supported by the current phase.

## Why

TypeScript is one of the easiest mainstream languages to pick up and one of the most widely known. Rust and Go offer dramatically better runtime performance and a much simpler deployment story (one statically-linked binary, no runtime install). `cake_bear` aims to give TypeScript developers that same deployment and performance story without forcing them to learn a second language.

## Project goals

- **Source language:** full TypeScript, rolled out in phases.
- **Output:** a single native binary per program (no Node, no V8, no bundler).
- **Codegen backend:** [Cranelift](https://cranelift.dev/).
- **Concurrency:** familiar JS `async`/`await`, backed by an M:N work-stealing runtime — no new keywords to learn.
- **Performance target:** in the ballpark of Go/Rust.
- **Package management:** designed to avoid NPM-style ecosystem bloat. Details TBD in a later phase.

## Repo layout

```
cake_bear/
├── crates/                # the compiler workspace
│   ├── cakec-cli/         # `cakec` binary
│   ├── cakec-ast/         # AST nodes
│   ├── cakec-lexer/       # tokenizer
│   ├── cakec-parser/      # tokens → AST
│   ├── cakec-sema/        # name resolution + type checking
│   ├── cakec-ir/          # lowered SSA-ish IR
│   ├── cakec-codegen/     # Cranelift backend
│   └── cakec-runtime/     # linked into every compiled binary
└── examples/
    └── basic/
        └── main.ts        # smallest cakebear program
```

## Build the compiler

```sh
# One-time, if your local rustup is stale:
rustup update

cargo build --workspace --release
```

The resulting `cakec` binary lives at `target/release/cakec`.

## Compile the example

```sh
# From the repo root:
cargo run -p cakec-cli --release -- build examples/basic/main.ts
./examples/basic/bin/main
# 5
```

By convention, `cakec build <file>` writes the produced binary to `<project-root>/bin/<name>` — for `examples/basic/main.ts`, that's `examples/basic/bin/main`. Pass `-o <path>` to override the destination.

## Current phase scope

Phase 1 covers:

- Primitive types: `number`, `string`, `boolean`, `null`, `undefined`
- `const` / `let` with explicit type annotations
- Function declarations with typed parameters and return types
- Arithmetic, string concatenation with `+`, boolean operators
- `if` / `else` / `while` / `return`
- `console.log` on a primitive argument

Everything else — modules, classes, generics, `async`/`await`, arrays, the GC — lands in subsequent phases. See `CLAUDE.md` for the live phase definition.

## License

MIT — see [`LICENSE`](LICENSE).
