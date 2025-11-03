
#![allow(deprecated)]
#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

pub mod state;
use state::*;
pub mod instructions;
use instructions::*;

use spl_discriminator::SplDiscriminate;
use spl_transfer_hook_interface::{
    instruction::{
        ExecuteInstruction, 
        InitializeExtraAccountMetaListInstruction
    },
};
use spl_tlv_account_resolution::state::ExtraAccountMetaList;
declare_id!("Eya9GTKqj9JF3B3U2sitKEa419dSyBhZ4qJAoCtGx2gX");

#[program]
pub mod whitelist_transfer {
    use super::*;

  pub fn initialize_whitelist(ctx:Context<InitWhitelist>)->Result<()>{
    ctx.accounts.initalize_whitelist(ctx.bumps)
  }

  pub fn add_to_whitelist(ctx:Context<WhitelistOperation>,user:Pubkey)->Result<()>{
    ctx.accounts.add_to_whitelist(user)
  }

  pub fn remove_from_whitelist(ctx:Context<WhitelistOperation>,user:Pubkey)->Result<()>{
    ctx.accounts.remove_from_whitelist(user)
  }

  #[instruction(discriminator=InitializeExtraAccountMetaListInstruction::SPL_DISCRIMINATOR_SLICE)]
  pub fn initialize_transfer_hook(ctx:Context<InitextraAccountMetaList>)->Result<()>{
    msg!("Init transfer hook");

    let extra_account_metas=InitextraAccountMetaList::extra_account_metas()?;

    msg!("Extra Account Metas {:?}",extra_account_metas);

    msg!("extra account metas length{}",extra_account_metas.len());

    ExtraAccountMetaList::init::<ExecuteInstruction>(
        &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
        &extra_account_metas
    )?;
    Ok(())


  }
  #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        // Call the transfer hook logic
        ctx.accounts.transfer_hook(amount)
    }
        
}


