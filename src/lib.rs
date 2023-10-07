use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use serde::{Serialize, Deserialize};
use std::convert::TryInto;

entrypoint!(process_instruction);

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceData {
    pub ftt_price: u64,  // FTT price
    pub hpos_price: u64,  // HPOS price
    pub wld_price: u64,  // WLD price
    pub ldo_price: u64,  // LDO price
    pub gmx_price: u64,  // GMX price
    pub link_price: u64,  // LINK price
    pub dydx_price: u64,  // dYdX price
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
        ftt_price: 0,
        hpos_price: 0,
        wld_price: 0,
        ldo_price: 0,
        gmx_price: 0,
    };

    let ftt_price = u64::from_le_bytes(instruction_data[0..8].try_into().unwrap());
    let hpos_price = u64::from_le_bytes(instruction_data[8..16].try_into().unwrap());
    let wld_price = u64::from_le_bytes(instruction_data[16..24].try_into().unwrap());
    let ldo_price = u64::from_le_bytes(instruction_data[24..32].try_into().unwrap());
    let gmx_price = u64::from_le_bytes(instruction_data[32..40].try_into().unwrap());

    price_data.ftt_price = ftt_price;
    price_data.hpos_price = hpos_price;
    price_data.wld_price = wld_price;
    price_data.ldo_price = ldo_price;
    price_data.gmx_price = gmx_price;

    let mut data = account.try_borrow_mut_data()?;
    data[..].copy_from_slice(&bincode::serialize(&price_data).unwrap());

    Ok(())
}
