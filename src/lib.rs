use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("contract_instantiated!");

    let accounts_iter = &mut accounts.iter();
    let caller_account = next_account_info(accounts_iter)?;

    msg!("incoming request from: {:?}", caller_account.key);
    
    let deserialized_data =
        Instruction::try_from_slice(instruction_data).expect("unable to desrialize instruction");

    msg!(
        "initiating destruction sequence for target {}",
        deserialized_data.payload
    );

    Ok(())
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Instruction {
    pub payload: String,
    pub created_at: u64,
}

#[cfg(test)]
mod test {
    use solana_program::clock::Epoch;
    use std::mem;

    use super::*;

    #[test]
    pub fn test_instantiated() {
        let key = Pubkey::new_unique();
        let owner = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];

        let program_id = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            true,
            Epoch::default(),
        );

        let accounts = vec![account];
        let instruction_data = Instruction {
            payload: "12.9716° N, 77.5946° E".to_string(),
            created_at: 32,
        };

        let mut serialzed_instruction = vec![];
        instruction_data
            .serialize(&mut serialzed_instruction)
            .unwrap();

        process_instruction(&program_id, &accounts, &serialzed_instruction).unwrap()
    }
}
