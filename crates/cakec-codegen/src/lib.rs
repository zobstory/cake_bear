//! Codegen backend: `cakec_ir::Module` → native object file via Cranelift,
//! then linked against `cakec-runtime` by the system linker (`cc`) to
//! produce the final executable.
