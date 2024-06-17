# Oracle Smart Contract  

Solana smart contract built using the [Anchor](https://www.anchor-lang.com/) framework. Serves as a proof of concept for [Internet of Value and DeFi in Solana](https://github.com/xduricai/bp-main). The smart contract oversees the data collection process. It determines which of the active oracles will be the leader of the current cycle based on a proof of stake mechanism. The contract also maintains the accounts for all users as well as active oracle nodes. Oracle accounts contain their stake and credentials while the user accounts are used to save subscription data.

## Prerequisites  

- [Solana](https://docs.solanalabs.com/cli/install)
- [Anchor](https://www.anchor-lang.com/docs/installation)
- [Rust](https://www.rust-lang.org/tools/install)

## Setup  

The smart contract is intended to be configured before moving on to the [Web Application](https://github.com/xduricai/bp-web-app) and the [Oracle Network](https://github.com/xduricai/bp-oracle-network)  

NOTE: The Solana Development Environment is only compatible with UNIX based operating systems and will require WSL to run on Windows

- open a terminal and configure solana with the following commands
```bash
solana config set -–url localhost  
solana-keygen new
solana-test-validator
```

- open `./bp-smart-contract` in a terminal and enter the following commands to build and deploy the smart contract on your local validator
```bash
anchor build
anchor deploy
```

- save the Program ID from your terminal as it will be needed later, alternatively it can also be retrieved with the following command `solana address -k target/deploy/oracle_smart_contract-keypair.json` 
- set the oracle_smart_contract field inside of `./bp-smart-contract/Anchor.toml` to the current Program ID
- set the input of declare_id! inside of `./bp-smart-contract/programs/oracle-smart-contract/lib.rs` to the current Program ID  

- build and deploy the application again using the following commands
```bash
anchor build
anchor deploy
```
- the initialization is now complete
