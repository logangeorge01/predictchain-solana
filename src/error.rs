//! Error types

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::{
    decode_error::DecodeError, msg, program_error::PrintProgramError, program_error::ProgramError,
};
use thiserror::Error;

/// Errors that may be returned by the PredictChain program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum PredictChainError {
    /// Generic error for now
    #[error("PredictChain generic error")]
    GenericError,

    // /// PredictChain account already in use
    // #[error("PredictChain account already in use")]
    // AlreadyInUse,
    // /// Deposit account already in use
    // #[error("Deposit account already in use")]
    // DepositAccountInUse,
    // /// Token mint account already in use
    // #[error("Token account already in use")]
    // TokenMintInUse,
    // /// Invalid seed or bump_seed was provided
    // #[error("Failed to generate program account because of invalid data")]
    // InvalidAuthorityData,
    // /// Invalid authority account provided
    // #[error("Invalid authority account provided")]
    // InvalidAuthorityAccount,
    // /// Lamport balance below rent-exempt threshold.
    // #[error("Lamport balance below rent-exempt threshold")]
    // NotRentExempt,
    // /// Expected an SPL Token mint
    // #[error("Input token mint account is not valid")]
    // InvalidTokenMint,
    // /// Amount should be more than zero
    // #[error("Amount should be more than zero")]
    // InvalidAmount,
    // /// Wrong decider account
    // #[error("Wrong decider account was sent")]
    // WrongDeciderAccount,
    // /// Signature missing in transaction
    // #[error("Signature missing in transaction")]
    // SignatureMissing,
    // /// Decision was already made for this PredictChain
    // #[error("Decision was already made for this PredictChain")]
    // DecisionAlreadyMade,
    // /// Decision can't be made in current slot
    // #[error("Decision can't be made in current slot")]
    // InvalidSlotForDecision,
    // /// Deposit can't be made in current slot
    // #[error("Deposit can't be made in current slot")]
    // InvalidSlotForDeposit,
    // /// No decision has been made yet
    // #[error("No decision has been made yet")]
    // NoDecisionMadeYet,
}

impl From<PredictChainError> for ProgramError {
    fn from(e: PredictChainError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for PredictChainError {
    fn type_of() -> &'static str {
        "PredictChain Error"
    }
}

impl PrintProgramError for PredictChainError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            PredictChainError::GenericError => msg!("Error: PredictChain generic error message"),
            // PredictChainError::AlreadyInUse => msg!("Error: PredictChain account already in use"),
            // PredictChainError::DepositAccountInUse => msg!("Error: Deposit account already in use"),
            // PredictChainError::TokenMintInUse => msg!("Error: Token account already in use"),
            // PredictChainError::InvalidAuthorityData => {
            //     msg!("Error: Failed to generate program account because of invalid data")
            // }
            // PredictChainError::InvalidAuthorityAccount => msg!("Error: Invalid authority account provided"),
            // PredictChainError::NotRentExempt => msg!("Error: Lamport balance below rent-exempt threshold"),
            // PredictChainError::InvalidTokenMint => msg!("Error: Input token mint account is not valid"),
            // PredictChainError::InvalidAmount => msg!("Error: Amount should be more than zero"),
            // PredictChainError::WrongDeciderAccount => msg!("Error: Wrong decider account was sent"),
            // PredictChainError::SignatureMissing => msg!("Error: Signature missing in transaction"),
            // PredictChainError::DecisionAlreadyMade => {
            //     msg!("Error: Decision was already made for this PredictChain")
            // }
            // PredictChainError::InvalidSlotForDecision => {
            //     msg!("Error: Decision can't be made in current slot")
            // }
            // PredictChainError::InvalidSlotForDeposit => msg!("Deposit can't be made in current slot"),
            // PredictChainError::NoDecisionMadeYet => msg!("Error: No decision has been made yet"),
        }
    }
}
