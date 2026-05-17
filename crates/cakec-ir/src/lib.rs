//! Lowered, SSA-ish IR — the contract between frontend and backend.
//!
//! Phase 1 keeps the surface minimal, but spans and types live on every
//! node so later passes (generics monomorphization, async lowering, GC
//! barriers) have a stable place to attach.
