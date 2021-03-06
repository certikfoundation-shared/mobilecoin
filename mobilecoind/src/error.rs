// Copyright (c) 2018-2020 MobileCoin Inc.

//! Errors generated by the mobilecoind system

use failure::Fail;
use lmdb::Error as LmdbError;
use mc_connection::Error as ConnectionError;
use mc_consensus_api::ConversionError;
use mc_crypto_keys::KeyError;
use mc_ledger_db::Error as LedgerDbError;
use mc_util_serial::{decode::Error as DecodeError, encode::Error as EncodeError};
use prost::DecodeError as ProstDecodeError;
use retry::Error as RetryError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failure with LMDB: {}", _0)]
    LMDB(LmdbError),

    #[fail(display = "Failure with LedgerDB: {}", _0)]
    LedgerDB(LedgerDbError),

    #[fail(display = "Failure with serialization: {}", _0)]
    Serialization(EncodeError),

    #[fail(display = "Failure with deserialization: {}", _0)]
    Deserialization(DecodeError),

    #[fail(display = "Prost (proto) decode error: {}", _0)]
    ProstDecode(ProstDecodeError),

    #[fail(display = "Connection error: {:?}", _0)]
    Connection(RetryError<ConnectionError>),

    #[fail(display = "IO error: {}", _0)]
    IO(std::io::Error),

    #[fail(display = "API conversion error: {}", _0)]
    ApiConversion(mc_consensus_api::ConversionError),

    #[fail(display = "Key error: {}", _0)]
    Key(KeyError),

    #[fail(display = "Channel send error")]
    ChannelSend,

    #[fail(display = "Invalid argument: {}: {}", _0, _1)]
    InvalidArgument(String, String),

    #[fail(display = "An entry in MonitorStore already exists for this MonitorId key")]
    MonitorIdExists,

    #[fail(display = "No matching key in MonitorStore was found")]
    MonitorIdNotFound,

    #[fail(display = "Failed to deserialize lmdb key bytes")]
    KeyDeserializationError,

    #[fail(display = "Subaddress SPK not found")]
    SubaddressSPKNotFound,

    #[fail(display = "An entry in SubaddressStore already exists for this index")]
    SubaddressSPKIdExists,

    #[fail(display = "Got transactions data but no key images - this should never happen")]
    MissingKeyImagesInLedgerDb,

    #[fail(display = "UnspentTxOut already in database")]
    DuplicateUnspentTxOut,

    #[fail(display = "No matching key was found in UtxoStore:subaddress_id_to_utxo_id")]
    SubaddressIdNotFound,

    #[fail(display = "No matching key was found in UtxoStore:utxo_id_to_utxo")]
    UtxoIdNotFound,

    #[fail(display = "Optimization not beneficial: {}", _0)]
    OptimizationNotBeneficial(String),

    #[fail(display = "Tx build error: {}", _0)]
    TxBuildError(String),

    #[fail(display = "Insufficient funds")]
    InsufficientFunds,

    #[fail(display = "Insufficient funds due to UTXO fragmentation")]
    InsufficientFundsFragmentedUtxos,

    #[fail(display = "The validator node we're tying to talk to was not found")]
    NodeNotFound,

    #[fail(display = "The ledger does not contain enough tx outs for rings")]
    InsufficientTxOuts,
}

impl From<RetryError<ConnectionError>> for Error {
    fn from(e: RetryError<ConnectionError>) -> Self {
        Error::Connection(e)
    }
}

impl From<LmdbError> for Error {
    fn from(e: LmdbError) -> Self {
        Error::LMDB(e)
    }
}

impl From<LedgerDbError> for Error {
    fn from(e: LedgerDbError) -> Self {
        Error::LedgerDB(e)
    }
}

impl From<EncodeError> for Error {
    fn from(e: EncodeError) -> Self {
        Error::Serialization(e)
    }
}

impl From<DecodeError> for Error {
    fn from(e: DecodeError) -> Self {
        Error::Deserialization(e)
    }
}

impl From<ProstDecodeError> for Error {
    fn from(e: ProstDecodeError) -> Self {
        Error::ProstDecode(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e)
    }
}

impl<M> From<crossbeam_channel::SendError<M>> for Error {
    fn from(_e: crossbeam_channel::SendError<M>) -> Self {
        Error::ChannelSend
    }
}

impl From<ConversionError> for Error {
    fn from(e: ConversionError) -> Self {
        Error::ApiConversion(e)
    }
}

impl From<KeyError> for Error {
    fn from(e: KeyError) -> Self {
        Error::Key(e)
    }
}
