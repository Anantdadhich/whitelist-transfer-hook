use anchor_lang::prelude::*;


use crate::state::Whitelist;

#[derive(Accounts)]
pub struct InitWhitelist<'info> {
    #[account(mut)]
   pub admin:Signer<'info> ,
     
    #[account(
        init,
        payer=admin,
        space=8+4+1,
        seeds=[b"whitelist"],
        bump 
        
    )]
   pub whitelist:Account<'info,Whitelist>,

   pub system_program:Program<'info,System>

}


impl<'info> InitWhitelist<'info> {
    pub fn initalize_whitelist(&mut self,bumps:InitWhitelistBumps) ->Result<()>{
               self.whitelist.set_inner(Whitelist { 
                
                address: vec![],

                 bump: bumps.whitelist });
       Ok(()) 
    }
}