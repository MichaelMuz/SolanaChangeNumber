use solana_program::{program_error::ProgramError};
use std::convert::TryInto;

#[derive(Debug)]
pub enum HelloInstruction{
    Increment,
    Decrement,
    Set(u32)
}

impl HelloInstruction{
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        //split_first() will seperate the first element form the rest and return both
        //ok_or() converts option to a result value goes to a OK type and none becomes an Err
        //we set the error type tp be invalid instruction data
        let(&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        
        match tag{
            0 => return Ok(HelloInstruction::Increment),
            1 => return Ok(HelloInstruction::Decrement),
            2 => {
                //lets be sure that the rest of the data is actually an array of size 4
                if rest.len() != 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                //try into will attempt to convert a slice into an array of size 4(in this case bc ; 4)so that size is known at compile time
                let val: Result<[u8 ; 4], _> = rest[..4].try_into();
                match val{
                    Ok(i) => {
                        //from_le_bytes() will convert 4 little endian bytes into a u32
                        return Ok(HelloInstruction::Set(u32::from_le_bytes(i)));
                    },
                    _ => return Err(ProgramError::InvalidInstructionData)
                }
            },
            _ => Err(ProgramError::InvalidInstructionData)
        }
    
    }
}