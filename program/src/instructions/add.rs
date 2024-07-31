use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    system_instruction,
    rent::Rent,
    program::invoke,
    sysvar::Sysvar,
};
use crate::state::AddInfo;


pub fn insert_data(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    add_info: AddInfo,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let target_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let add_info_boxed = Box::new(add_info);

    let account_span = add_info_boxed.try_to_vec()?.len();
    let lamports_required = Rent::get()?.minimum_balance(account_span);

    invoke(
        &system_instruction::create_account(
            payer.key,
            target_account.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[
            payer.clone(),
            target_account.clone(),
            system_program.clone(),
        ],
    )?;
    
    msg!("Info is going to be added");

    add_info_boxed.serialize(&mut &mut target_account.data.borrow_mut()[..])?;
    Ok(())
}
