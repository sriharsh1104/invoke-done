pub mod processor;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    instruction::{AccountMeta, Instruction},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey, 
    program::invoke
};


mod instruction;


// use std::str;
// mod instruction;
// use crate::instruction::HelloInstruction;
use core::convert::From;
// use byteorder::{BigEndian, ReadBytesExt};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub input_a: u32,
    pub input_b: u32,
    pub sum: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);
solana_program::declare_id!("8XN4SLFgTwq48N8uq5p2GAG6kSCWKfLBvKNRuJNTjdVg");

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8],
     // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");
//     let keyReceived = String::from_utf8_lossy(_instruction_data);
//    let instruction = HelloInstruction::unpack(_instruction_data);



   for i in 0..accounts.len() {
    msg!("Account no. {} info: {:?}",i, accounts[i]);
}
    let number : u32 = From::from(_instruction_data[0]);
    let number2 :u32 = From::from(_instruction_data[1]);
   msg!("instruction data :=> {}", number);
   //Iterating accounts is safer then indexing
     let accounts_iter = &mut accounts.iter();
    
   msg!("instruction data {:?}", _instruction_data);
    // Get msg!the account to say hello to
     let account = next_account_info(accounts_iter)?;
    
    msg!("account {:?}",account.data );
    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
 }

 

   // Increment and store the number of times the account has been greeted
   let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
   
   greeting_account.sum = greeting_account.input_a + greeting_account.input_b;
   msg!("abc =================================================>{},{},{}", greeting_account.input_a, greeting_account.input_b,greeting_account.sum);
   let num1= _instruction_data[0];
   msg!("first number {}",num1);

   greeting_account.input_a =number;
   greeting_account.input_b =number2;
   greeting_account.sum = greeting_account.input_a + greeting_account.input_b;
   greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Greeted {} time(s)!", greeting_account.input_a);

    let acc1 = account.key;
    let account = next_account_info(accounts_iter)?;

    let acc2 = account.key; 
    let mut data: Vec<u8> = Vec::with_capacity(10);
  
    
    /*
    let p_id = solana_program::pubkey::Pubkey::FromStr("8XN4SLFgTwq48N8uq5p2GAG6kSCWKfLBvKNRuJNTjdVg");
    */

 create_invoke_instruction(&id() , acc1 , acc2 ,data) ;

    Ok(())
  }
  
  pub fn create_invoke_instruction(program_id: &Pubkey ,acc1: &Pubkey,acc2: &Pubkey,data:Vec<u8>) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*acc1, false),
        AccountMeta::new(*acc2, false),
    ];

    msg!("Invoking instruction");

    Ok(Instruction {
        program_id: *program_id,
       accounts,
        data,
    })
  }