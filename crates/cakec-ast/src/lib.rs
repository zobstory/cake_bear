//! AST node definitions for cakebear.
//!
//! Pure data: no logic lives here. Every node carries a [`Span`] so later
//! phases (sema, IR, codegen) can attach diagnostics to source locations.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}
