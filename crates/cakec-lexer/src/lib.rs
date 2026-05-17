//! Lexer: TypeScript source `&str` → token stream.
//!
//! Phase 1 will cover identifiers, primitive-type keywords (`number`, `string`,
//! `boolean`, `null`, `undefined`), `const`/`let`/`function`/`return`/`if`/`else`/`while`,
//! numeric and string literals, and punctuation.
