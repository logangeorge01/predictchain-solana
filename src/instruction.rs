//! Instruction types

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Arguments for PurchaseSharesArgs
#[derive(BorshSerialize, BorshDeserialize)]
pub struct PurchaseSharesArgs {
    /// index of user's desired event side: 0-YES, 1-NO
    pub side_index: u8,
    /// number of tokens to mint
    pub num_tokens: u64
}

/// Instruction definition
#[derive(BorshSerialize, BorshDeserialize)]
pub enum PredictChainInstruction {
    /// Purchase shares which mints tokens to a user
    /// 
    /// 0. `[s]` User signer
    /// 1. `[]` Authority (Program Derived Address)
    /// 2. `[w]` Event account
    /// 3. `[w]` YES token mint
    /// 4. `[w]` NO token mint
    /// 5. `[w]` Associated token account to receive shares
    /// 6. `[w]` Payment PDA
    /// 7. `[]` Token program id
    PurchaseShares(PurchaseSharesArgs)
}

/// Create `PurchaseShares` instruction
#[allow(clippy::too_many_arguments)]
pub fn purchase_shares(
    program_id: &Pubkey,
    user_signer: &Pubkey,
    authority: &Pubkey,
    event: &Pubkey,
    yes_mint: &Pubkey,
    no_mint: &Pubkey,
    user_token_account: &Pubkey,
    payment_account: &Pubkey,
    token_program_id: &Pubkey,
    args: PurchaseSharesArgs
) -> Result<Instruction, ProgramError> {
    let init_data = PredictChainInstruction::PurchaseShares(args);
    let data = init_data.try_to_vec()?;

    let accounts = vec![
        AccountMeta::new_readonly(*user_signer, true),
        AccountMeta::new_readonly(*authority, false),
        AccountMeta::new(*event, false),
        AccountMeta::new(*yes_mint, false),
        AccountMeta::new(*no_mint, false),
        AccountMeta::new(*user_token_account, false),
        AccountMeta::new(*payment_account, false),
        AccountMeta::new_readonly(*token_program_id, false)
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}
