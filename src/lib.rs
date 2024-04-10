use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("contract_instantiated!");
    msg!("program_id {}", program_id);
    msg!("account_info {:?}", accounts);
    msg!("instruction_data {:?}", instruction_data);

    msg!("hello world!");

    Ok(())
}

#[cfg(test)]
mod test {
    use solana_program::clock::Epoch;
    use std::mem;

    use super::*;

    #[test]
    pub fn test_instantiated() {
        let key = Pubkey::default();
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
        let instruction_data = Vec::new();

        process_instruction(&program_id, &accounts, &instruction_data).unwrap()
    }
}
