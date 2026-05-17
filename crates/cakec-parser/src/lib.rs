//! Parser: token stream → `cakec_ast::Program`.
//!
//! Hand-rolled recursive descent + Pratt for expressions. Diagnostics carry
//! source spans so the CLI can render them with surrounding context.
