/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */

import {
  Keypair,
  Connection,
  PublicKey,
  LAMPORTS_PER_SOL,
  SystemProgram,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import fs from 'mz/fs';
import path from 'path';
import * as borsh from 'borsh';

import {getPayer, getRpcUrl, createKeypairFromFile} from './utils';
import * as BufferLayout from '@solana/buffer-layout';
import { Buffer } from 'buffer';
import * as readline from 'readline';
/**
 * Connection to the networkx
 */
let connection: Connection;

/**
 * Keypair associated to the fees' payer
 */
let payer: Keypair;

/**
 * Hello world's program id
 */
let programId: PublicKey;

/**
 * The public key of the account we are saying hello to
 */
let greetedPubkey: PublicKey;

/**
 * The public key of the account we are saying hello to
 */
 let secondPubkey: PublicKey;
/**
 * Path to program files
 */
const PROGRAM_PATH = path.resolve(__dirname, '../../dist/program');

/**
 * Path to program shared object file which should be deployed on chain.
 * This file is created when running either:
 *   - `npm run build:program-c`
 *   - `npm run build:program-rust`
 */
const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'helloworld.so');

/**
 * Path to the keypair of the deployed program.
 * This file is created when running `solana program deploy dist/program/helloworld.so`
 */
const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, 'helloworld-keypair.json');

/**
 * The state of a greeting account managed by the hello world program
 */
class GreetingAccount {
  inputA = 0;
  inputB = 3;
 // sum=0;
  constructor(fields: {inputA: number, inputB: number} | undefined = undefined) {
    if (fields) {
      this.inputA = fields.inputA;
      this.inputB = fields.inputB;
     // this.sum = fields.sum;
    }
  }
}

/**
 * The state of a greeting account managed by the hello world program
 */
 class SecondAccount {
  customdata1 = 0;
  customdata2 = 0;
  
  constructor(fields: {customdata1: number, customdata2: number} | undefined = undefined) {
    if (fields) {
      this.customdata1 = fields.customdata1;
      this.customdata2 = fields.customdata2;
      
    }
  }
}

/**
 * Borsh schema definition for greeting accounts
 */
const SecondAccountSchema = new Map([
  [SecondAccount, {kind: 'struct', fields: [['customdata1', 'u32'], ['customdata2', 'u32'] ]}],
]);

/**
 * The expected size of each greeting account.
 */
const SECOND_ACCOUNT_SIZE = borsh.serialize(
  SecondAccountSchema,
  new SecondAccount(),
).length;

/**
 * Borsh schema definition for greeting accounts
 */
 const GreetingSchema = new Map([
  [GreetingAccount, {kind: 'struct', fields: [['inputA', 'u32'], ['inputB', 'u32'] ]}],
]);

/**
 * The expected size of each greeting account.
 */
const instructiondata= new GreetingAccount();
instructiondata.inputA=17;
instructiondata.inputB=9999;
const GREETING_SIZE = borsh.serialize(
  GreetingSchema,
  instructiondata,
).length;

/**
 * Establish a connection to the cluster
 */
export async function establishConnection(): Promise<void> {
  const rpcUrl = await getRpcUrl();
  connection = new Connection(rpcUrl, 'confirmed');
  const version = await connection.getVersion();
  console.log('Connection to cluster established:', rpcUrl, version);
}

/**
 * Establish an account to pay for everything
 */
export async function establishPayer(): Promise<void> {
  let fees = 0;
  if (!payer) {
    const {feeCalculator} = await connection.getRecentBlockhash();

    // Calculate the cost to fund the greeter account
    fees += await connection.getMinimumBalanceForRentExemption(GREETING_SIZE);

    // Calculate the cost of sending transactions
    fees += feeCalculator.lamportsPerSignature * 100; // wag

    payer = await getPayer();
  }

  let lamports = await connection.getBalance(payer.publicKey);
  if (lamports < fees) {
    // If current balance is not enough to pay for fees, request an airdrop
    const sig = await connection.requestAirdrop(
      payer.publicKey,
      fees - lamports,
    );
    await connection.confirmTransaction(sig);
    lamports = await connection.getBalance(payer.publicKey);
  }

  console.log(
    'Using account',
    payer.publicKey.toBase58(),
    'containing',
    lamports / LAMPORTS_PER_SOL,
    'SOL to pay for fees',
  );
}
export async function createAccount(seed:any,size:any,lamports:any)
{
  const transaction = new Transaction().add(
    SystemProgram.createAccountWithSeed({
      fromPubkey: payer.publicKey,
      basePubkey: payer.publicKey,
      seed: seed,
      newAccountPubkey: greetedPubkey,
      lamports,
      space: size,
      programId,
    }),
  );
  await sendAndConfirmTransaction(connection, transaction, [payer]);

};


