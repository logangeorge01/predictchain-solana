//! Program state processor

use crate::{
    error::PredictChainError,
    instruction::PredictChainInstruction,
    state::{EventAccount},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::next_account_info,
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey
};

/// Program state handler.
pub struct Processor {}
impl Processor {
    /// Processes an instruction
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction = PredictChainInstruction::try_from_slice(input)?;
        match instruction {
            PredictChainInstruction::PurchaseShares(args) => {
                msg!("Instruction: PurchaseShares");
                Self::purchase_shares(
                    program_id,
                    accounts,
                    args.side_index,
                    args.num_tokens
                )
            }
        }
    }


    /// Calculates the authority id by generating a program address.
    pub fn authority_id(
        program_id: &Pubkey,
        seed: &Pubkey,
        bump_seed: u8,
    ) -> Result<Pubkey, ProgramError> {
        Pubkey::create_program_address(&[&seed.to_bytes()[..32], &[bump_seed]], program_id)
            .map_err(|_| PredictChainError::GenericError.into())
    }

    /// Mint tokens
    pub fn mint<'a>(
        token_program_id: AccountInfo<'a>,
        mint_account: AccountInfo<'a>,
        destination_account: AccountInfo<'a>,
        authority_account: AccountInfo<'a>,
        amount: u64,
        event_pubkey: &Pubkey,
        bump_seed: u8,
    ) -> ProgramResult {
        let me_bytes = event_pubkey.to_bytes();
        let authority_signature_seeds = [&me_bytes[..32], &[bump_seed]];
        let signers = &[&authority_signature_seeds[..]];

        invoke_signed(
            &spl_token::instruction::mint_to(
                token_program_id.key,
                mint_account.key,
                destination_account.key,
                authority_account.key,
                &[authority_account.key],
                amount,
            )
            .unwrap(),
            &[
                token_program_id,
                mint_account,
                destination_account,
                authority_account,
            ],
            signers,
        )
    }

    /// Purchase Shares instruction
    pub fn purchase_shares(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        side_index: u8,
        num_tokens: u64
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        let user_signer = next_account_info(account_iter)?;
        let authority = next_account_info(account_iter)?;
        let event = next_account_info(account_iter)?;
        let yes_mint = next_account_info(account_iter)?;
        let no_mint = next_account_info(account_iter)?;
        let user_token_account = next_account_info(account_iter)?;
        let payment_account = next_account_info(account_iter)?;
        let token_program_id = next_account_info(account_iter)?;

        // side index must be in [0, 1]
        if side_index > 1 {
            return Err(PredictChainError::GenericError.into());
        }

        // num tokens must be > 0
        if num_tokens == 0 {
            return Err(PredictChainError::GenericError.into());
        }

        if event.owner != program_id {
            // msg!("event account isn't owned by program");
            return Err(PredictChainError::GenericError.into());
        }
        if payment_account.owner != program_id {
            // msg!("payment account isn't owned by program");
            return Err(PredictChainError::GenericError.into());
        }
        if !user_signer.is_signer {
            // msg!("user should be signer");
            return Err(PredictChainError::GenericError.into());
        }

        let mint_to_use = if side_index == 0 { yes_mint } else { no_mint };

        // msg!("129");
        // msg!("{}", event.key);
        let mut event_data = EventAccount::try_from_slice(&event.data.borrow())?;
        msg!("131");

        msg!("event pubkey: {}", event.key);        

        let authority_pub_key = Self::authority_id(program_id, event.key, event_data.bump_seed)?;
        msg!("135");
        msg!("bumpseed: {}", event_data.bump_seed);
        msg!("received authority: {}", *authority.key);
        msg!("calculated authority: {}", authority_pub_key);
        if *authority.key != authority_pub_key {
            msg!("136");
            return Err(PredictChainError::GenericError.into());
        }

        msg!("140");

        // mint tokens to user account
        Self::mint(
            token_program_id.clone(),
            mint_to_use.clone(),
            user_token_account.clone(),
            authority.clone(),
            num_tokens,
            event.key,
            event_data.bump_seed,
        )?;

        msg!("152");

        // update volume in event account
        event_data.volume += **payment_account.lamports.borrow();
        event_data.serialize(&mut &mut event.data.borrow_mut()[..])?;

        msg!("158");

        // transfer solana to event account
        **event.try_borrow_mut_lamports()? += **payment_account.lamports.borrow();
        **payment_account.try_borrow_mut_lamports()? = 0;

        Ok(())
    }

}
