use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use serde_json::{Result,Value};
//use serde::{Deserialize,Serialize};

//use std::str;
mod instruction;
use crate::instruction::HelloInstruction;
use core::convert::From;
//use rand::Rng;
//use byteorder::{BigEndian, ReadBytesExt};
//extern crate rand;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings//#[derive(Serialize, Deserialize)]

    pub name: u32,
    pub age: u32,
    //pub sum: u32,
}
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Instructiondata {
    // Note that the key does not have to be "message" like the argument name.
    val1: u32,
    val2: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

fn lottery_number_gen()-> u32
{
   // let lottery_num: u32 =rng.gen(); 
return 10;
}
// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");
  // let keyReceived = String::from_utf8_lossy(&_instruction_data);
  //let data = _instruction_data
  //.split_first();
  // let data: GreetingAccount  = serde_json::from_str(&keyReceived).unwrap();
  //  let instruction = HelloInstruction::unpack_amount(_instruction_data);
    //let mut data = GreetingAccount::try_from_slice(&_instruction_data)?;

    //let mut data = try_from_slice(&_instruction_data)?;
    //let data = r#" { "name": "John Doe", "age": 43, ... } "#;
   // let v: Value = serde_json::from_str(data)?;
    //let x = keyReceived.split(',');
  //  msg!("Instruction data => {:?}", keyReceived);

   let message = GreetingAccount::try_from_slice(_instruction_data).unwrap();
    // .map_err(|err|
    // {msg!("Receiving message as string failed {:?}",err);
    // ProgramError::InvalidInstructionData;
    // });
let instdata = message;
   //Iterating accounts is safer then indexing
   let accounts_iter = &mut accounts.iter();
    
    // Get msg!the account to say hello to
    let account = next_account_info(accounts_iter)?;
    
    // let data = Instructiondata {
    //   val1:0,
    //   val2:0,
    // };
   //data = data[.._instruction_data.len()].copy_from_slice(&_instruction_data);
    msg!("Instruction data message ==> {:?}", instdata);
   for i in 0..accounts.len() 
   {
    msg!("Account no. {} info: {:?}",i+1, accounts[i]);
   } 
  

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
 }

   //let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;

  // let encoded_a = _instruction_data.try_to_vec().unwrap();
  //  let decoded_a = GreetingAccount::try_from_slice(&encoded_a).unwrap();
   
   //msg!("instrcution data {:?}",decoded_a);
  // let number : u32 = From::from(_instruction_data[0]);
   //let number2 :u32 = From::from(_instruction_data[1]);
  //  greeting_account.input_a= number;
  //  greeting_account.input_b =number2;

  //  greeting_account.sum = sum(number, number2);
  
   //greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

   // msg!("Greeted {} time(s)!", greeting_account.input_a);

    Ok(())
  }


  pub fn sum(num1:u32,num2:u32)-> u32
  {
    return num1 + num2;
  }
// Sanity tests
// #[cfg(test)]
// mod test {
//     use super::*;
//     use solana_program::clock::Epoch;
//     use std::mem;

//     #[test]
//     fn test_sanity() {
//         let program_id = Pubkey::default();
//         let key = Pubkey::default();
//         let mut lamports = 0;fn pop(barry: &[u8]) -> [u8; 3] {
     // expected array `[u8; 3]`, found slice `[u8]`

//         let mut data = vec![0; mem::size_of::<u32>()];
//         let owner = Pubkey::default();
//         let account = AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default(),
//         );
//         let instruction_data: Vec<u8> = Vec::new();

//         let accounts = vec![account];

//         // assert_eq!(
//         //     GreetingAccount::try_from_slice(&accounts[0].data.borrow())
//         //         .unwrap()
//         //         .counter,
//         //     0
//         // );
//         // process_instruction(&program_id, &accounts, &instruction_data).unwrap();
//         // assert_eq!(
//         //     GreetingAccount::try_from_slice(&accounts[0].data.borrow())
//         //         .unwrap()
//         //         .counter,
//         //     1
//         // );
//         // process_instruction(&program_id, &accounts, &instruction_data).unwrap();
//         // assert_eq!(
//         //     GreetingAccount::try_from_slice(&accounts[0].data.borrow())
//         //         .unwrap()
//         //         .counter,
//         //     2
//         // );
//     }
// }