/**
 * Check if the hello world BPF program has been deployed
 */
export async function checkProgram(): Promise<void> {
  // Read program id from keypair file
  try {
    const programKeypair = await createKeypairFromFile(PROGRAM_KEYPAIR_PATH);

    programId = programKeypair.publicKey;
  } catch (err) {
    const errMsg = (err as Error).message;
    throw new Error(
      `Failed to read program keypair at '${PROGRAM_KEYPAIR_PATH}' due to error: ${errMsg}. Program may need to be deployed with \`solana program deploy dist/program/helloworld.so\``,
    );
  }

  // Check if the program has been deployed
  const programInfo = await connection.getAccountInfo(programId);
  if (programInfo === null) {
    if (fs.existsSync(PROGRAM_SO_PATH)) {
      throw new Error(
        'Program needs to be deployed with `solana program deploy dist/program/helloworld.so`',
      );
    } else {
      throw new Error('Program needs to be built and deployed');
    }
  } else if (!programInfo.executable) {
    throw new Error(`Program is not executable`);
  }
  console.log(`Using program ${programId.toBase58()}`);

  // Derive the address (public key) of a greeting account from the program so that it's easy to find later.
  const GREETING_SEED = ' two number sum';
  const SECOND_ACCOUNT_SEED = "Testing Account iterator"
  greetedPubkey = await PublicKey.createWithSeed(
    payer.publicKey,
    GREETING_SEED,
    programId,
  );

  secondPubkey = await PublicKey.createWithSeed(
    payer.publicKey,
    SECOND_ACCOUNT_SEED,
    programId,
  );

console.log('second pubkey',secondPubkey)
  // Check if the greeting account has already been created
  const greetedAccount = await connection.getAccountInfo(greetedPubkey);
  if (greetedAccount === null) {
    console.log(
      'Creating account',
      greetedPubkey.toBase58(),
      'to say hello to',
    );
    const lamports = await connection.getMinimumBalanceForRentExemption(
      GREETING_SIZE,
    );
      createAccount(GREETING_SEED,GREETING_SIZE,lamports);
    
    
  }

  // Check if the greeting account has already been created
  const secondAccount = await connection.getAccountInfo(secondPubkey);
  if (secondAccount === null) {
    console.log(
      'Creating second account',
      secondPubkey.toBase58(),
      'to say hello to',
    );
    const lamports = await connection.getMinimumBalanceForRentExemption(
      SECOND_ACCOUNT_SIZE,
    );

    createAccount(SECOND_ACCOUNT_SEED,SECOND_ACCOUNT_SIZE,lamports);
  }
}

function createInstructionData(): Buffer {

  // const jsonObject = {
  //   "A":'29',
  //   "B":'28',
  //   "C":'IT'
  // }
  const msg = '{"name":"John", "age":"22"}';
var jsonObj = JSON.parse(msg);
 
// convert JSON object to String
var jsonStr = JSON.stringify(jsonObj);
 
// read json string to Buffer
const buf = Buffer.from(jsonStr);
 
// console.log(buf.length);

  const dataLayout = BufferLayout.struct([
    BufferLayout.u8('A'),BufferLayout.u8('B')
  ],
  );

  const data = Buffer.alloc(dataLayout.span);
  dataLayout.encode({
    A: 23,
    B: 24
  }, data);
console.log('data:------->', data )
console.log('buffer data: ', jsonObj);
  return data ;
}

/**
 * Say hello
 */
export async function sayHello(): Promise<void> {



// console.log('Saying hello to', greetedPubkey.toBase58());
// const instruction = new TransactionInstruction({
//   keys: [{pubkey: greetedPubkey, isSigner: false, isWritable: true}],
//   programId,
//   data: Buffer.from(JSON.stringify({a: 25, b: 36, res: 0}) ), // All instructions are hellos
// });
  
const instruction = new TransactionInstruction({
    keys: [{pubkey: greetedPubkey, isSigner: false, isWritable: true},{pubkey: secondPubkey, isSigner: false, isWritable: true}],
    programId,
    data: Buffer.from(borsh.serialize(
      GreetingSchema,
      instructiondata,
    )), 
  });

  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payer],
  );
}

/**
 * Report the number of times the greeted account has been said hello to
 */
export async function reportGreetings(): Promise<void> {
  const accountInfo = await connection.getAccountInfo(greetedPubkey);
 
  if (accountInfo === null) {
    throw 'Error: cannot find the greeted account';
  }
  // const greeting = borsh.deserialize(
  //   GreetingSchema,
  //   GreetingAccount,
  //   accountInfo.data,
  // );
 
  //console.log("accountinfo", greeting);
  console.log(
    //greetedPubkey.toBase58(),
    //'has been greeted',
    'the sum of the two numbers is =>',
    //greeting.sum,
    //'time(s)',
  );
}
