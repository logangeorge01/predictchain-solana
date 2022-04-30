//! PredictChain
#![deny(missing_docs)]

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

// PredictChain id
solana_program::declare_id!("Ay9tmvYBVXC4n9SDrLhoS8noJfASC6Kcr8kxnRYKgbJG");
