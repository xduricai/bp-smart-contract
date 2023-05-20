use anchor_lang::prelude::*;

mod context;
mod state;
mod errors;

use crate::context::*;
use crate::state::*;

//anchor keys list
declare_id!("5hNwnrnKRrKmy8bHyGAZy9psMZPJSPsaq4eriGSpN7N5");

//TODO uncomment before compile
#[program]
pub mod oracle_smart_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let seed = State::generate_seed(&ctx.accounts.recent_slothashes)?;
        ctx.accounts.state.initialize(seed)?;

        Ok(())
    }

    pub fn add_subscription(ctx: Context<AddSubscription>, input: SubscriptionInput) -> Result<()> {
        let subscription: &mut Account<Subscription> = &mut ctx.accounts.subscription;
        let round: u64 = ctx.accounts.state.round_number;
        
        if input.options.chars().count() > crate::state::MAX_OPTIONS_SIZE {
            return Err(crate::errors::MyError::OptionsTooLong.into())
        }

        subscription.client = *ctx.accounts.client.key;
        subscription.address = input.address;
        subscription.expiration = round + input.duration;
        subscription.options = input.options;

        Ok(())
    }

    pub fn add_oracle(ctx: Context<AddOracle>) -> Result<()> {
        let stake = ctx.accounts.oracle.to_account_info().lamports();
        let seed = State::generate_seed(&ctx.accounts.recent_slothashes)?;
        ctx.accounts.state.add_oracle(seed, stake, *ctx.accounts.address.key)
    }

    pub fn report_data(ctx: Context<ReportData>, data: String) -> Result<()> {
        let subscription: &mut Account<Subscription> = &mut ctx.accounts.subscription;
        subscription.data = data;

        Ok(())
    }
    
    pub fn end_round(ctx: Context<EndRound>, accepted: bool) -> Result<()> {
        ctx.accounts.state.end_round(accepted)?;
        let seed = State::generate_seed(&ctx.accounts.recent_slothashes)?;
        ctx.accounts.state.next_cycle(seed);
        Ok(())
    }
}