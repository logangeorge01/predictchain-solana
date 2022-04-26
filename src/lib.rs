use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() == 0 {
        return Err(ProgramError::InvalidInstructionData);
    }

    if instruction_data[0] == 0 {
        return handle_new_event(
            program_id,
            accounts,
            // &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[0] == 1 {
        return handle_purchase_shares(
            program_id,
            accounts,
            // &instruction_data[1..instruction_data.len()],
        );
    }

    msg!("Didn't found the entrypoint required");
    Err(ProgramError::InvalidInstructionData)
}
entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct EventPDA {
    pub resolve_authority: Pubkey,
    pub yes_mint_address: Pubkey,
    pub no_mint_address: Pubkey,
    pub volume: u64
}

fn handle_new_event(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    // instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let signer_account = next_account_info(accounts_iter)?;
    let event_pda = next_account_info(accounts_iter)?;
    let yes_mint_account = next_account_info(accounts_iter)?;
    let no_mint_account = next_account_info(accounts_iter)?;
    
    if !signer_account.is_signer {
        msg!("signer_account should be signer");
        return Err(ProgramError::IncorrectProgramId);
    }

    if event_pda.owner != program_id {
        msg!("event_pda isn't owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }

    // maybe check if mint authority is event pda pubkey for yes and no

    let mut event_data = EventPDA::try_from_slice(&event_pda.data.borrow())
        .expect("Instruction data serialization didn't worked");

    let rent_exemption = Rent::get()?.minimum_balance(event_pda.data_len());
    if **event_pda.lamports.borrow() < rent_exemption {
        msg!("{}", rent_exemption);
        msg!("{}", **event_pda.lamports.borrow());
        msg!("The balance of event_pda should be more then rent_exemption");
        return Err(ProgramError::InsufficientFunds);
    }

    event_data.yes_mint_address = *yes_mint_account.key;
    event_data.no_mint_address = *no_mint_account.key;

    event_data.serialize(&mut &mut event_pda.try_borrow_mut_data()?[..])?;

    Ok(())
}

fn handle_purchase_shares(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    // instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let writing_account = next_account_info(accounts_iter)?;
    let creator_account = next_account_info(accounts_iter)?;

    Ok(())
}
