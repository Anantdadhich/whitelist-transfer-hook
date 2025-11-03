use anchor_lang::{prelude::*, system_program};
use crate::state::*;

#[derive(Accounts)]
pub struct WhitelistOperation<'info> {
    #[account(mut)]
    pub admin:Signer<'info> ,
    #[account(
        mut,
        seeds=[b"whitelist"],
        bump
    )]

    pub whitelist:Account<'info,Whitelist>,

    pub system_program:Program<'info,System>,


}

impl <'info> WhitelistOperation<'info>{
          

          //check if account is alreadu whitelsit 
          //if not add whitlist accoount
         pub fn add_to_whitelist(&mut self, address: Pubkey) -> Result<()> {
              if !self.whitelist.address.contains(&address){
                self.realloc_whitelist(true)?;
                self.whitelist.address.push(address);
              }
        Ok(()) 
       }


       pub fn remove_from_whitelist(&mut self,address: Pubkey)->Result<()>{

           if let Some(pos)=self.whitelist.address.iter().position(|&x|x==address){
            self.whitelist.address.remove(pos);
            self.realloc_whitelist(false)?; 
           }
           Ok(())
       }

       pub fn realloc_whitelist(&mut self,is_adding:bool)->Result<()>{

           let account_info=self.whitelist.to_account_info();

           if is_adding{
            //current size + add space for new pubkey
            let new_account_size=account_info.data_len() + std::mem::size_of::<Pubkey>();
                          //get the current rent parameters from the solana runtime 
            let lamports_required=(Rent::get()?).minimum_balance(new_account_size);

              //we will calculate the how much sol neeed for the new size 
              //rrent dff if additional sol needed
            
            let rent_diff=lamports_required - account_info.lamports();

            let cpi_program=self.system_program.to_account_info();
            //we will transfer the sol from admin to whitlelist account  
            let cpi_accounts=system_program::Transfer{
                from:self.admin.to_account_info(),
                to:account_info.clone()
            };

            let cpi_ctx=CpiContext::new(cpi_program, cpi_accounts);

            system_program::transfer(cpi_ctx, rent_diff)?;

            account_info.realloc(new_account_size,false)?;

            msg!("Account size updated: {}",account_info.data_len()); 


        


                   }else {
                       let new_account_size=account_info.data_len() - std::mem::size_of::<Pubkey> ();

                       let lamports_required=(Rent::get()?).minimum_balance(new_account_size);

                       let rent_diff=account_info.lamports()-lamports_required;

                       account_info.realloc(new_account_size,false)?;

                       msg!("Account size downgraded: {}",account_info.data_len());
                       //This is manual lamport manipulation (low-level operation)

                       // double dereference to access the actual lamport value

                      // Add to admin's balance, subtract from whitelist's balance

                       // Admin gets refunded for the smaller account siz  

                          **self.admin.to_account_info().try_borrow_mut_lamports()? += rent_diff;
            **self.whitelist.to_account_info().try_borrow_mut_lamports()? -= rent_diff;
                   }

                   Ok(())
       }
}