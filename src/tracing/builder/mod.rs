//! Builder types for building traces

/// Geth style trace builders for `debug_` namespace
pub mod geth;

/// Parity style trace builders for `trace_` namespace
pub mod parity;

// Sentio trace
pub mod sentio;

/// Walker types used for traversing various callgraphs
mod walker;
