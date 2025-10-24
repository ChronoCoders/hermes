use thiserror::Error;

#[derive(Error, Debug)]
pub enum HermesError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    #[error("Decryption failed - wrong password?")]
    DecryptionFailed,

    #[error("SFTP connection failed: {0}")]
    SftpConnectionFailed(String),

    #[error("SFTP operation failed: {0}")]
    SftpOperationFailed(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Invalid password")]
    InvalidPassword,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),

    #[error("SSH error: {0}")]
    SshError(#[from] ssh2::Error),

    #[error("Key derivation failed")]
    KeyDerivationFailed,

    #[error("Invalid configuration file")]
    InvalidConfig,

    #[error("Remote path not specified")]
    RemotePathNotSpecified,
}

pub type Result<T> = std::result::Result<T, HermesError>;
