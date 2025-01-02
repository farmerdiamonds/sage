use thiserror::Error;

#[derive(Debug, Error)]
pub enum AntiCounterfeitError {
    #[error("No NFC device found")]
    NoNfcDevice,

    #[error("NFC device not supported")]
    NfcDeviceNotSupported,

}
