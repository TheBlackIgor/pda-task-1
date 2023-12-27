use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("Fe1qa64RN7rRXV8DBFZB6y5UDrgqF9UwxtKFTQ52oNzF");

#[program]
pub mod pda_task_1 {
    use super::*;

    pub fn create_escrow(ctx: Context<CreateEscrow>, amount: u64) -> Result<()> {
        // Get escrow account
        let escrow_account = &mut ctx.accounts.escrow_account;
        // Set from
        escrow_account.from = ctx.accounts.from.key();
        // Set to
        escrow_account.to = ctx.accounts.to.key();
        // Set amount
        escrow_account.amount = amount;

        Ok(())
    }
}
// Create Escrow context
#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    // Escrow account PDA
    #[account(
        init,
        seeds = [b"escrow".as_ref(), from.key().as_ref(), to.key().as_ref()],
        bump,
         payer = from, 
         space = size_of::<EscrowAccount>()
    )]

    pub escrow_account: Account<'info, EscrowAccount>,

    #[account(mut)]
    pub from: AccountInfo<'info>,

    #[account(mut)]
    pub to: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    
}

// Escrow account structure
#[account]
pub struct EscrowAccount {
    // From
    pub from: Pubkey,
    // To
    pub to: Pubkey,
    // Amount
    pub amount: u64,
}
pub struct Initialize {}
