#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ConfigError(#[from] twelf::Error),

    #[error(transparent)]
    ProviderError(#[from] alloy::transports::TransportError),

    #[error(transparent)]
    TypeError(#[from] alloy::sol_types::Error),

    #[error("Block field {0} is invalid or empty")]
    BlockInfoError(String),

    #[error(transparent)]
    ContractAddressError(#[from] const_hex::FromHexError),

    #[error(transparent)]
    BlockTagError(#[from] alloy::eips::eip1898::ParseBlockNumberError),

    #[error(transparent)]
    DatabaseError(#[from] sea_orm::DbErr),

    #[error(transparent)]
    ConvertError(#[from] std::num::TryFromIntError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
