use crate::ProgramError;
use borsh::{BorshDeserialize, BorshSerialize};
use std::convert::TryInto;

#[derive(Debug)]
#[derive(BorshSerialize, BorshDeserialize)]
pub enum HelloInstruction {
    SayHello,
    SayBye,
}

impl HelloInstruction {
       pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
            let (&tag, rest) = input
                .split_first()
                .ok_or(ProgramError::InvalidInstructionData)?;
            //msg!("tag data {}",&tag);
    
            Ok(match tag {
                0 => HelloInstruction::SayHello,
                1 => HelloInstruction::SayBye,
                _ => return Err(ProgramError::InvalidInstructionData),
            })
        }
   
        pub fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
            let amount = input
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(u64::from_le_bytes)
                .ok_or(ProgramError::InvalidInstructionData)?;
            Ok(amount)
        }
    }