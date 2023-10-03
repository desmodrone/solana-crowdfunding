use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

// telling solana to enter our program on `process_instruction`
entrypoint!(process_instruction);

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
    // Implement the logic for creating a campaign here.
    // Use the provided accounts and data as needed.
    // You can also call helper functions if necessary.
    // ...

    Ok(())
}
