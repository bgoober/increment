use anchor_lang::prelude::*;

declare_id!("GZRJsqKKeFFpznts1cXWKnC76jbKnB8fG3kRaJJbsTaU");

#[program]
pub mod increment {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
