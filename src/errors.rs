use kube;
use std::convert::From;

impl From<std::io::Error> for KeyGenerationError {
    fn from(err: std::io::Error) -> Self {
        KeyGenerationError::IoError(err)
    }
}

impl From<kube::Error> for KeyGenerationError {
    fn from(err: kube::Error) -> Self {
        KeyGenerationError::KubeError(err)
    }
}

#[derive(Debug)]
pub enum KeyGenerationError {
    IoError(std::io::Error),
    KubeError(kube::Error),
    CommandError(std::process::Output),
}

impl From<KeyGenerationError> for Error {
    fn from(err: KeyGenerationError) -> Self {
        Error::KeyGenerationError(err)
    }
}

impl From<kube::Error> for Error {
    fn from(err: kube::Error) -> Self {
        Error::KubeError(err)
    }
}

#[derive(Debug)]
pub enum Error {
    KeyGenerationError(KeyGenerationError),
    KubeError(kube::Error),
}

