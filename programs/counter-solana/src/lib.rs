use anchor_lang::prelude::*;

declare_id!("ETQbUYXC6codoF79uGx4CmC3aLJ1BiSs6gLd8ZQFZa2C");

#[program]
pub mod counter_solana {
    use super::*;

    pub fn initialize_counter(_ctx: Context<InitializeCounter>) -> Result<()> {
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.counter.count += 1;
        msg!("Count is {}", ctx.accounts.counter.count);
        Ok(())
    }
}

#[account]
pub struct Counter {
    pub count: u64,
}

impl Counter {
    const LEN: usize = 8 + 8;
}

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(
        init,
        payer = payer,
        space = Counter::LEN,
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}
