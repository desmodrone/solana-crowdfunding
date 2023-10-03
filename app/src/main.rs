use solana_program::{
    account_info::{next_acount_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramErorr,
    pubkey::Pubkey
    rent::Rent,
    sysvar::Sysvar,
}

    pub struct CampaignAccount {
        pub campaign_owner: Pubkey,
        pub campaign_amount: u64,
        pub campaign_description: String,
        pub campaign_fulfilled: u64,
    }

    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8],
    ) -> ProgramResult {
        let (instruction_byte, all_other_bytes) = data.split_first().unwrap();

        if *instruction_byte == 0 {
            //create campaign
        } else if *instruction_byte == 1 {
            //fund campaing
        } else if *instruction_byte == 2 {
            // get how many funds are left to reach the requested amount
        } else if *instruction_byte == 3 {
            // withdraw all the collected funds and close campaign
        }
    }

    let iterable_accounts = &mut accounts.iter();
    let campaign_account = next_account_info(iterable_accounts);

    let amout = rest_of_data
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .unwrap();
