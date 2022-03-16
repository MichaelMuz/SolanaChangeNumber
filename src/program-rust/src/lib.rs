use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
pub mod instruction;
use crate::instruction::HelloInstruction;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {

    let instructions = HelloInstruction::unpack(instruction_data)?;


    msg!("Hello World Rust program entrypoint");

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of times the account has been greeted
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    
    match instructions{
        HelloInstruction::Increment => greeting_account.counter += 1,
        HelloInstruction::Decrement => greeting_account.counter -= 1,
        HelloInstruction::Set(x) => greeting_account.counter = x,
    }

    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Greeted {} time(s)!", greeting_account.counter);

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        //0 - increment
        //1 - decrement
        //2 - set
        //  1-4 -> u32 little endian
        // [2, 100, 0, 0, 0]

        //this will return an array of u8 of length 2
        let arr = u32::to_le_bytes(100);
        //this will create an array of length 5 with each value set to 2
        let mut instruction_data = [2; 5];
        //we keep the first one as 2, the set instruction, the rest is the 100 for what to set it to
        for i in 1..5{
            instruction_data[i] = arr[i - 1];
        }
        //^[2, 100, 0, 0, 0]


        

        let accounts = vec![account];

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            100
        );
        //here we set the instrucctions to be all 0 and now it is an increment instruction
        let instruction_data = [0; 5];
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            101
        );
    }
    #[test]
    #[should_panic]
    fn test_sub_from_zero() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        //0 - increment
        //1 - decrement
        //2 - set
        //  1-4 -> u32 little endian
        // [2, 100, 0, 0, 0]

        //this will return an array of u8 of length 2
        let arr = u32::to_le_bytes(100);
        //this will create an array of length 5 with each value set to 2
        let mut instruction_data = [1; 5];
        //we keep the first one as 2, the set instruction, the rest is the 100 for what to set it to
        for i in 1..5{
            instruction_data[i] = arr[i - 1];
        }
        //^[2, 100, 0, 0, 0]


        

        let accounts = vec![account];

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            100
        );
        //here we set the instrucctions to be all 0 and now it is an increment instruction
        let instruction_data = [0; 5];
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            101
        );
    }
}
