pub mod processor;
use borsh::{BorshDeserialize, BorshSerialize};
use std::str::FromStr;

// use solana_program::{pubkey, pubkey::Pubkey};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    instruction::{AccountMeta, Instruction},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey, pubkey::Pubkey, 
    program::invoke
};


mod instruction;

use instruction::initialize_sum_account;



use core::convert::From;






#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct instructiondata {
    /// number of greetings
    pub input_a: u32,
    pub input_b: u32,
    pub program_id: String,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);
solana_program::declare_id!("DyAFrm47pSvZbUd5Mv8BgXDnX9wWt2db5TPkk1rzin79");

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8],
     // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");
     // Increment and store the number of times the account has been greeted
   let mut greeting_account = instructiondata::try_from_slice(_instruction_data).unwrap();
   msg!("{:?}",greeting_account);
//     let keyReceived = String::from_utf8_lossy(_instruction_data);
//    let instruction = HelloInstruction::unpack(_instruction_data);

let my_id = Pubkey::from_str(&greeting_account.program_id).unwrap();
msg!("program_id => {:?}",my_id);

//    for i in 0..accounts.len() {
//     msg!("Account no. {} info: {:?}",i, accounts[i]);
// }
//    let (number, program2_id) = _instruction_data.split_at(2);


//     let p_id = String::from_utf8_lossy(program2_id);
//    msg!("instruction data :=> {:?}", _instruction_data);
   //Iterating accounts is safer then indexing
     let accounts_iter = &mut accounts.iter();
    
   //msg!("instruction data {:?}", number2);
    // Get msg!the account to say hello to
     let account1 = next_account_info(accounts_iter)?;

    // msg!("instruction data {}", p_id);
    // Get msg!the account to say hello to
    // let account = next_account_info(accounts_iter)?;
    
    msg!("account {:?}",account1.data );

    // msg!("Greeted {} time(s)!", greeting_account.input_a);

    let acc1 = account1.key;
    let account2 = next_account_info(accounts_iter)?;

    let acc2 = account2.key; 
    let mut data: Vec<u8> = Vec::with_capacity(10);
   
    let programaccount = next_account_info(accounts_iter)?;
    
    // let p_id = solana_program::pubkey::Pubkey::FromStr("G15b24WaZZrZEFLU2JVCpLXZ9fz3cCxrN8yRBgFwGqLM");
    

//Create_invoke_instruction(&id() , acc1 , acc2 ,data) ;
  let test = initialize_sum_account(
         &my_id , acc1 , acc2 
  );
msg!("sum account {:?}", test);
//     );


 invoke( &initialize_sum_account(
    &my_id,
    acc1,
    acc2,
   
)?, &[account1.clone(), account2.clone(), programaccount.clone()]);
      
 

    Ok(())
  }
