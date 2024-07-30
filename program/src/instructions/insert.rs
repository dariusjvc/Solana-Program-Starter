use borsh::BorshSerialize;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::state::InsertIdentityInfo;
use solana_program::program_error::ProgramError;

pub fn insert_data(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    insert_identity_info: InsertIdentityInfo,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let base_pk = next_account_info(accounts_iter)?;
    let target_account = next_account_info(accounts_iter)?;

    msg!("Inside the Insert method...");
    msg!("The target account is: {:?}", target_account.key.to_string());


        insert_identity_info
            .details
            .serialize(&mut &mut target_account.data.borrow_mut()[..])?;

    Ok(())
}
