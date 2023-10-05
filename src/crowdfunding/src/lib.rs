#[allow(unused_imports)]
#[allow(unused_variables)]

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
	account_info::{next_account_info, AccountInfo},
	entrypoint,
	entrypoint::ProgramResult,
	msg,
	program_error::ProgramError,
	pubkey::Pubkey,
    sysvar::Sysvar,
    sysvar::rent::Rent,
};

use std::convert::TryInto;


const UNAUTHORIZED_ERROR: u32 = 0x0A; 
const INSUFFICIENT_FUNDS_ERROR: u32 = 0x0B;
const INSUFFICIENT_LAMPORTS_ERROR: u32 = 0x0C;


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CampaignAccount {
    pub campaign_owner: Pubkey,
    pub campaign_amount: u64,
    pub campaign_description: String,
    pub campaign_fulfilled: u64,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey, 
    accounts: &[AccountInfo], 
    data: &[u8], 
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let campaign_account = next_account_info(accounts_iter)?;

    let (instruction_byte, rest_of_data) = data.split_first().unwrap();

    let amount = rest_of_data
      .get(..8)
      .and_then(|slice| slice.try_into().ok())
      .map(u64::from_le_bytes)
      .unwrap();
      
    let description = String::from_utf8(rest_of_data[9..].to_vec()).unwrap();      

    if *instruction_byte == 0 {
      let campaign_owner_account = next_account_info(accounts_iter)?;
      let mut campaign_account_data = CampaignAccount::try_from_slice(&campaign_account.data.borrow())?;
      campaign_account_data.campaign_owner = *campaign_owner_account.owner;
      campaign_account_data.campaign_amount = amount;
      campaign_account_data.campaign_description = description;
      campaign_account_data.campaign_fulfilled = 0;
      campaign_account_data.serialize(&mut &mut campaign_account.data.borrow_mut()[..])?;
    }

    if *instruction_byte == 1 {
      //get campaign status 

      let mut campaign_account_data = CampaignAccount::try_from_slice(&campaign_account.data.borrow())?;
      msg!("{}",campaign_account_data.campaign_amount - campaign_account_data.campaign_fulfilled);

    }

    if *instruction_byte == 2 {
        // Fetch the campaign account data.
        let campaign_account_data = CampaignAccount::try_from_slice(&campaign_account.data.borrow())?;
    
        // Calculate the remaining amount needed for campaign fulfillment.
        let funds_needed = campaign_account_data.campaign_amount
            .saturating_sub(campaign_account_data.campaign_fulfilled);
        
        // Display or use the information about remaining funds needed.
        msg!("Funds needed to fulfill the campaign: {}", funds_needed);
    }
    

    if *instruction_byte == 3 {
        // Withdraw and close campaign logic.
        let requester_account = next_account_info(accounts_iter)?;
        let mut campaign_account_data = CampaignAccount::try_from_slice(&campaign_account.data.borrow())?;
    
        // Check ownership and campaign status (e.g., if it's fulfilled)
        if campaign_account_data.campaign_owner != *requester_account.key {
            return Err(ProgramError::Custom(UNAUTHORIZED_ERROR));
        }
    
        // Assume the campaign has a target amount that needs to be met
        // for withdrawal to be allowed. Add this check if relevant.
        if campaign_account_data.campaign_fulfilled < campaign_account_data.campaign_amount {
            return Err(ProgramError::Custom(INSUFFICIENT_FUNDS_ERROR));
        }
    
        // Ensure that there are enough lamports in the campaign account
        let rent = Rent::get()?;
        let rent_exempt_reserve = rent.minimum_balance(campaign_account.data_len());
        if **campaign_account.lamports.borrow() < rent_exempt_reserve + campaign_account_data.campaign_amount {
            return Err(ProgramError::Custom(INSUFFICIENT_LAMPORTS_ERROR));
        }
    
        // Transfer lamports from campaign account to requester account
        **campaign_account.try_borrow_mut_lamports()? -= campaign_account_data.campaign_amount;
        **requester_account.try_borrow_mut_lamports()? += campaign_account_data.campaign_amount;
    
        // Mark the campaign as closed and save it back
        campaign_account_data.campaign_fulfilled = 0; // Or another way to mark it as closed/withdrawn
        campaign_account_data.serialize(&mut &mut campaign_account.data.borrow_mut()[..])?;
    }
    
    

    Ok(())
}
