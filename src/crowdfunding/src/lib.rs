use borsh::{BorshDeserialize, BorshSerialize};
use std::convert::TryInto;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

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

    else if *instruction_byte == 1 {
        // Get campaign status 
        let campaign_account_data = CampaignAccount::try_from_slice(&campaign_account.data.borrow())?;
        msg!("Remaining amount to fulfill the campaign: {}", campaign_account_data.campaign_amount - campaign_account_data.campaign_fulfilled);
    }

    else if *instruction_byte == 2 {
        // Example: Update campaign status 
        let mut campaign_account_data = CampaignAccount::try_from_slice(&campaign_account.data.borrow())?;
        // Check the campaign status and update if necessary (logic to be added per requirements)
        // Update the on-chain campaign data
        campaign_account_data.serialize(&mut &mut campaign_account.data.borrow_mut()[..])?;
    }

    else if *instruction_byte == 3 {
        // Example: Withdraw funds from campaign
        let withdrawer_account = next_account_info(accounts_iter)?;
        let mut campaign_account_data = CampaignAccount::try_from_slice(&campaign_account.data.borrow())?;

        // Validate that the withdrawer is the campaign owner
        if campaign_account_data.campaign_owner != *withdrawer_account.key {
            return Err(ProgramError::Custom(0x01)); // Error: Unauthorized
        }

        // Validate that the campaign is fundable or has been funded 
        if campaign_account_data.campaign_fulfilled < campaign_account_data.campaign_amount {
            return Err(ProgramError::Custom(0x02)); // Error: Not fundable or not fully funded
        }

        // Logic for transferring funds to the owner (consider using SPL tokens or lamports)
        // ...

        // Update the campaign as closed or update the relevant data fields
        campaign_account_data.campaign_fulfilled = 0; // Or other status updates
        campaign_account_data.serialize(&mut &mut campaign_account.data.borrow_mut()[..])?;
    }

    else {
        return Err(ProgramError::Custom(0x03)); // Error: Invalid instruction
    }

    Ok(())
}
