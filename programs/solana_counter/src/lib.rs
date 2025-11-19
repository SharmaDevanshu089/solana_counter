//STEP 1: Add the solana prelude
use anchor_lang::prelude::*;

//STEP 2 : Add the public module and the program trait
#[program]
pub mod solana_counter {
    //STEP 3 : Add a initialse funtion
    pub fn initialise() -> Result<()> {
        Ok(())
    }
}

// STEP 4: Create the accounts
#[derive(Accounts)]
pub struct Initialise {}
