use cosmwasm_std::StdError;
use cw_utils::PaymentError;
use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid reply ID")]
    InvalidReplyID {},

    #[error("Not enough funds sent")]
    NotEnoughFunds {},

    #[error("TooManyCoins")]
    TooManyCoins {},

    #[error("IncorrectPaymentAmount")]
    IncorrectPaymentAmount {},

    #[error("Num tokens exceeds max token limit {max}")]
    MaxTokenLimitExceeded { max: u32 },

    #[error("Sold out")]
    SoldOut {},

    #[error("InvalidDenom {expected} got {got}")]
    InvalidDenom { expected: String, got: String },

    #[error("Minimum network mint price {expected} got {got}")]
    InsufficientMintPrice { expected: u128, got: u128 },

    #[error("Invalid address {addr}")]
    InvalidAddress { addr: String },

    #[error("Invalid token id")]
    InvalidTokenId {},

    #[error("Instantiate sg721 error")]
    InstantiateSg721Error {},

    #[error("Invalid base token URI (must be an IPFS URI)")]
    InvalidBaseTokenURI {},

    #[error("address not on whitelist: {addr}")]
    NotWhitelisted { addr: String },

    #[error("Minting has not started yet")]
    BeforeMintStartTime {},

    #[error("Invalid minting limit per address. max: {max}, got: {got}")]
    InvalidPerAddressLimit { max: String, got: String },

    #[error("Max minting limit per address exceeded")]
    MaxPerAddressLimitExceeded {},

    #[error("Invalid batch mint limit. max: {max}, got: {got}")]
    InvalidBatchMintLimit { max: String, got: String },

    #[error("Max batch mint limit exceeded")]
    MaxBatchMintLimitExceeded {},

    #[error("Token id: {token_id} already sold")]
    TokenIdAlreadySold { token_id: u64 },

    #[error("{0}")]
    Payment(#[from] PaymentError),
}

impl From<ParseError> for ContractError {
    fn from(_err: ParseError) -> ContractError {
        ContractError::InvalidBaseTokenURI {}
    }
}
