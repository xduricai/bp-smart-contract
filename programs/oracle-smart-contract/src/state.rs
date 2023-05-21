use anchor_lang::prelude::*;
use arrayref::array_ref;
use std::convert::TryFrom;
use crate::errors::*;

pub const MAX_ORACLES: usize = 10;
pub const ORACLE_SIZE: usize = 60;
pub const ORACLE_ACC_SIZE: usize = 20;
pub const MAX_STATE_SIZE: usize = (ORACLE_SIZE * MAX_ORACLES) + ORACLE_ACC_SIZE + 100;

pub const MAX_OPTIONS_SIZE: usize = 900;
pub const MAX_DATA_SIZE: usize = 8500;
pub const MAX_SUBSCRIPTION_SIZE: usize = MAX_OPTIONS_SIZE + MAX_DATA_SIZE + 150;

#[account]
pub struct State {
    pub oracles: [Oracle; MAX_ORACLES],
    pub leader: u8,
    pub leader_id: u8,
    pub oracle_count: u8,
    pub round_number: u64,
    pub initialized: bool,
}

impl State {
    pub fn initialize(&mut self, seed: u64) -> Result<()> {
        
        if self.oracle_count > 0 {
            self.round_number = 1;
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
                self.leader_id = oracle.id;
                break;
            }
        }

        return Ok(())
    }

    pub fn next_cycle(&mut self, seed: u64) {
        self.select_leader(seed).unwrap();
        self.round_number += 1;
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
            self.oracles[index].id = index as u8 + 1;
            self.oracles[index].address = address;
            self.oracles[index].total_stake += stake;
            self.oracle_count += 1;

            if self.oracle_count == 1 {
                self.initialized = true;
                self.round_number += 1;
                self.select_leader(seed).unwrap();
            }

            return Ok(())
        }
    }

    pub fn end_round(&mut self, accepted: bool) -> Result<()> {
        if accepted {
            let reward = self.oracles[self.leader_id as usize].total_stake as f64 * 1.2;
            let result = reward.floor() as u64;
            self.oracles[self.leader_id as usize].total_stake = result; 
        } else {
            let penalty = self.oracles[self.leader_id as usize].total_stake as f64 * 0.8;
            let result = penalty.floor() as u64;
            self.oracles[self.leader_id as usize].total_stake = result; 
        }
        
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
    pub id: u8,
    pub address: Pubkey,
    pub total_stake: u64,
    pub total_rewards: u64,
}

#[account]
pub struct OracleAccount { }

#[account]
pub struct Subscription {
    pub client: Pubkey,
    pub expiration: u64,
    pub options: String,
    pub data: String
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SubscriptionInput {
    pub duration: u64,
    pub options: String
}