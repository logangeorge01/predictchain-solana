//! State transition types

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// EventAccount struct.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct EventAccount {
    /// bump_seed
    pub bump_seed: u8,
    /// resolve_authority
    pub resolve_authority: Pubkey,
    /// yes_mint_address
    pub yes_mint_address: Pubkey,
    /// no_mint_address:
    pub no_mint_address: Pubkey,
    /// volume
    pub volume: u64
}

impl EventAccount {
    /// Length serialized data
    pub const LEN: usize = 105;
}
