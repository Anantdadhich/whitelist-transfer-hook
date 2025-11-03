use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, 
    seeds::Seed, 
    state::ExtraAccountMetaList
};



//it will create an account to store the metadata 
//token 2022 uses this metadata account what account to pass calling the transfer hook \
//withou this transfer hook will fail becauese token 2022 dont knbow which accounts hook you needs

#[derive(Accounts)]
pub struct InitextraAccountMetaList<'info>{
    #[account(mut)]
   pub payer:Signer<'info> ,
///CHECK: ExtraAccount Metalist account

     #[account(
        init,
        seeds=[b"extra-account-metas",mint.key().as_ref()],
        bump,
        space=ExtraAccountMetaList::size_of(
            InitextraAccountMetaList::extra_account_metas()?.len()
        )?,
        payer=payer
     )]

    pub extra_account_meta_list:AccountInfo<'info>,

    pub mint:InterfaceAccount<'info,Mint>,

    pub system_program:Program<'info,System>
     

}

impl <'info> InitextraAccountMetaList <'info> {
    pub fn extra_account_metas()->Result<Vec<ExtraAccountMeta>>{
        Ok(
            vec![
                ExtraAccountMeta::new_with_seeds(
                   &[
                    Seed::Literal { bytes:
                    b"whitelist".to_vec(),
                     }
                   ],
                   false,
                   false

                )?
            ]
        )
    }
}