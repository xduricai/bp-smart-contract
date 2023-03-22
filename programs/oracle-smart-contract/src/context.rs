use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = crate::state::MAX_STATE_SIZE)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
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
    #[account(mut)]
    pub state: Account<'info, State>,
    pub address: Signer<'info>,
}

#[derive(Accounts)]
pub struct ReportData<'info> {
    #[account(mut)]
    pub state: Account<'info, State>
}