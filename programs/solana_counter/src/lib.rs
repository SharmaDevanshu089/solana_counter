use anchor_lang::prelude::*;

declare_id!("FnVAqKWhENmtGWnRowegRX8A4GSTYgsG15sEBdmnF3Xg");

#[program]
pub mod solana_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
