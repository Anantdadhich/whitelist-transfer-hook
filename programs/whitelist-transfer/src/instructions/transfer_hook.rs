use core::panic;
use std::cell::RefMut;

use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{
            transfer_hook::TransferHookAccount, 
            BaseStateWithExtensionsMut, 
            PodStateWithExtensionsMut
        }, 
        pod::PodAccount
    }, 
    token_interface::{
        Mint, 
        TokenAccount
    }
};

use crate::state::*;


#[derive(Accounts)]
pub struct TransferHook<'info> {
     /// CHECK: Unchecked source account owner 
    pub owner:UncheckedAccount<'info>,

    pub mint:InterfaceAccount<'info,Mint>,
      #[account(
        token::mint=mint,
        token::authority=owner
      )]
    pub source_token:InterfaceAccount<'info,TokenAccount>,
      #[account(
        token::mint=mint
      )]
    pub destination_token:InterfaceAccount<'info,TokenAccount>,
    #[account(
        seeds=[b"whitelist"],
        bump=whitelist.bump
    )]
     pub whitelist:Account<'info,Whitelist>, 

     /// CHECK: Extraaccountmeta list account
     #[account(
      seeds=[b"extra-account-metas",mint.key().as_ref()],
      bump
     )]
    pub extra_account_meta_list:UncheckedAccount<'info>
}

impl<'info> TransferHook<'info> {
      pub fn transfer_hook(&mut self,_amount:u64)->Result<()>{
         self.check_is_transferring()?;

         if !self.whitelist.address.contains(self.owner.key){
            panic!("Transferhook : Owner is not whitlisted");
         };

    Ok(())
      }

      pub fn check_is_transferring(&mut self)->Result<()>{
           let source_token=self.source_token.to_account_info();

           let mut account_data_ref:RefMut<&mut [u8]>=source_token.try_borrow_mut_data()?;
           
            
       let mut account=PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref)?;

       let account_extensions=account.get_extension_mut::<TransferHookAccount>()?;


        if !bool::from(account_extensions.transferring){
            panic!("Transfer not tranfer ");
        }  
       
        Ok(())
      }
}