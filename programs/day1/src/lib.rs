use anchor_lang::prelude::*;

declare_id!("CFCdbLq48XLGt9Ak2qhjUyanmfHYinkhvRJidgutktFJ");

#[program]
pub mod day1 {
    use super::*;

    pub fn initialize2(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello world: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
