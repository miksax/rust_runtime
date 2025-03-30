use core::fmt::Debug;

pub enum Error {
    NoMoreData,
    BufferIsFull,
    UnknownSelector,
    DuplicateKey,
    DeadAddress,
    InsufficientTotalSupply,
    NoBalance,
    InsufficientBalance,
    MaxSupplyReached,
    CanNotTransferFromSelfAccount,
    CannotTransferZeroTokens,
    OnlyOwner,
    NoTokens,
    InsufficientAllowance,
    ConvertError,
    ParseError,
    NoValidAddress,

    Test,

    Revert(&'static str),
    Extra(&'static str),
}

impl Error {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NoMoreData => "No more data",
            Self::BufferIsFull => "Buffer is full",
            Self::UnknownSelector => "Unknown selector",
            Self::DuplicateKey => "Duplicate key",
            Self::DeadAddress => "Dead address",
            Self::InsufficientTotalSupply => "InsufficientTotalSupply",
            Self::NoBalance => "No balance",
            Self::InsufficientBalance => "Insufficient balance",
            Self::MaxSupplyReached => "Max supply reached",
            Self::CanNotTransferFromSelfAccount => "CanNotTransferFromSelfAccount",
            Self::CannotTransferZeroTokens => "CannotTransferZeroTokens",
            Self::OnlyOwner => "OnlyOwner",
            Self::NoTokens => "NoTokens",
            Self::InsufficientAllowance => "InsufficientAllowance",
            Self::ConvertError => "Convert error",
            Self::ParseError => "ParseError",
            Self::NoValidAddress => "NoValidAddress",

            Self::Test => "Test",

            Self::Revert(err) => err,
            Self::Extra(err) => err,
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(self.as_str()).finish()
    }
}
