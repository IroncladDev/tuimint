use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    InvalidInviteCode,
    MissingModule,
    InvalidNotes,
    SpendError,
    DBLoadError,
    MnemonicError,
    Wallet(WalletError),
}

#[derive(Debug)]
pub enum WalletError {
    BuildError,
    JoinError,
    PreviewError,
    OpenError,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
