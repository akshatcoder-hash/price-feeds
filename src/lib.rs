use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use serde::{Serialize, Deserialize};

entrypoint!(process_instruction);

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceData {
    pub btc_price: u64,
    pub eth_price: u64,
    pub sol_price: u64,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        msg!("Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut price_data = PriceData {
        btc_price: 0,
        eth_price: 0,
        sol_price: 0,
    };

    let btc_price = u64::from_le_bytes(instruction_data[0..8].try_into().unwrap());
    let eth_price = u64::from_le_bytes(instruction_data[8..16].try_into().unwrap());
    let sol_price = u64::from_le_bytes(instruction_data[16..24].try_into().unwrap());

    price_data.btc_price = btc_price;
    price_data.eth_price = eth_price;
    price_data.sol_price = sol_price;

    let mut data = account.try_borrow_mut_data()?;
    data[..].copy_from_slice(&bincode::serialize(&price_data).unwrap());

    Ok(())
}
