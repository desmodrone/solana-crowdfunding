use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

// telling solana to enter our program on `process_instruction`
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
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
    // msg! prints out as a log
    msg!("Hello Solana (From the Rust side!)");
    Ok(())
}
