use anchor_lang::prelude::*;

declare_id!("7oZAwjFYYAohXSfpkNDrSZmEZi4TJxz5fRutmaB7Ss8H");

#[program]
pub mod anchor_hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, World!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
