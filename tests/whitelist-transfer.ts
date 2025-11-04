import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { WhitelistTransfer } from "../target/types/whitelist_transfer";
import {ASSOCIATED_TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, createInitializeMintInstruction, createInitializeTransferHookInstruction, createMintToInstruction, createTransferCheckedWithTransferHookInstruction, ExtensionType, getAssociatedTokenAddressSync, getMintLen, TOKEN_2022_PROGRAM_ID} from "@solana/spl-token"
import { SendTransactionError, SystemProgram, Transaction, sendAndConfirmTransaction } from '@solana/web3.js';
describe("whitelist-transfer", () => {
  // Configure the client to use the local cluster.
   const provider=anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  
  const program = anchor.workspace.whitelistTransfer as Program<WhitelistTransfer>;
   
   const wallet=provider.wallet as anchor.Wallet;
   const mint2022=anchor.web3.Keypair.generate();
      
   const sourcetokenaccount=getAssociatedTokenAddressSync(
    mint2022.publicKey,
    wallet.publicKey,
    false,
    TOKEN_2022_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID
   );

   const recipent=anchor.web3.Keypair.generate();

   const destinationTokenAccount=getAssociatedTokenAddressSync(
    mint2022.publicKey,
    recipent.publicKey,
    false,
    TOKEN_2022_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID
   )    ;


   const [extraAccountMetalistpda]=anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("extra-account-metas"),mint2022.publicKey.toBuffer()],
    program.programId
   );


   const whitelist=anchor.web3.PublicKey.findProgramAddressSync(
[Buffer.from("whitelist")],
program.programId

   )[0];



  it("Init the whitelist!", async () => {
    const tx=await program.methods.initializeWhitelist().accountsPartial({
      admin:wallet.publicKey,
      whitelist,
      systemProgram:anchor.web3.SystemProgram.programId
    }).rpc();

    console.log("Whitelist init",whitelist.toBase58());
    console.log("Transaction signature",tx);
  
  });

  it("Add user to whitlelist",async()=>{
    const tx=await program.methods.addToWhitelist(provider.wallet.publicKey).accountsPartial({

      admin:wallet.publicKey,
      whitelist,
     
    }).rpc();

    console.log("User added to whitlist",provider.publicKey.toBase58());
    console.log("Transaction signature",tx);
  })

  it("remove user from whitelist",async ()=>{
    const tx=await program.methods.removeFromWhitelist(provider.publicKey).accountsPartial({
      admin:wallet.publicKey,
      whitelist
    }).rpc();

    console.log("user removed from whitelist",provider.publicKey.toBase58());
    console.log("Transaction signature",tx);
  })

  it("Create mint account with transfer hook extension",async ()=>{
    const extensions=[ExtensionType.TransferHook];
    const mintlen=getMintLen(extensions);
    const lamports=await provider.connection.getMinimumBalanceForRentExemption(mintlen);

   

    const transaction=new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey:wallet.publicKey,
        newAccountPubkey:mint2022.publicKey,
        space:mintlen,
        lamports:lamports,
        programId:TOKEN_2022_PROGRAM_ID
      }),
      createInitializeTransferHookInstruction(
        mint2022.publicKey,
        wallet.publicKey,
        program.programId,
        TOKEN_2022_PROGRAM_ID
      ),
      createInitializeMintInstruction(
        mint2022.publicKey,
        9,
        wallet.publicKey,
        null,
        TOKEN_2022_PROGRAM_ID
      )
    );

    const txsign=await sendAndConfirmTransaction(provider.connection,transaction,[wallet.payer,mint2022],{
      skipPreflight:true,
      commitment:"finalized"
    });
    
    const txdetails=await program.provider.connection.getTransaction(txsign,{
      maxSupportedTransactionVersion:0,
      commitment:"confirmed"
    })

    console.log("Transaction signature",txsign)

});
 

 it("Create Token accounts and mint tokens ",async()=>{
  const amount=100*10**9;

  const transaction=new Transaction().add(
    createAssociatedTokenAccountInstruction(
      wallet.publicKey,
      sourcetokenaccount,
      wallet.publicKey,
      mint2022.publicKey,
      TOKEN_2022_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    ),

    createAssociatedTokenAccountInstruction(
      wallet.publicKey,
      destinationTokenAccount,
      recipent.publicKey,
      mint2022.publicKey,
      TOKEN_2022_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    ),
    createMintToInstruction(
      mint2022.publicKey,
      sourcetokenaccount,
      wallet.publicKey,
      amount,
      [],
      TOKEN_2022_PROGRAM_ID
    ),
  );

  const txsign=await sendAndConfirmTransaction(provider.connection,transaction,[wallet.payer],{
    skipPreflight:true
  })

  console.log("Tra sig",txsign);
 });

 it("Create Extra Account Metalist",async()=>{
  const initaliszeExtraAccountMetalistinst=await program.methods.initializeTransferHook().accountsPartial({
    payer:wallet.publicKey,
    mint:mint2022.publicKey,
    extraAccountMetaList:extraAccountMetalistpda,
    systemProgram:SystemProgram.programId,
  }).instruction();

  const transaction=new Transaction().add(
    initaliszeExtraAccountMetalistinst,
  );

  const txsig=await sendAndConfirmTransaction(provider.connection,transaction,[wallet.payer],{
    skipPreflight:true,
    commitment:"confirmed"
  })
    console.log("Tra sig",txsig);
 })
it("Re-add users to whitelist before transfer", async() => {
  // Add sender back
  const tx1 = await program.methods
    .addToWhitelist(wallet.publicKey)
    .accountsPartial({
      admin: wallet.publicKey,
      whitelist,
      systemProgram: anchor.web3.SystemProgram.programId
    }).rpc();
        console.log("Transaction signature",tx1);
  
  // Add recipient
  const tx2 = await program.methods
    .addToWhitelist(recipent.publicKey)
    .accountsPartial({
      admin: wallet.publicKey,
      whitelist,
      systemProgram: anchor.web3.SystemProgram.programId
    }).rpc();
  
  console.log("Wallet and recipient added to whitelist",tx2);
});

 it("Transfer hook" ,async()=>{
  const amount=1*10**9;
  const amountBigInt=BigInt(amount);

  const transferInstructionwithhelper=await createTransferCheckedWithTransferHookInstruction(
    provider.connection,
    sourcetokenaccount,
    mint2022.publicKey,
    destinationTokenAccount,
    wallet.publicKey,
    amountBigInt,
    9,
    [],
    "confirmed",
    TOKEN_2022_PROGRAM_ID
  );

  const transaction=new Transaction().add(
    transferInstructionwithhelper
  );

  try {
    const txsig=await sendAndConfirmTransaction(provider.connection,transaction,[wallet.payer],{
      skipPreflight:false,
    })
    console.log("Tra sig",txsig)
  } catch (error) {
       if (error instanceof SendTransactionError){
        console.log("Transaction failed ",error.logs[4]);
       }else{
        console.log("unkown error ",error)
       }
  }
 });

});