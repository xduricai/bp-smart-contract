use anchor_lang::prelude::*;

mod context;
mod state;
mod errors;

use crate::context::*;
use crate::state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

//TODO uncomment before compile
//#[program]
pub mod oracle_smart_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.state.start()?;
        Ok(())
    }

    pub fn add_subscription(ctx: Context<AddSubscription>, input: SubscriptionInput) -> Result<()> {
        let subscription: &mut Account<Subscription> = &mut ctx.accounts.subscription;
        
        if input.api.chars().count() > crate::state::MAX_API_SIZE {
            return Err(crate::errors::MyError::ApiTooLarge.into())
        }

        if input.headers.chars().count() > crate::state::MAX_HEADERS_SIZE {
            return Err(crate::errors::MyError::HeadersTooLarge.into())
        }

        if input.params.chars().count() > crate::state::MAX_PARAMS_SIZE {
            return Err(crate::errors::MyError::ParamsTooLarge.into())
        }

        subscription.client = *ctx.accounts.client.key;
        subscription.recipient = input.recipient;
        subscription.length = input.length;
        subscription.api = input.api;
        subscription.headers = input.headers;
        subscription.params = input.params;

        Ok(())
    }

    pub fn add_oracle(ctx: Context<AddOracle>, stake: u32) {
        ctx.accounts.state.add_oracle(stake, *ctx.accounts.address.key);
    }

    pub fn report_data(ctx: Context<ReportData>, data: Vec<DataInput>) {
        ctx.accounts.state.process_data(data);
    }
} 