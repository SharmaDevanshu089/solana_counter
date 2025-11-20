//STEP 1: Add the solana prelude
use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// STEP 4: Create the accounts
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    //STEP 8: Add payer and user
    #[account(init,payer = user , space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Counter {
    pub count: u64,
}

//STEP 2 : Add the public module and the program trait
#[program]
pub mod solana_counter {
    use super::*;
    //STEP 3 : Add a initialse funtion
    //STEP 5: Add Initialise as context
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        //STEP7 : Read User from context
        let user = &ctx.accounts.user;
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        Ok(())
    }
}
