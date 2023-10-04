use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

// telling solana to enter our program on `process_instruction`
entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CampaignAccount {
    pub campaign_owner: Pubkey,
    pub campaign_amount: u64,
    pub campaign_description: String,
    pub campaign_fulfilled: u64,
}

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (instruction_byte, rest_of_data) = instruction_data.split_first().unwrap();

    match *instruction_byte {
        0 => create_campaign(program_id, accounts, rest_of_data), // Call a separate function for creating a campaign.
        1 => fund_campaign(program_id, accounts, rest_of_data), // Call a separate function for funding a campaign.
        2 => get_funds_remaining(program_id, accounts), // Call a separate function to get remaining funds.
        3 => withdraw_and_close_campaign(program_id, accounts), // Call a separate function to withdraw funds and close.
        _ => {
            // Handle unknown instructions or errors.
            msg!("Unknown instruction received");
            Err(ProgramError::InvalidInstructionData)
        }
    }
}

fn create_campaign(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    rest_of_data: &[u8],
) -> ProgramResult {
    let iterable_accounts = &mut accounts.iter();
    let campaign_account = next_account_info(iterable_accounts);

    let amount = rest_of_data
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)?;
    Ok(())
}

fn create_campaign_account(
    campaign_account: &mut AccountInfo,
    amount: u64,
    description: String,
) -> ProgramResult {
    let mut campaign_account_data =
        CampaignAccount::try_from_slice(&campaign_account.data.borrow_mut())?;

    campaign_account_data.campaign_amount = amount;
    campaign_account_data.campaign_description = description;
    campaign_account_data.campaign_fulfilled = 0;

    // Serialize the modified data and write it back to the account.
    campaign_account_data.serialize(&mut &mut campaign_account.data.borrow_mut()[..])?;

    Ok(())
}

fn get_campaign_status(accounts: &[AccountInfo]) -> ProgramResult {
    // Assuming the first account in the accounts list is the campaign account.
    let campaign_account = next_account_info(accounts)?;

    let campaign_account_data = CampaignAccount::try_from_slice(&campaign_account.data.borrow())?;

    // Calculate the amount left to be collected.
    let amount_left =
        campaign_account_data.campaign_amount - campaign_account_data.campaign_fulfilled;

    // Print the amount left using msg.
    msg!("Amount left to be collected: {}", amount_left);

    Ok(())
}
