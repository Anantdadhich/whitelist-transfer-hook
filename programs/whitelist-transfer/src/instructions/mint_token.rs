use anchor_lang::prelude::*;

use anchor_spl::token_interface::{
    Mint,
    TokenInterface
};

use crate::state::Whitelist;

#[derive(Accounts)]
pub struct TokenmMint<'info>{

    #[account(mut)]
    pub user:Signer<'info> ,
     #[account(
        init,
        payer=user,
        mint::decimals=9,
        mint::authority=user,
        extensions::transfer_hook::authority=user,
        extensions::transfer_hook::program_id=crate::ID, 

     )]
    pub mint:InterfaceAccount<'info,Mint>,
       ///CHECK :ExtraAccountMetaList account
    #[account(mut)]
    pub extra_account_meta_list:UncheckedAccount<'info>,

       #[account(
        seeds=[b"whitelist"],
        bump
       )]
    pub blocklist:Account<'info,Whitelist>,
    pub token_program:Interface<'info,TokenInterface>,
    pub system_program:Program<'info,System>
}

impl <'info> TokenmMint <'info> {
     pub fn init_mint(&mut self)->Result<()>{

        Ok(())
     }
}