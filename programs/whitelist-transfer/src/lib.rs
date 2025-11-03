
#![allow(deprecated)]
#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

pub mod state;
use state::*;
pub mod instructions;
use instructions::*;
declare_id!("Eya9GTKqj9JF3B3U2sitKEa419dSyBhZ4qJAoCtGx2gX");

#[program]
pub mod whitelist_transfer {
    use super::*;

  
}


