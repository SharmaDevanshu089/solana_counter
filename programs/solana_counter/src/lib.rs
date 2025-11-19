//STEP 1: Add the solana prelude
use anchor_lang::prelude::*;

//STEP 2 : Add the public module and the program trait
#[program]
pub mod solana_counter {

    //STEP 3 : Add a initialse funtion
    //STEP 5: Add Initialise as context
    pub fn initialise(ctx: Context<Initialise>) -> Result<()> {
        Ok(())
    }
}

// STEP 4: Create the accounts
#[derive(Accounts)]
pub struct Initialise {}
