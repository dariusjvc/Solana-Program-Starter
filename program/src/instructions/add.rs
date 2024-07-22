use borsh::BorshSerialize;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::state::AddInfo;
use solana_program::program_error::ProgramError;

pub fn insert_data(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    add_info: AddInfo,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let pk1 = next_account_info(accounts_iter)?;
    let pk2 = next_account_info(accounts_iter)?;

    msg! ("Info is going to be added");

    add_info.details.serialize(&mut &mut target_account.data.borrow_mut()[..])?;
    Ok(())
}