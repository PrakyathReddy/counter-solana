use anchor_lang::prelude::*;

declare_id!("7JHcUJmYyN4Qto1i3SsYLY28f9QQcYB8SyxzniyQ32mc");

#[program]
pub mod counter_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
