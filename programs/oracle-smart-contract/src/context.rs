use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = crate::state::MAX_STATE_SIZE)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: account constraints checked in account trait
    #[account(address = sysvar::slot_hashes::id())]
    pub recent_slothashes: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct AddSubscription<'info> {
    #[account(init, payer = client, space = crate::state::MAX_SUBSCRIPTION_SIZE)]
    pub subscription: Account<'info, Subscription>,
    #[account(mut)]
    pub client: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddOracle<'info> {
    #[account(init, payer = payer, space = crate::state::ORACLE_ACC_SIZE)]
    pub oracle: Account<'info, OracleAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub state: Account<'info, State>,
    pub address: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: account constraints checked in account trait
    #[account(address = sysvar::slot_hashes::id())]
    pub recent_slothashes: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct ReportData<'info> {
    #[account(mut)]
    pub subscription: Account<'info, Subscription>
}

#[derive(Accounts)]
pub struct EndRound<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    /// CHECK: account constraints checked in account trait
    #[account(address = sysvar::slot_hashes::id())]
    pub recent_slothashes: UncheckedAccount<'info>,
}