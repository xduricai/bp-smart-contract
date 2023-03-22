use anchor_lang::prelude::*;
use std::convert::TryFrom;
use crate::errors::*;

pub const MAX_ORACLES: usize = 10;
pub const ORACLE_SIZE: usize = 44;
pub const MAX_STATE_SIZE: usize = (ORACLE_SIZE * MAX_ORACLES) + 20;

pub const MAX_OPTIONS_SIZE: usize = 900;
pub const MAX_SUBSCRIPTION_SIZE: usize = MAX_OPTIONS_SIZE + 150;

#[account]
pub struct State {
    pub oracles: [Oracle; MAX_ORACLES],
    pub leader: usize,
    pub oracle_count: usize,
    pub initialized: bool,
}

impl State {
    pub fn start(&mut self) -> Result<()> {
        if self.oracle_count > 0 {
            self.initialized = true;
        }
        Ok(())
    }

    pub fn select_leader(&mut self) -> Result<()> {
        if self.oracle_count == 0 {
            self.initialized = false;
            return Err(MyError::NoOracles.into())
        }

        self.leader += 1;
        self.leader %= self.oracle_count;

        return Ok(())
    }

    pub fn next_cycle(&mut self) {
        self.select_leader();
    }

    pub fn add_oracle(&mut self, stake: u32, address: Pubkey) -> Result<()> {
        if self.oracle_count >= MAX_ORACLES {
            return Err(MyError::OracleCapReached.into())
        }
        else {
            let index = usize::try_from(self.oracle_count).unwrap();
            self.oracles[index].address = address;
            self.oracles[index].total_stake += stake;
            self.oracle_count += 1;

            if self.oracle_count > 1 {
                self.initialized = true;
            }

            return Ok(())
        }
    }

    pub fn process_data(&mut self, data: Vec<DataInput>) -> Result<()> {
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy)]
pub struct Oracle {
    pub address: Pubkey,
    pub total_stake: u32,
    pub total_rewards: u32,
}

#[account]
pub struct Subscription {
    pub client: Pubkey,
    pub recipient: Pubkey,
    pub length: i64,
    pub options: String
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SubscriptionInput {
    pub recipient: Pubkey,
    pub length: i64,
    pub options: String
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DataInput {
    pub data_json: String,
    pub recipient: Pubkey
}