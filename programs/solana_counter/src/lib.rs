//STEP 1: Add the solana prelude
use anchor_lang::prelude::*;

//STEP 2 : Add the public module and the program trait
#[program]
pub mod solana_counter {

    //STEP 3 : Add a initialse funtion
    //STEP 5: Add Initialise as context
    pub fn initialise(ctx: Context<Initialise>) -> Result<()> {
        //STEP7 : Read User from context
        let user = &ctx.accounts.user;
        Ok(())
    }
}

// STEP 4: Create the accounts
#[derive(Accounts)]
pub struct Initialise<'info> {
    pub user: Signer<'info>,
    //STEP 8: Add payer and user
    #[account(init,payer = user , space = 8+ 8)]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Counter {
    pub count: u64,
}
