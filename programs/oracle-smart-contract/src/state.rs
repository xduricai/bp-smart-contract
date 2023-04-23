use anchor_lang::prelude::*;
use arrayref::array_ref;
use std::convert::TryFrom;
use crate::errors::*;

pub const MAX_ORACLES: usize = 10;
pub const ORACLE_SIZE: usize = 52;
pub const ORACLE_ACC_SIZE: usize = 4;
pub const MAX_STATE_SIZE: usize = (ORACLE_SIZE * MAX_ORACLES) + 30;

pub const MAX_OPTIONS_SIZE: usize = 900;
pub const MAX_SUBSCRIPTION_SIZE: usize = MAX_OPTIONS_SIZE + 150;

#[account]
pub struct State {
    pub oracles: [Oracle; MAX_ORACLES],
    pub leader: u8,
    pub oracle_count: u8,
    pub initialized: bool,
    pub seed: u64 //TODO remove
}

impl State {
    pub fn initialize(&mut self, seed: u64) -> Result<()> {
        self.seed = seed;
        
        if self.oracle_count > 0 {
            self.initialized = true;
            self.select_leader(seed).unwrap();
        }/* TODO remove, only used for testing
        else {
            self.oracles[0].total_stake = 40000;
            self.oracles[1].total_stake = 80000;
            self.oracles[2].total_stake = 40;
            self.oracle_count = 3;

            self.select_leader(seed).unwrap();
        }*/

        Ok(())
    }

    pub fn select_leader(&mut self, seed: u64) -> Result<()> {
        if self.oracle_count == 0 {
            self.initialized = false;
            return Err(MyError::NoOracles.into())
        }

        let total_stake = self.oracles.iter().map(|oracle| oracle.total_stake).sum::<u64>();
        let random = seed % total_stake;
        let mut current_stake = 0;

        for (index, oracle) in self.oracles.iter().enumerate() {
            current_stake += oracle.total_stake;

            if current_stake > random {
                self.leader = index as u8;
                break;
            }
        }

        return Ok(())
    }

    pub fn next_cycle(&mut self, seed: u64) {
        self.select_leader(seed).unwrap();
    }

    pub fn add_oracle(&mut self, seed: u64, stake: u64, address: Pubkey) -> Result<()> {
        if self.oracle_count as usize >= MAX_ORACLES {
            return Err(MyError::OracleCapReached.into())
        }
        if stake == 0 {
            return Err(MyError::NoStakeDeposited.into())
        }
        else {
            let index = usize::try_from(self.oracle_count).unwrap();
            self.oracles[index].address = address;
            self.oracles[index].total_stake += stake;
            self.oracle_count += 1;

            if self.oracle_count == 1 {
                self.initialized = true;
                self.select_leader(seed).unwrap();
            }

            return Ok(())
        }
    }

    pub fn process_data(&mut self, _data: Vec<DataInput>) -> Result<()> {
        Ok(())
    }

    pub fn generate_seed(recent_slothashes: &UncheckedAccount) -> Result<u64> {
        let clock = Clock::get()?;
        let data = recent_slothashes.data.borrow();
        let most_recent = array_ref![data, 12, 8];
        Ok(u64::from_le_bytes(*most_recent).saturating_sub(clock.unix_timestamp as u64))
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy)]
pub struct Oracle {
    pub address: Pubkey,
    pub total_stake: u64,
    pub total_rewards: u64,
}

#[account]
pub struct OracleAccount { }

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