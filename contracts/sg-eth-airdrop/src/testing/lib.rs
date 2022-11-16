#[path = "./constants/claim_constants.rs"]
mod claim_constants;
#[path = "./constants/collection_constants.rs"]
mod collection_constants;
#[path = "./setup/setup_accounts_and_block.rs"]
mod setup_accounts_and_block;
#[path = "./setup/setup_collection_whitelist.rs"]
mod setup_collection_whitelist;
#[path = "./setup/setup_contracts.rs"]
mod setup_contracts;
#[path = "./setup/setup_minter.rs"]
mod setup_minter;
#[path = "./setup/setup_signatures.rs"]
mod setup_signatures;

#[path = "./setup/mock_minter.rs"]
mod mock_minter;
#[path = "./setup/mock_whitelist.rs"]
mod mock_whitelist;

#[path = "./tests/collection_whitelist_helpers.rs"]
pub mod collection_whitelist_helpers;
#[cfg(test)]
#[path = "./tests/test_claim.rs"]
pub mod test_claim;
#[cfg(test)]
#[path = "./tests/test_collection_whitelist.rs"]
pub mod test_collection_whitelist;
#[cfg(test)]
#[path = "./tests/test_eth_whitelist.rs"]
pub mod test_eth_whitelist;