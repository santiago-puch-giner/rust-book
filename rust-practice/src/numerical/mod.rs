// Exports the recursive module
pub mod recursive;

// Re-exports the function so that one can import with:
// use crate::numerical::fibonacci;
// Instead of
// use crate::numerical::recursive::fibonacci;
pub use recursive::fibonacci;
